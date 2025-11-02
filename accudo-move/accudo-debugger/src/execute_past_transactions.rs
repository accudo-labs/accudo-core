// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{accudo_debugger::AccudoDebugger, common::Opts};
use accudo_rest_client::Client;
use anyhow::Result;
use clap::Parser;
use url::Url;

#[derive(Parser)]
pub struct Command {
    #[clap(flatten)]
    opts: Opts,

    #[clap(long)]
    begin_version: u64,

    #[clap(long)]
    limit: u64,

    #[clap(long)]
    skip_result: bool,

    #[clap(long)]
    repeat_execution_times: Option<u64>,

    #[clap(long)]
    use_same_block_boundaries: bool,
}

impl Command {
    pub async fn run(self) -> Result<()> {
        let debugger = if let Some(rest_endpoint) = self.opts.target.rest_endpoint {
            AccudoDebugger::rest_client(Client::new(Url::parse(&rest_endpoint)?))?
        } else if let Some(db_path) = self.opts.target.db_path {
            AccudoDebugger::db(db_path)?
        } else {
            unreachable!("Must provide one target.");
        };

        let result = debugger
            .execute_past_transactions(
                self.begin_version,
                self.limit,
                self.use_same_block_boundaries,
                self.repeat_execution_times.unwrap_or(1),
                &self.opts.concurrency_level,
            )
            .await?;

        if !self.skip_result {
            println!("{result:#?}",);
        }

        Ok(())
    }
}
