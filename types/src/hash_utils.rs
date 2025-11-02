// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

//! Helpers for selecting the default hash function used across core types.
//!
//! By centralizing the decision here we ensure every call site uses the
//! lattice-augmented digest, keeping the 32-byte layout while removing the
//! classical SHA3-only fallback.

use accudo_crypto::hash::HashValue;

/// Computes the canonical digest used for account addresses, authentication
/// keys, and other protocol-critical identifiers.
#[inline]
pub fn canonical_hash(bytes: &[u8]) -> HashValue {
    HashValue::quantum_safe_of(bytes)
}
