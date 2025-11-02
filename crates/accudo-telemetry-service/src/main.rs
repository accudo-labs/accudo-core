#![forbid(unsafe_code)]

// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

use accudo_telemetry_service::AccudoTelemetryServiceArgs;
use clap::Parser;

#[tokio::main]
async fn main() {
    accudo_logger::Logger::new().init();
    AccudoTelemetryServiceArgs::parse().run().await;
}
