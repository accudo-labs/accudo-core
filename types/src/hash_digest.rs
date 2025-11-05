// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::hash_version::HashVersion;
use accudo_crypto::hash::HashValue;
use serde::{Deserialize, Serialize};

/// Dual digest representation persisted in ledger metadata.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, Hash)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(proptest_derive::Arbitrary))]
pub struct HashDigest {
    /// Canonical hash (PQ when available).
    hash: HashValue,
    /// Legacy SHA3 digest if retained for compatibility.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    legacy_hash: Option<HashValue>,
    /// Hash algorithm cohort.
    #[serde(default)]
    version: HashVersion,
}

impl HashDigest {
    pub const fn new(
        hash: HashValue,
        legacy_hash: Option<HashValue>,
        version: HashVersion,
    ) -> Self {
        Self {
            hash,
            legacy_hash,
            version,
        }
    }

    pub const fn post_quantum(hash: HashValue) -> Self {
        Self::new(hash, None, HashVersion::PostQuantum)
    }

    pub const fn legacy(hash: HashValue) -> Self {
        Self::new(hash, Some(hash), HashVersion::LegacySha3)
    }

    pub const fn dual(hash: HashValue, legacy_hash: HashValue) -> Self {
        Self::new(hash, Some(legacy_hash), HashVersion::Dual)
    }

    pub const fn hash(&self) -> HashValue {
        self.hash
    }

    pub const fn legacy_hash(&self) -> Option<HashValue> {
        self.legacy_hash
    }

    pub const fn version(&self) -> HashVersion {
        self.version
    }
}

impl From<HashValue> for HashDigest {
    fn from(value: HashValue) -> Self {
        HashDigest::post_quantum(value)
    }
}
