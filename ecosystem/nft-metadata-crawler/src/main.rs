// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

use accudo_indexer_grpc_server_framework::ServerArgs;
use accudo_nft_metadata_crawler::config::NFTMetadataCrawlerConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = <ServerArgs as clap::Parser>::parse();
    args.run::<NFTMetadataCrawlerConfig>().await
}
