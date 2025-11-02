// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::on_chain_config::OnChainConfig;
use serde::{Deserialize, Serialize};

/// Defines the version of Accudo Validator software.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct AccudoVersion {
    pub major: u64,
}

impl OnChainConfig for AccudoVersion {
    const MODULE_IDENTIFIER: &'static str = "version";
    const TYPE_IDENTIFIER: &'static str = "Version";
}

// NOTE: version number for release 1.2 Accudo
// Items gated by this version number include:
//  - the EntryFunction payload type
pub const ACCUDO_VERSION_2: AccudoVersion = AccudoVersion { major: 2 };

// NOTE: version number for release 1.3 of Accudo
// Items gated by this version number include:
//  - Multi-agent transactions
pub const ACCUDO_VERSION_3: AccudoVersion = AccudoVersion { major: 3 };

// NOTE: version number for release 1.4 of Accudo
// Items gated by this version number include:
//  - Conflict-Resistant Sequence Numbers
pub const ACCUDO_VERSION_4: AccudoVersion = AccudoVersion { major: 4 };

// Maximum current known version
pub const ACCUDO_MAX_KNOWN_VERSION: AccudoVersion = ACCUDO_VERSION_4;
