// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::module_and_script_storage::module_storage::AccudoModuleStorage;
use move_binary_format::file_format::CompiledScript;
use move_vm_runtime::Script;
use move_vm_types::code::ScriptCache;

/// Represents code storage used by the Accudo blockchain, capable of caching scripts and modules.
pub trait AccudoCodeStorage:
    AccudoModuleStorage + ScriptCache<Key = [u8; 32], Deserialized = CompiledScript, Verified = Script>
{
}

impl<T> AccudoCodeStorage for T where
    T: AccudoModuleStorage
        + ScriptCache<Key = [u8; 32], Deserialized = CompiledScript, Verified = Script>
{
}
