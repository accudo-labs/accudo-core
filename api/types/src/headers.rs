// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

/// Chain ID of the current chain
pub const X_ACCUDO_CHAIN_ID: &str = "X-Accudo-Chain-Id";
/// Current epoch of the chain
pub const X_ACCUDO_EPOCH: &str = "X-Accudo-Epoch";
/// Current ledger version of the chain
pub const X_ACCUDO_LEDGER_VERSION: &str = "X-Accudo-Ledger-Version";
/// Oldest non-pruned ledger version of the chain
pub const X_ACCUDO_LEDGER_OLDEST_VERSION: &str = "X-Accudo-Ledger-Oldest-Version";
/// Current block height of the chain
pub const X_ACCUDO_BLOCK_HEIGHT: &str = "X-Accudo-Block-Height";
/// Oldest non-pruned block height of the chain
pub const X_ACCUDO_OLDEST_BLOCK_HEIGHT: &str = "X-Accudo-Oldest-Block-Height";
/// Current timestamp of the chain
pub const X_ACCUDO_LEDGER_TIMESTAMP: &str = "X-Accudo-Ledger-TimestampUsec";
/// Cursor used for pagination.
pub const X_ACCUDO_CURSOR: &str = "X-Accudo-Cursor";
/// The cost of the call in terms of gas. Only applicable to calls that result in
/// function execution in the VM, e.g. view functions, txn simulation.
pub const X_ACCUDO_GAS_USED: &str = "X-Accudo-Gas-Used";
/// Provided by the client to identify what client it is.
pub const X_ACCUDO_CLIENT: &str = "x-accudo-client";
