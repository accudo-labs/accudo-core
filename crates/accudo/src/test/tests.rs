// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    move_tool::{ArgWithType, FunctionArgType},
    CliResult, Tool,
};
use clap::Parser;
use std::str::FromStr;

/// In order to ensure that there aren't duplicate input arguments for untested CLI commands,
/// we call help on every command to ensure it at least runs
#[tokio::test]
async fn ensure_every_command_args_work() {
    assert_cmd_not_panic(&["accudo"]).await;

    assert_cmd_not_panic(&["accudo", "account"]).await;
    assert_cmd_not_panic(&["accudo", "account", "create", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "account", "create-resource-account", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "account", "fund-with-faucet", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "account", "list", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "account", "lookup-address", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "account", "rotate-key", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "account", "transfer", "--help"]).await;

    assert_cmd_not_panic(&["accudo", "config"]).await;
    assert_cmd_not_panic(&["accudo", "config", "generate-shell-completions", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "config", "init", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "config", "set-global-config", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "config", "show-global-config"]).await;
    assert_cmd_not_panic(&["accudo", "config", "show-profiles"]).await;

    assert_cmd_not_panic(&["accudo", "genesis"]).await;
    assert_cmd_not_panic(&["accudo", "genesis", "generate-genesis", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "genesis", "generate-keys", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "genesis", "generate-layout-template", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "genesis", "set-validator-configuration", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "genesis", "setup-git", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "genesis", "generate-admin-write-set", "--help"]).await;

    assert_cmd_not_panic(&["accudo", "governance"]).await;
    assert_cmd_not_panic(&["accudo", "governance", "execute-proposal", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "governance", "generate-upgrade-proposal", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "governance", "propose", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "governance", "vote", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "governance", "delegation_pool", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "governance", "delegation_pool", "vote", "--help"]).await;
    assert_cmd_not_panic(&[
        "accudo",
        "governance",
        "delegation_pool",
        "propose",
        "--help",
    ])
    .await;

    assert_cmd_not_panic(&["accudo", "info"]).await;

    assert_cmd_not_panic(&["accudo", "init", "--help"]).await;

    assert_cmd_not_panic(&["accudo", "key"]).await;
    assert_cmd_not_panic(&["accudo", "key", "generate", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "key", "extract-peer", "--help"]).await;

    assert_cmd_not_panic(&["accudo", "move"]).await;
    assert_cmd_not_panic(&["accudo", "move", "clean", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "move", "compile", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "move", "compile-script", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "move", "decompile", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "move", "disassemble", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "move", "download", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "move", "init", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "move", "list", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "move", "prove", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "move", "publish", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "move", "run", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "move", "run-script", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "move", "test", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "move", "transactional-test", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "move", "view", "--help"]).await;

    assert_cmd_not_panic(&["accudo", "node"]).await;
    assert_cmd_not_panic(&["accudo", "node", "check-network-connectivity", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "node", "get-stake-pool", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "node", "analyze-validator-performance", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "node", "bootstrap-db-from-backup", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "node", "initialize-validator", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "node", "join-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "node", "leave-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "node", "run-local-testnet", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "node", "show-validator-config", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "node", "show-validator-set", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "node", "show-validator-stake", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "node", "update-consensus-key", "--help"]).await;
    assert_cmd_not_panic(&[
        "accudo",
        "node",
        "update-validator-network-addresses",
        "--help",
    ])
    .await;

    assert_cmd_not_panic(&["accudo", "stake"]).await;
    assert_cmd_not_panic(&["accudo", "stake", "add-stake", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "stake", "increase-lockup", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "stake", "initialize-stake-owner", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "stake", "set-delegated-voter", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "stake", "set-operator", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "stake", "unlock-stake", "--help"]).await;
    assert_cmd_not_panic(&["accudo", "stake", "withdraw-stake", "--help"]).await;
}

/// Ensure we can parse URLs for args
#[tokio::test]
async fn ensure_can_parse_args_with_urls() {
    let result = ArgWithType::from_str("string:https://accudo.org").unwrap();
    matches!(result._ty, FunctionArgType::String);
    assert_eq!(
        result.arg,
        bcs::to_bytes(&"https://accudo.org".to_string()).unwrap()
    );
}

async fn assert_cmd_not_panic(args: &[&str]) {
    // When a command fails, it will have a panic in it due to an improperly setup command
    // thread 'main' panicked at 'Command propose: Argument names must be unique, but 'assume-yes' is
    // in use by more than one argument or group', ...

    match run_cmd(args).await {
        Ok(inner) => assert!(
            !inner.contains("panic"),
            "Failed to not panic cmd {}: {}",
            args.join(" "),
            inner
        ),
        Err(inner) => assert!(
            !inner.contains("panic"),
            "Failed to not panic cmd {}: {}",
            args.join(" "),
            inner
        ),
    }
}

async fn run_cmd(args: &[&str]) -> CliResult {
    let tool: Tool = Tool::try_parse_from(args).map_err(|msg| msg.to_string())?;
    tool.execute().await
}
