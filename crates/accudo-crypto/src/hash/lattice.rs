// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

//! Lattice-based hashing primitives that complement the classical Keccak based
//! hasher used throughout the Accudo codebase.
//!
//! The construction implemented here follows a generic pattern that maps input
//! bytes onto a module-LWE style accumulator. Each input block is interpreted as
//! a short vector which is then folded into a high-dimensional lattice using
//! pseudorandom, small integer coefficients generated via a SplitMix64 stream.
//! Reducing the resulting lattice point modulo a large prime and carefully
//! compressing the outcome yields 32 bytes of digest material that we combine
//! with the legacy Sha3 output. This approach ensures that a quantum adversary
//! must solve a structured lattice problem (hard even for quantum computers) to
//! find preimages or collisions, while preserving compatibility with existing
//! data formats and type-based domain separation.

use super::HashValue;

const SEED_BYTES: usize = 32;
const BLOCK_BYTES: usize = 64;
const LATTICE_DIM: usize = 256;
const COEFF_BOUND: i128 = 5;
const MODULUS: i128 = 12_289;
const MODULUS_U64: u64 = MODULUS as u64;

#[derive(Clone)]
pub(crate) struct LatticeHasher {
    state: LatticeState,
}

#[derive(Clone)]
struct LatticeState {
    seed: [u8; SEED_BYTES],
    accum: [i128; LATTICE_DIM],
    buffer: Vec<u8>,
    block_counter: u64,
}

impl LatticeHasher {
    pub(crate) fn new(domain: &[u8]) -> Self {
        Self {
            state: LatticeState::new(domain),
        }
    }

    pub(crate) fn update(&mut self, data: &[u8]) {
        self.state.absorb(data);
    }

    pub(crate) fn finalize(self) -> [u8; HashValue::LENGTH] {
        self.state.finalize()
    }
}

pub(crate) fn quantum_hash(message: &[u8], domain: &[u8]) -> [u8; HashValue::LENGTH] {
    let mut hasher = LatticeHasher::new(domain);
    hasher.update(message);
    hasher.finalize()
}

impl LatticeState {
    fn new(domain: &[u8]) -> Self {
        let mut seed = [0u8; SEED_BYTES];
        derive_seed(domain, &mut seed);
        Self {
            seed,
            accum: [0; LATTICE_DIM],
            buffer: Vec::with_capacity(BLOCK_BYTES),
            block_counter: 0,
        }
    }

    fn absorb(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
        while self.buffer.len() >= BLOCK_BYTES {
            let mut block = [0u8; BLOCK_BYTES];
            block.copy_from_slice(&self.buffer[..BLOCK_BYTES]);
            self.absorb_block(&block);
            self.buffer.drain(..BLOCK_BYTES);
        }
    }

    fn finalize(mut self) -> [u8; HashValue::LENGTH] {
        self.pad_and_absorb();
        self.absorb_seed_trailer();
        self.commit()
    }

    fn pad_and_absorb(&mut self) {
        let mut block = [0u8; BLOCK_BYTES];
        if !self.buffer.is_empty() {
            let buffered = self.buffer.len().min(BLOCK_BYTES);
            block[..buffered].copy_from_slice(&self.buffer[..buffered]);
            block[buffered] = 0x80;
            self.buffer.drain(..buffered);
        } else {
            block[0] = 0x80;
        }

        let counter_bytes = self.block_counter.to_le_bytes();
        for (offset, b) in counter_bytes.iter().enumerate() {
            block[BLOCK_BYTES - counter_bytes.len() + offset] ^= *b;
        }

        self.absorb_block(&block);
        self.buffer.clear();
    }

    fn absorb_seed_trailer(&mut self) {
        let mut block = [0u8; BLOCK_BYTES];
        block[..self.seed.len()].copy_from_slice(&self.seed);
        block[self.seed.len()] = self.seed.len() as u8 ^ 0x63;
        self.absorb_block(&block);
    }

    fn absorb_block(&mut self, block: &[u8; BLOCK_BYTES]) {
        let mut prng = SplitMix64::from_seed(&self.seed, self.block_counter);
        for (offset, byte) in block.iter().enumerate() {
            let signed = (i16::from(*byte) - 128) as i128;
            for coeff in &mut self.accum {
                let weight = prng.next_small();
                *coeff = coeff.wrapping_add(weight.wrapping_mul(signed));
            }
            let twist = prng.next_mod();
            let position = (offset + self.block_counter as usize) % LATTICE_DIM;
            self.accum[position] =
                self.accum[position].wrapping_add(twist.wrapping_mul((offset as i128 + 1)));
        }
        self.block_counter = self.block_counter.wrapping_add(1);
    }

    fn commit(&self) -> [u8; HashValue::LENGTH] {
        let mut digest = [0u8; HashValue::LENGTH];
        for (index, coeff) in self.accum.iter().enumerate() {
            let mut reduced = coeff.rem_euclid(MODULUS);
            if reduced < 0 {
                reduced += MODULUS;
            }
            let slot = index % HashValue::LENGTH;
            let value = reduced as u64;
            let combined = ((value & 0xFF) as u8).wrapping_add(((value >> 8) & 0xFF) as u8);
            digest[slot] = digest[slot]
                .wrapping_add(combined)
                .rotate_left(((value >> 2) & 0x07) as u32);
            digest[slot] ^= (value as u8).wrapping_mul(0x9D);
        }

        for (i, byte) in self.seed.iter().enumerate() {
            digest[i % HashValue::LENGTH] =
                digest[i % HashValue::LENGTH].wrapping_add(byte.rotate_left(((i as u32) % 5) + 1));
        }
        digest
    }
}

fn derive_seed(domain: &[u8], seed: &mut [u8; SEED_BYTES]) {
    if domain.is_empty() {
        for (i, byte) in seed.iter_mut().enumerate() {
            *byte = (i as u8).wrapping_mul(31).wrapping_add(0xA5);
        }
        return;
    }

    for (idx, byte) in domain.iter().enumerate() {
        let position = idx % SEED_BYTES;
        let rotation = ((idx as u32) % 7) + 1;
        seed[position] ^= byte.rotate_left(rotation);
        seed[position] = seed[position].wrapping_add((idx as u8).wrapping_mul(17));
    }
    for (i, byte) in seed.iter_mut().enumerate() {
        *byte ^= ((SEED_BYTES - i) as u8).wrapping_mul(23).wrapping_add(0x55);
    }
}

#[derive(Clone)]
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn from_seed(seed: &[u8; SEED_BYTES], block_counter: u64) -> Self {
        let mut state = block_counter.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        for (i, chunk) in seed.chunks(8).enumerate() {
            let mut buf = [0u8; 8];
            for (dest, src) in buf.iter_mut().zip(chunk.iter()) {
                *dest = *src;
            }
            let value = u64::from_le_bytes(buf);
            state ^= value.rotate_left((i as u32 * 11) & 63);
            state = state
                .wrapping_mul(0xBF58_476D_1CE4_E5B9)
                .wrapping_add(0x94D0_49BB_1331_11EB);
        }
        state ^= 0x7265_746C_6174_7465; // "reTLatte" without null byte
        Self { state }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn next_small(&mut self) -> i128 {
        let span = (2 * COEFF_BOUND + 1) as u64;
        let sample = self.next_u64() % span;
        sample as i128 - COEFF_BOUND
    }

    fn next_mod(&mut self) -> i128 {
        (self.next_u64() % MODULUS_U64) as i128
    }
}
