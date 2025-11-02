// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

use super::*;
use accudo_schemadb::{schema::fuzzing::assert_encode_decode, test_no_panic_decoding};
use accudo_types::transaction::{PersistedAuxiliaryInfo, Version};
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_encode_decode(version in any::<Version>(), info in any::<PersistedAuxiliaryInfo>()) {
        assert_encode_decode::<PersistedAuxiliaryInfoSchema>(&version, &info);
    }
}

test_no_panic_decoding!(PersistedAuxiliaryInfoSchema);
