// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    move_vm_ext::{AccudoMoveResolver, SessionId},
    AccudoVM,
};
use accudo_types::validator_txn::ValidatorTransaction;
use accudo_vm_logging::log_schema::AdapterLogSchema;
use accudo_vm_types::{
    module_and_script_storage::module_storage::AccudoModuleStorage, output::VMOutput,
};
use move_core_types::vm_status::VMStatus;

impl AccudoVM {
    pub(crate) fn process_validator_transaction(
        &self,
        resolver: &impl AccudoMoveResolver,
        module_storage: &impl AccudoModuleStorage,
        txn: ValidatorTransaction,
        log_context: &AdapterLogSchema,
    ) -> Result<(VMStatus, VMOutput), VMStatus> {
        let session_id = SessionId::validator_txn(&txn);
        match txn {
            ValidatorTransaction::DKGResult(dkg_node) => {
                self.process_dkg_result(resolver, module_storage, log_context, session_id, dkg_node)
            },
            ValidatorTransaction::ObservedJWKUpdate(jwk_update) => self.process_jwk_update(
                resolver,
                module_storage,
                log_context,
                session_id,
                jwk_update,
            ),
        }
    }
}

mod dkg;
mod jwk;
