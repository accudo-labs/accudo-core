//! Post-quantum abstraction layer for cryptographic primitives.
//!
//! The goal of this module is to let the wider codebase ask for signing,
//! hashing, or key agreement functionality without baking in assumptions
//! about the underlying algorithm (e.g. Ed25519 vs. Dilithium).  Concrete
//! implementations can register themselves against a [`SchemeId`] so higher
//! level code can negotiate capabilities or send dual-signature payloads
//! during the migration to post-quantum primitives.

use crate::ed25519::{
    Ed25519PublicKey, Ed25519Signature, ED25519_PUBLIC_KEY_LENGTH, ED25519_SIGNATURE_LENGTH,
};
use anyhow::{anyhow, bail, Context};
use pqcrypto_dilithium::dilithium3;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt, sync::Arc};

/// Canonical identifiers for classical and post-quantum schemes.
///
/// These values are intended for serialization and on-the-wire negotiation, so
/// only append new entries; do not reorder existing variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum SchemeId {
    // Classical primitives (legacy support)
    Ed25519 = 0x0101,
    MultiEd25519 = 0x0102,
    Secp256k1Ecdsa = 0x0103,
    Secp256r1Ecdsa = 0x0104,
    Bls12381 = 0x0201,
    X25519 = 0x0301,

    // Post-quantum candidates
    Dilithium2 = 0x1101,
    Dilithium3 = 0x1102,
    Dilithium5 = 0x1103,
    Falcon512 = 0x1110,

    Kyber768 = 0x2101,
    Kyber1024 = 0x2102,

    ShakeLattice = 0x3101,
}

impl SchemeId {
    /// Returns a stable string label for logging or telemetry.
    pub fn label(self) -> &'static str {
        match self {
            SchemeId::Ed25519 => "ed25519",
            SchemeId::MultiEd25519 => "multi_ed25519",
            SchemeId::Secp256k1Ecdsa => "secp256k1_ecdsa",
            SchemeId::Secp256r1Ecdsa => "secp256r1_ecdsa",
            SchemeId::Bls12381 => "bls12381",
            SchemeId::X25519 => "x25519",
            SchemeId::Dilithium2 => "dilithium2",
            SchemeId::Dilithium3 => "dilithium3",
            SchemeId::Dilithium5 => "dilithium5",
            SchemeId::Falcon512 => "falcon512",
            SchemeId::Kyber768 => "kyber768",
            SchemeId::Kyber1024 => "kyber1024",
            SchemeId::ShakeLattice => "shake_lattice",
        }
    }
}

impl fmt::Display for SchemeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// Unified representation for signature payloads.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct SignatureBundle {
    /// Which algorithm generated the signature bytes.
    pub scheme: SchemeId,
    /// Serialized signature payload.
    #[serde(with = "serde_bytes")]
    pub signature: Vec<u8>,
}

impl SignatureBundle {
    /// Creates a new bundle by copying signature bytes.
    pub fn new<S: Into<Vec<u8>>>(scheme: SchemeId, signature: S) -> Self {
        SignatureBundle {
            scheme,
            signature: signature.into(),
        }
    }

    /// Access the signature bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.signature
    }
}

/// Common interface for signature algorithms.
pub trait SignatureAlgorithm: Send + Sync {
    /// Returns the identifier advertised by this algorithm.
    fn scheme(&self) -> SchemeId;

    /// Returns the expected public key length in bytes, if fixed.
    fn public_key_length(&self) -> Option<usize> {
        None
    }

    /// Returns the expected signature length in bytes, if fixed.
    fn signature_length(&self) -> Option<usize> {
        None
    }

    /// Verifies a signature against a given public key and message.
    fn verify(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> anyhow::Result<()>;
}

/// Interface for key encapsulation / key agreement algorithms.
pub trait KeyExchangeAlgorithm: Send + Sync {
    /// Algorithm identifier.
    fn scheme(&self) -> SchemeId;

    /// Length of the public key in bytes, if fixed.
    fn public_key_length(&self) -> Option<usize> {
        None
    }

    /// Length of the shared secret in bytes, if fixed.
    fn shared_secret_length(&self) -> Option<usize> {
        None
    }

