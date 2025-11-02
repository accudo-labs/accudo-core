#!/bin/bash
# Copyright (c) Accudo
# SPDX-License-Identifier: Apache-2.0
set -e

PROFILE=cli

echo "Building tools and services docker images"
echo "PROFILE: $PROFILE"
echo "CARGO_TARGET_DIR: $CARGO_TARGET_DIR"

# Build all the rust binaries
cargo build --locked --profile=$PROFILE \
    -p accudo \
    -p accudo-backup-cli \
    -p accudo-faucet-service \
    -p accudo-openapi-spec-generator \
    -p accudo-telemetry-service \
    -p accudo-keyless-pepper-service \
    -p accudo-debugger \
    -p accudo-transaction-emitter \
    -p accudo-release-builder \
    "$@"

# After building, copy the binaries we need to `dist` since the `target` directory is used as docker cache mount and only available during the RUN step
BINS=(
    accudo
    accudo-faucet-service
    accudo-openapi-spec-generator
    accudo-telemetry-service
    accudo-keyless-pepper-service
    accudo-debugger
    accudo-transaction-emitter
    accudo-release-builder
)

mkdir dist

for BIN in "${BINS[@]}"; do
    cp $CARGO_TARGET_DIR/$PROFILE/$BIN dist/$BIN
done

# Build the Accudo Move framework and place it in dist. It can be found afterwards in the current directory.
echo "Building the Accudo Move framework..."
(cd dist && cargo run --locked --profile=$PROFILE --package accudo-framework -- release)
