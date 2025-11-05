// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Identifies which hash algorithm cohort was used to derive digest fields.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd, Serialize, Hash)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(proptest_derive::Arbitrary))]
pub enum HashVersion {
    /// Legacy SHA3-only digests.
    LegacySha3 = 0,
    /// Hybrid mode storing both SHA3 and PQ digests.
    Dual = 1,
    /// PQ-only digests.
    PostQuantum = 2,
}

impl HashVersion {
    pub const fn is_legacy(self) -> bool {
        matches!(self, HashVersion::LegacySha3)
    }

    pub const fn is_dual(self) -> bool {
        matches!(self, HashVersion::Dual)
    }

    pub const fn is_post_quantum(self) -> bool {
        matches!(self, HashVersion::PostQuantum)
    }
}

impl Default for HashVersion {
    fn default() -> Self {
        HashVersion::PostQuantum
    }
}

impl Display for HashVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            HashVersion::LegacySha3 => "legacy",
            HashVersion::Dual => "dual",
            HashVersion::PostQuantum => "post_quantum",
        };
        write!(f, "{label}")
    }
}