    /// Performs a static-static key agreement (e.g., Diffie-Hellman).
    fn derive_shared_secret(
        &self,
        private_key: &[u8],
        peer_public_key: &[u8],
    ) -> anyhow::Result<Vec<u8>>;
}

/// Interface for hashing backends (streaming API).
pub trait HashAlgorithm: Send + Sync {
    /// Identifier describing the hash family.
    fn scheme(&self) -> SchemeId;

    /// Digest length in bytes.
    fn digest_length(&self) -> usize;

    /// Returns a boxed hasher that implements `std::io::Write` and `finish`.
    fn create(&self) -> Box<dyn HashState>;
}

/// Streaming hash helper.
pub trait HashState: std::io::Write + Send {
    /// Finalizes the hash computation and returns the digest.
    fn finish(self: Box<Self>) -> Vec<u8>;
}

/// Minimal registry backing to look up algorithms by identifier.
#[derive(Default)]
pub struct AlgorithmRegistry {
    signatures: BTreeMap<SchemeId, Arc<dyn SignatureAlgorithm>>,
    key_exchanges: BTreeMap<SchemeId, Arc<dyn KeyExchangeAlgorithm>>,
    hashes: BTreeMap<SchemeId, Arc<dyn HashAlgorithm>>,
}

impl AlgorithmRegistry {
    /// Registers a signature algorithm.
    pub fn register_signature<A>(&mut self, algorithm: A)
    where
        A: SignatureAlgorithm + 'static,
    {
        self.signatures
            .insert(algorithm.scheme(), Arc::new(algorithm));
    }

    /// Registers a key exchange algorithm.
    pub fn register_key_exchange<A>(&mut self, algorithm: A)
    where
        A: KeyExchangeAlgorithm + 'static,
    {
        self.key_exchanges
            .insert(algorithm.scheme(), Arc::new(algorithm));
    }

    /// Registers a hash algorithm.
    pub fn register_hash<A>(&mut self, algorithm: A)
    where
        A: HashAlgorithm + 'static,
    {
        self.hashes.insert(algorithm.scheme(), Arc::new(algorithm));
    }

    /// Fetches a signature implementation by identifier.
    pub fn signature(&self, scheme: SchemeId) -> Option<Arc<dyn SignatureAlgorithm>> {
        self.signatures.get(&scheme).cloned()
    }

    /// Fetches a key exchange implementation by identifier.
    pub fn key_exchange(&self, scheme: SchemeId) -> Option<Arc<dyn KeyExchangeAlgorithm>> {
        self.key_exchanges.get(&scheme).cloned()
    }

    /// Fetches a hash implementation by identifier.
    pub fn hash(&self, scheme: SchemeId) -> Option<Arc<dyn HashAlgorithm>> {
        self.hashes.get(&scheme).cloned()
    }
}

/// Adapter exposing the existing Ed25519 verifier through the unified trait.
pub struct Ed25519Verifier;

impl SignatureAlgorithm for Ed25519Verifier {
    fn scheme(&self) -> SchemeId {
        SchemeId::Ed25519
    }

    fn public_key_length(&self) -> Option<usize> {
        Some(ED25519_PUBLIC_KEY_LENGTH)
    }

    fn signature_length(&self) -> Option<usize> {
        Some(ED25519_SIGNATURE_LENGTH)
    }

    fn verify(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> anyhow::Result<()> {
        let pubkey = Ed25519PublicKey::try_from(public_key)
            .map_err(|e| anyhow!("failed to parse ed25519 public key: {e:?}"))?;
        let signature = Ed25519Signature::try_from(signature)
            .map_err(|e| anyhow!("failed to parse ed25519 signature: {e:?}"))?;
        signature
            .verify_arbitrary_msg(message, &pubkey)
            .context("ed25519 signature verification failed")
    }
}

/// Adapter exposing Dilithium 3 verification through the unified trait.
pub struct Dilithium3Verifier;

impl SignatureAlgorithm for Dilithium3Verifier {
    fn scheme(&self) -> SchemeId {
        SchemeId::Dilithium3
    }

    fn public_key_length(&self) -> Option<usize> {
        Some(dilithium3::PUBLIC_KEY_BYTES)
    }

