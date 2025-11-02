// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

use accudo_infallible::Mutex;
use accudo_logger::info;
use accudo_storage_interface::state_store::state_view::cached_state_view::CachedStateView;
use accudo_vm::{
    sharded_block_executor::{local_executor_shard::LocalExecutorClient, ShardedBlockExecutor},
    AccudoVM,
};
use once_cell::sync::Lazy;
use std::sync::Arc;

pub static SHARDED_BLOCK_EXECUTOR: Lazy<
    Arc<Mutex<ShardedBlockExecutor<CachedStateView, LocalExecutorClient<CachedStateView>>>>,
> = Lazy::new(|| {
    info!("LOCAL_SHARDED_BLOCK_EXECUTOR created");
    Arc::new(Mutex::new(
        LocalExecutorClient::create_local_sharded_block_executor(AccudoVM::get_num_shards(), None),
    ))
});
