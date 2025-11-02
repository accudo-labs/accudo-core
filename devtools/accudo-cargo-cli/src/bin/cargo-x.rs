// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

use accudo_cargo_cli::{AccudoCargoCommand, SelectedPackageArgs};
use clap::Parser;
use std::process::exit;

#[derive(Parser)] // requires `derive` feature
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    #[command(name = "x")]
    AccudoCargoTool(AccudoCargoToolArgs),
}

#[derive(Parser)]
struct AccudoCargoToolArgs {
    #[command(subcommand)]
    cmd: AccudoCargoCommand,
    #[command(flatten)]
    package_args: SelectedPackageArgs,
}

fn main() {
    let CargoCli::AccudoCargoTool(args) = CargoCli::parse();
    let AccudoCargoToolArgs { cmd, package_args } = args;
    let result = cmd.execute(&package_args);

    // At this point, we'll want to print and determine whether to exit for an error code
    match result {
        Ok(_) => {},
        Err(inner) => {
            println!("{}", inner);
            exit(1);
        },
    }
}
