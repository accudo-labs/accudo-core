// Copyright © Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use accudo_metrics_core::{
    exponential_buckets, make_thread_local_histogram_vec, make_thread_local_int_counter_vec,
};

make_thread_local_histogram_vec!(
    pub(crate),
    TIMER,
    "accudo_storage_interface_timer_seconds",
    "Various timers for performance analysis.",
    &["name"],
    exponential_buckets(/*start=*/ 1e-6, /*factor=*/ 2.0, /*count=*/ 22).unwrap(),
);

make_thread_local_int_counter_vec!(
    pub(crate),
    COUNTER,
    "accudo_storage_interface_counter",
    "Various counters for storage-interface.",
    &["name"],
);
