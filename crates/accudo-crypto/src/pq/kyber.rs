// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

//! Thin wrappers around the pqcrypto Kyber768 implementation.

use crate::traits::{self, CryptoMaterialError, PrivateKey, PublicKey, ValidCryptoMaterial};
use accudo_crypto_derive::{DeserializeKey, SerializeKey, SilentDebug, SilentDisplay};
use anyhow::{anyhow, Result};
use pqcrypto_kyber::kyber768;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

/// Length in bytes of a Kyber768 public key.
pub const KYBER_PUBLIC_KEY_LENGTH: usize = kyber768::public_key_bytes();
/// Length in bytes of a Kyber768 secret key.
pub const KYBER_PRIVATE_KEY_LENGTH: usize = kyber768::secret_key_bytes();
/// Length in bytes of a Kyber768 ciphertext.
pub const KYBER_CIPHERTEXT_LENGTH: usize = kyber768::ciphertext_bytes();
/// Length in bytes of a Kyber768 shared secret.
pub const KYBER_SHARED_SECRET_LENGTH: usize = kyber768::shared_secret_bytes();

/// Wrapper for Kyber768 private keys.
#[derive(DeserializeKey, SerializeKey, SilentDisplay, SilentDebug)]
pub struct KyberPrivateKey(#[serde(with = "serde_bytes")] Vec<u8>);

/// Wrapper for Kyber768 public keys.
#[derive(
    Clone, PartialEq, Eq, Hash, SerializeKey, DeserializeKey, Serialize, Deserialize, SilentDebug,
)]
pub struct KyberPublicKey(#[serde(with = "serde_bytes")] Vec<u8>);

/// Wrapper for Kyber768 ciphertexts produced during encapsulation.
#[derive(Clone)]
pub struct KyberCiphertext(pub(crate) Vec<u8>);

impl KyberPrivateKey {
    /// Generates a fresh Kyber keypair.
    pub fn generate() -> Result<KyberKeyPair> {
        let (public, secret) = kyber768::keypair();
        Ok(KyberKeyPair {
            private: KyberPrivateKey(secret.as_bytes().to_vec()),
            public: KyberPublicKey(public.as_bytes().to_vec()),
        })
    }

    /// Decapsulates a Kyber ciphertext, returning the shared secret bytes.
    pub fn decapsulate(&self, ciphertext: &KyberCiphertext) -> Result<Vec<u8>> {
        let secret = kyber768::SecretKey::from_bytes(&self.0)
            .map_err(|_| anyhow!("invalid Kyber private key bytes"))?;
        let ct = kyber768::Ciphertext::from_bytes(&ciphertext.0)
            .map_err(|_| anyhow!("invalid Kyber ciphertext"))?;
        let shared = kyber768::decapsulate(&ct, &secret)
            .map_err(|_| anyhow!("Kyber decapsulation failed"))?;
        Ok(shared.as_bytes().to_vec())
    }
}

impl Clone for KyberPrivateKey {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl KyberPublicKey {
    /// Returns the underlying byte representation.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Encapsulates to the public key, returning `(ciphertext, shared_secret)`.
    pub fn encapsulate(&self) -> Result<(KyberCiphertext, Vec<u8>)> {
        let pk = kyber768::PublicKey::from_bytes(&self.0)
            .map_err(|_| anyhow!("invalid Kyber public key bytes"))?;
        let (ct, ss) = kyber768::encapsulate(&pk);
        Ok((
            KyberCiphertext(ct.as_bytes().to_vec()),
            ss.as_bytes().to_vec(),
        ))
    }
}

/// Tuple struct representing a Kyber keypair.
#[derive(Clone)]
pub struct KyberKeyPair {
    /// Private component of the Kyber keypair.
    pub private: KyberPrivateKey,
    /// Public component of the Kyber keypair.
    pub public: KyberPublicKey,
}

impl TryFrom<&[u8]> for KyberPrivateKey {
    type Error = CryptoMaterialError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != KYBER_PRIVATE_KEY_LENGTH {
            return Err(CryptoMaterialError::DeserializationError);
        }
        Ok(Self(value.to_vec()))
    }
}

impl TryFrom<&[u8]> for KyberPublicKey {
    type Error = CryptoMaterialError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != KYBER_PUBLIC_KEY_LENGTH {
            return Err(CryptoMaterialError::DeserializationError);
        }
        Ok(Self(value.to_vec()))
    }
}

impl From<KyberCiphertext> for Vec<u8> {
    fn from(value: KyberCiphertext) -> Self {
        value.0
    }
}

impl ValidCryptoMaterial for KyberPrivateKey {
    const AIP_80_PREFIX: &'static str = "kyber-priv-";

    fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl ValidCryptoMaterial for KyberPublicKey {
    const AIP_80_PREFIX: &'static str = "kyber-pub-";

    fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl PrivateKey for KyberPrivateKey {
    type PublicKeyMaterial = KyberPublicKey;
}

impl PublicKey for KyberPublicKey {
    type PrivateKeyMaterial = KyberPrivateKey;
}

impl KyberCiphertext {
    /// Returns the ciphertext as raw bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Constructs a ciphertext from raw bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != KYBER_CIPHERTEXT_LENGTH {
            return Err(anyhow!(
                "invalid Kyber ciphertext length: expected {} bytes, got {}",
                KYBER_CIPHERTEXT_LENGTH,
                bytes.len()
            ));
        }
        Ok(Self(bytes.to_vec()))
    }
}

impl TryInto<Vec<u8>> for KyberCiphertext {
    type Error = CryptoMaterialError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Ok(self.0)
    }
}