    fn signature_length(&self) -> Option<usize> {
        Some(dilithium3::SIGNATURE_BYTES)
    }

    fn verify(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> anyhow::Result<()> {
        if public_key.len() != dilithium3::PUBLIC_KEY_BYTES {
            bail!(
                "unexpected dilithium public key size: expected {}, got {}",
                dilithium3::PUBLIC_KEY_BYTES,
                public_key.len()
            );
        }
        if signature.len() != dilithium3::SIGNATURE_BYTES {
            bail!(
                "unexpected dilithium signature size: expected {}, got {}",
                dilithium3::SIGNATURE_BYTES,
                signature.len()
            );
        }

        let pk = dilithium3::PublicKey::from_bytes(public_key)
            .map_err(|e| anyhow!("failed to parse dilithium public key: {e:?}"))?;
        let sig = dilithium3::DetachedSignature::from_bytes(signature)
            .map_err(|e| anyhow!("failed to parse dilithium signature: {e:?}"))?;

        dilithium3::verify_detached_signature(&sig, message, &pk)
            .map_err(|e| anyhow!("dilithium verification failed: {e:?}"))
    }
}

#[derive(Clone)]
/// Helper representing an in-memory Dilithium keypair.
pub struct Dilithium3KeyPair {
    public_key: Vec<u8>,
    secret_key: Arc<[u8]>,
}

impl Dilithium3KeyPair {
    /// Generates a fresh Dilithium3 keypair.
    pub fn generate() -> anyhow::Result<Self> {
        let (public, secret) = dilithium3::keypair();
        Ok(Self {
            public_key: public.as_bytes().to_vec(),
            secret_key: Arc::from(secret.as_bytes().to_vec()),
        })
    }

    /// Returns the raw public key bytes.
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    /// Returns the serialized secret key bytes.
    pub fn secret_key(&self) -> Arc<[u8]> {
        Arc::clone(&self.secret_key)
    }

    /// Signs a message, returning a `SignatureBundle` tagged as Dilithium3.
    ///
    /// Callers are responsible for providing the exact message bytes that should be
    /// authenticated (e.g. the BCS-serialized signing message generated via
    /// `accudo_crypto::traits::signing_message`).
    pub fn sign(&self, message: &[u8]) -> anyhow::Result<SignatureBundle> {
        let secret = dilithium3::SecretKey::from_bytes(self.secret_key.as_ref())
            .map_err(|e| anyhow!("failed to recover dilithium secret key: {e:?}"))?;
        let signature = dilithium3::detached_sign(message, &secret);
        Ok(SignatureBundle::new(
            SchemeId::Dilithium3,
            signature.as_bytes().to_vec(),
        ))
    }
}

/// Returns an [`AlgorithmRegistry`] populated with the currently supported algorithms.
pub fn baseline_registry() -> AlgorithmRegistry {
    let mut registry = AlgorithmRegistry::default();
    registry.register_signature(Ed25519Verifier);
    registry.register_signature(Dilithium3Verifier);
    registry
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ed25519::{Ed25519PrivateKey, Ed25519PublicKey},
        traits::{SigningKey, Uniform},
    };
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn ed25519_adapter_verifies_signatures() {
        let mut rng = StdRng::seed_from_u64(42);
        let private = Ed25519PrivateKey::generate(&mut rng);
        let public: Ed25519PublicKey = (&private).into();
        let message = b"adapter-check";
        let signature = private.sign_arbitrary_message(message).to_bytes();

        let registry = baseline_registry();
        let adapter = registry
            .signature(SchemeId::Ed25519)
            .expect("ed25519 adapter registered");

        adapter
            .verify(&public.to_bytes(), message, &signature)
            .expect("signature should verify");
    }

    #[test]
    fn dilithium_adapter_roundtrip() {
        let message = b"dilithium-adapter";
        let keypair = Dilithium3KeyPair::generate().expect("dilithium keypair generation");
        let signature = keypair
            .sign(message)
            .expect("dilithium signing should succeed");

        let registry = baseline_registry();
        let adapter = registry
            .signature(SchemeId::Dilithium3)
            .expect("dilithium adapter registered");

        adapter
            .verify(keypair.public_key(), message, signature.bytes())
            .expect("dilithium signature should verify");
    }
}
