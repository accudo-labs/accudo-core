---
id: Accudo-framework
title: Accudo Framework
custom_edit_url: https://github.com/accudo-labs/accudo-core/edit/main/Accudo-move/Accudo-framework/README.md
---

## The Accudo Framework

The Accudo Framework defines the standard actions that can be performed on-chain
both by the Accudo VM---through the various prologue/epilogue functions---and by
users of the blockchain---through the allowed set of transactions. This
directory contains different directories that hold the source Move
modules and transaction scripts, along with a framework for generation of
documentation, ABIs, and error information from the Move source
files. See the [Layout](#layout) section for a more detailed overview of the structure.

## Documentation

Each of the main components of the Accudo Framework and contributing guidelines are documented separately. See them by version below:

* *Accudo tokens* - [main](https://github.com/accudo-labs/accudo-core/blob/main/accudo-move/framework/accudo-token/doc/overview.md), [testnet](https://github.com/accudo-labs/accudo-core/blob/testnet/accudo-move/framework/accudo-token/doc/overview.md), [devnet](https://github.com/accudo-labs/accudo-core/blob/devnet/accudo-move/framework/accudo-token/doc/overview.md)
* *Accudo framework* - [main](https://github.com/accudo-labs/accudo-core/blob/main/accudo-move/framework/accudo-framework/doc/overview.md), [testnet](https://github.com/accudo-labs/accudo-core/blob/testnet/accudo-move/framework/accudo-framework/doc/overview.md), [devnet](https://github.com/accudo-labs/accudo-core/blob/devnet/accudo-move/framework/accudo-framework/doc/overview.md)
* *Accudo stdlib* - [main](https://github.com/accudo-labs/accudo-core/blob/main/accudo-move/framework/accudo-stdlib/doc/overview.md), [testnet](https://github.com/accudo-labs/accudo-core/blob/testnet/accudo-move/framework/accudo-stdlib/doc/overview.md), [devnet](https://github.com/accudo-labs/accudo-core/blob/devnet/accudo-move/framework/accudo-stdlib/doc/overview.md)
* *Move stdlib* - [main](https://github.com/accudo-labs/accudo-core/blob/main/accudo-move/framework/move-stdlib/doc/overview.md), [testnet](https://github.com/accudo-labs/accudo-core/blob/testnet/accudo-move/framework/move-stdlib/doc/overview.md), [devnet](https://github.com/accudo-labs/accudo-core/blob/devnet/accudo-move/framework/move-stdlib/doc/overview.md)

Follow our [contributing guidelines](CONTRIBUTING.md) and basic coding standards for the Accudo Framework.

## Compilation and Generation

The documents above were created by the Move documentation generator for Accudo. It is available as part of the Accudo CLI. To see its options, run:
```shell
accudo move document --help
```

The documentation process is also integrated into the framework building process and will be automatically triggered like other derived artifacts, via `cached-packages` or explicit release building.

## Running Move tests

To test our Move code while developing the Accudo Framework, run `cargo test` inside this directory:

```
cargo test
```

(Alternatively, run `cargo test -p accudo-framework` from anywhere.)

To skip the Move prover tests, run:

```
cargo test -- --skip prover
```

To filter and run **all** the tests in specific packages (e.g., `accudo_stdlib`), run:

```
cargo test -- accudo_stdlib --skip prover
```

(See tests in `tests/move_unit_test.rs` to determine which filter to use; e.g., to run the tests in `accudo_framework` you must filter by `move_framework`.)

To **filter by test name or module name** in a specific package (e.g., run the `test_empty_range_proof` in `accudo_stdlib::ristretto255_bulletproofs`), run:

```
TEST_FILTER="test_range_proof" cargo test -- accudo_stdlib --skip prover
```

Or, e.g., run all the Bulletproof tests:
```
TEST_FILTER="bulletproofs" cargo test -- accudo_stdlib --skip prover
```

To show the amount of time and gas used in every test, set env var `REPORT_STATS=1`.
E.g.,
```
REPORT_STATS=1 TEST_FILTER="bulletproofs" cargo test -- accudo_stdlib --skip prover
```

Sometimes, Rust runs out of stack memory in dev build mode.  You can address this by either:
1. Adjusting the stack size

```
export RUST_MIN_STACK=4297152
```

2. Compiling in release mode

```
cargo test --release -- --skip prover
```

## Layout
The overall structure of the Accudo Framework is as follows:

```
├── accudo-framework                                 # Sources, testing and generated documentation for Accudo framework component
├── accudo-token                                 # Sources, testing and generated documentation for Accudo token component
├── accudo-stdlib                                 # Sources, testing and generated documentation for Accudo stdlib component
├── move-stdlib                                 # Sources, testing and generated documentation for Move stdlib component
├── cached-packages                                 # Tooling to generate SDK from move sources.
├── src                                     # Compilation and generation of information from Move source files in the Accudo Framework. Not designed to be used as a Rust library
├── releases                                    # Move release bundles
└── tests
```
