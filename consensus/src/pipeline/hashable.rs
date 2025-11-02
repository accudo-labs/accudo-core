// Copyright © Accudo Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use accudo_crypto::HashValue;

pub trait Hashable {
    fn hash(&self) -> HashValue;
}
