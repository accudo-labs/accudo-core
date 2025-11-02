// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

use accudo_api::context::Context;
use accudo_config::config::NodeConfig;
use accudo_mempool::mocks::MockSharedMempool;
use accudo_storage_interface::mock::MockDbReaderWriter;
use accudo_types::chain_id::ChainId;
use std::sync::Arc;

// This is necessary for building the API with how the code is structured currently.
pub fn get_fake_context() -> Context {
    let mempool = MockSharedMempool::new_with_runtime();
    Context::new(
        ChainId::test(),
        Arc::new(MockDbReaderWriter),
        mempool.ac_client,
        NodeConfig::default(),
        None, /* table info reader */
    )
}
