#!/bin/bash
# Copyright (c) Accudo
# SPDX-License-Identifier: Apache-2.0
set -e

PROFILE=${PROFILE:-release}

echo "Building indexer and related binaries"
echo "PROFILE: $PROFILE"

echo "CARGO_TARGET_DIR: $CARGO_TARGET_DIR"

# Build all the rust binaries
cargo build --locked --profile=$PROFILE \
    -p accudo-indexer-grpc-cache-worker \
    -p accudo-indexer-grpc-file-store \
    -p accudo-indexer-grpc-data-service \
    -p accudo-nft-metadata-crawler \
    -p accudo-indexer-grpc-file-checker \
    -p accudo-indexer-grpc-data-service-v2 \
    -p accudo-indexer-grpc-manager \
    -p accudo-indexer-grpc-gateway \
    "$@"

# After building, copy the binaries we need to `dist` since the `target` directory is used as docker cache mount and only available during the RUN step
BINS=(
    accudo-indexer-grpc-cache-worker
    accudo-indexer-grpc-file-store
    accudo-indexer-grpc-data-service
    accudo-nft-metadata-crawler
    accudo-indexer-grpc-file-checker
    accudo-indexer-grpc-data-service-v2
    accudo-indexer-grpc-manager
    accudo-indexer-grpc-gateway
)

mkdir dist

for BIN in "${BINS[@]}"; do
    cp $CARGO_TARGET_DIR/$PROFILE/$BIN dist/$BIN
done
