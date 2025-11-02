// Copyright (c) Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

pub use accudo_crypto::constant_time;
use dudect_bencher::ctbench_main;

ctbench_main!(constant_time::zkcrypto_scalar_mul::run_bench);
