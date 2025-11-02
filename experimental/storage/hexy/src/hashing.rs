// Copyright (c) Accudo Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::ARITY;
use accudo_crypto::{
    hash::{CryptoHasher, HexyHasher},
    HashValue,
};
use anyhow::{ensure, Result};

#[derive(Default)]
pub struct HexyHashBuilder {
    hasher: HexyHasher,
    seen_children: usize,
}

impl HexyHashBuilder {
    pub fn add_child(&mut self, hash: &HashValue) -> Result<()> {
        ensure!(self.seen_children < ARITY, "Too many children");

        self.hasher.update(hash.as_ref());
        self.seen_children += 1;

        Ok(())
    }

    pub fn finish(self) -> Result<HashValue> {
        ensure!(self.seen_children == ARITY, "Not enough children");
        Ok(self.hasher.finish())
    }
}
