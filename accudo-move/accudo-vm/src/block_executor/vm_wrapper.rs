// Copyright © Accudo Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{accudo_vm::AccudoVM, block_executor::AccudoTransactionOutput};
use accudo_block_executor::task::{ExecutionStatus, ExecutorTask};
use accudo_logger::{enabled, Level};
use accudo_mvhashmap::types::TxnIndex;
use accudo_types::{
    state_store::{StateView, StateViewId},
    transaction::{
        signature_verified_transaction::SignatureVerifiedTransaction, AuxiliaryInfo, Transaction,
        WriteSetPayload,
    },
};
use accudo_vm_environment::environment::AccudoEnvironment;
use accudo_vm_logging::{log_schema::AdapterLogSchema, prelude::*};
use accudo_vm_types::{
    module_and_script_storage::code_storage::AccudoCodeStorage,
    resolver::{BlockSynchronizationKillSwitch, ExecutorView, ResourceGroupView},
};
use fail::fail_point;
use move_core_types::vm_status::{StatusCode, VMStatus};

pub struct AccudoExecutorTask {
    vm: AccudoVM,
    id: StateViewId,
}

impl ExecutorTask for AccudoExecutorTask {
    type AuxiliaryInfo = AuxiliaryInfo;
    type Error = VMStatus;
    type Output = AccudoTransactionOutput;
    type Txn = SignatureVerifiedTransaction;

    fn init(environment: &AccudoEnvironment, state_view: &impl StateView) -> Self {
        let vm = AccudoVM::new(environment, state_view);
        let id = state_view.id();
        Self { vm, id }
    }

    // This function is called by the BlockExecutor for each transaction it intends
    // to execute (via the ExecutorTask trait). It can be as a part of sequential
    // execution, or speculatively as a part of a parallel execution.
    fn execute_transaction(
        &self,
        view: &(impl ExecutorView
              + ResourceGroupView
              + AccudoCodeStorage
              + BlockSynchronizationKillSwitch),
        txn: &SignatureVerifiedTransaction,
        auxiliary_info: &Self::AuxiliaryInfo,
        txn_idx: TxnIndex,
    ) -> ExecutionStatus<AccudoTransactionOutput, VMStatus> {
        fail_point!("accudo_vm::vm_wrapper::execute_transaction", |_| {
            ExecutionStatus::DelayedFieldsCodeInvariantError("fail points error".into())
        });

        let log_context = AdapterLogSchema::new(self.id, txn_idx as usize);
        let resolver = self.vm.as_move_resolver_with_group_view(view);
        match self
            .vm
            .execute_single_transaction(txn, &resolver, view, &log_context, auxiliary_info)
        {
            Ok((vm_status, vm_output)) => {
                if vm_output.status().is_discarded() {
                    speculative_trace!(
                        &log_context,
                        format!("Transaction discarded, status: {:?}", vm_status),
                    );
                }
                if vm_status.status_code() == StatusCode::SPECULATIVE_EXECUTION_ABORT_ERROR {
                    ExecutionStatus::SpeculativeExecutionAbortError(
                        vm_status.message().cloned().unwrap_or_default(),
                    )
                } else if vm_status.status_code()
                    == StatusCode::DELAYED_FIELD_OR_BLOCKSTM_CODE_INVARIANT_ERROR
                {
                    ExecutionStatus::DelayedFieldsCodeInvariantError(
                        vm_status.message().cloned().unwrap_or_default(),
                    )
                } else if AccudoVM::should_restart_execution(vm_output.events()) {
                    speculative_info!(
                        &log_context,
                        "Reconfiguration occurred: restart required".into()
                    );
                    ExecutionStatus::SkipRest(AccudoTransactionOutput::new(vm_output))
                } else {
                    assert!(
                        Self::is_transaction_dynamic_change_set_capable(txn),
                        "DirectWriteSet should always create SkipRest transaction, validate_waypoint_change_set provides this guarantee"
                    );
                    ExecutionStatus::Success(AccudoTransactionOutput::new(vm_output))
                }
            },
            // execute_single_transaction only returns an error when transactions that should never fail
            // (BlockMetadataTransaction and GenesisTransaction) return an error themselves.
            Err(err) => {
                if err.status_code() == StatusCode::SPECULATIVE_EXECUTION_ABORT_ERROR {
                    ExecutionStatus::SpeculativeExecutionAbortError(
                        err.message().cloned().unwrap_or_default(),
                    )
                } else if err.status_code()
                    == StatusCode::DELAYED_FIELD_OR_BLOCKSTM_CODE_INVARIANT_ERROR
                {
                    ExecutionStatus::DelayedFieldsCodeInvariantError(
                        err.message().cloned().unwrap_or_default(),
                    )
                } else {
                    ExecutionStatus::Abort(err)
                }
            },
        }
    }

    fn is_transaction_dynamic_change_set_capable(txn: &Self::Txn) -> bool {
        if txn.is_valid() {
            if let Transaction::GenesisTransaction(WriteSetPayload::Direct(_)) = txn.expect_valid()
            {
                // WriteSetPayload::Direct cannot be handled in mode where delayed_field_optimization or
                // resource_groups_split_in_change_set is enabled.
                return false;
            }
        }
        true
    }
}
