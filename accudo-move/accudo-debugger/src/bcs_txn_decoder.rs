// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::accudo_debugger::AccudoDebugger;
use accudo_rest_client::Client;
use accudo_types::transaction::SignedTransaction;
use anyhow::Result;
use clap::Parser;
use regex::Regex;
use std::io;
use url::Url;

#[derive(Parser)]
pub struct Command {
    #[clap(long, default_value_t = false)]
    execute: bool,

    #[clap(long, default_value_t = 1)]
    concurrency_level: usize,
}

impl Command {
    pub async fn run(self) -> Result<()> {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let re = Regex::new(r"\d+").unwrap();
        let bytes = re
            .find_iter(&buffer)
            .filter_map(|m| m.as_str().parse::<u8>().ok())
            .collect::<Vec<u8>>();

        let txn: SignedTransaction = bcs::from_bytes::<SignedTransaction>(&bytes)?;
        let chain_id = txn.chain_id();
        println!("===================");
        println!("Transaction Summary");
        println!("===================");
        println!("Sender: {:?}", txn.sender());
        println!("Sequence number: {:?}", txn.sequence_number());

        let network = if chain_id.is_mainnet() {
            "mainnet".to_string()
        } else if chain_id.is_testnet() {
            "testnet".to_string()
        } else {
            "devnet".to_string()
        };
        println!("Chain ID: {}", chain_id.id());
        println!("Network: {}", network);

        let endpoint = format!("https://{}.accudo.org/v1", network);
        let debugger = AccudoDebugger::rest_client(Client::new(Url::parse(&endpoint)?))?;
        let version = debugger
            .get_version_by_account_sequence(txn.sender(), txn.sequence_number())
            .await?
            .unwrap();
        println!("Version: {:?}", version);
        println!(
            "Overview: https://explorer.accudo.org/txn/{:?}/userTxnOverview?network={}",
            version, network
        );
        println!(
            "Payload: https://explorer.accudo.org/txn/{:?}/payload?network={}",
            version, network
        );

        if self.execute {
            println!();
            println!("===============================");
            println!("Transaction re-execution result");
            println!("===============================");
            println!(
                "{:#?}",
                debugger
                    .execute_past_transactions(version, 1, false, 1, &[self.concurrency_level])
                    .await?
            );
        }

        Ok(())
    }
}
