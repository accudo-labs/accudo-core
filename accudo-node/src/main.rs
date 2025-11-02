// Copyright © Accudo Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use accudo_node::{utils::ERROR_MSG_BAD_FEATURE_FLAGS, AccudoNodeArgs};
use clap::Parser;

#[cfg(unix)]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    // Check that we are not including any Move test natives
    accudo_vm::natives::assert_no_test_natives(ERROR_MSG_BAD_FEATURE_FLAGS);

    // Check that we do have the Move VM's tracing feature enabled
    move_vm_runtime::tracing::assert_move_vm_tracing_feature_disabled(ERROR_MSG_BAD_FEATURE_FLAGS);

    // Start the node
    AccudoNodeArgs::parse().run()
}
