// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

use accudo_indexer_grpc_data_service::IndexerGrpcDataServiceConfig;
use accudo_indexer_grpc_server_framework::ServerArgs;
use anyhow::Result;
use clap::Parser;

#[cfg(unix)]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> Result<()> {
    let args = ServerArgs::parse();
    args.run::<IndexerGrpcDataServiceConfig>().await
}
