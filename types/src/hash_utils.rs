// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

//! Helpers for selecting the default hash function used across core types.
//!
//! By centralizing the decision here we ensure every call site uses the
//! lattice-augmented digest, keeping the 32-byte layout while removing the
//! classical SHA3-only fallback.

use crate::hash_digest::HashDigest;
use accudo_crypto::hash::HashValue;

/// Computes the canonical digest used for account addresses, authentication
/// keys, and other protocol-critical identifiers.
#[inline]
pub fn canonical_hash(bytes: &[u8]) -> HashValue {
    HashValue::quantum_safe_of(bytes)
}

/// Computes the canonical digest alongside its legacy SHA3 companion to aid in
/// replay tooling and compatibility checks.
#[inline]
pub fn canonical_hash_digest(bytes: &[u8]) -> HashDigest {
    let (quantum_safe, legacy) = HashValue::quantum_safe_with_legacy(bytes);
    HashDigest::dual(quantum_safe, legacy)
}

#[cfg(test)]
mod tests {
    use super::canonical_hash;
    use accudo_crypto::hash::HashValue;

    #[test]
    fn canonical_hash_matches_quantum_safe_helper() {
        let data = b"phase2-hash-upgrade";
        assert_eq!(canonical_hash(data), HashValue::quantum_safe_of(data));
    }
}
