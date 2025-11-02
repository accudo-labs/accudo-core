// Copyright © Accudo Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

//! This module defines error types used by `AccudoDB`.
use accudo_types::state_store::errors::StateViewError;
use std::sync::mpsc::RecvError;
use thiserror::Error;

/// This enum defines errors commonly used among `AccudoDB` APIs.
#[derive(Clone, Debug, Error)]
pub enum AccudoDbError {
    /// A requested item is not found.
    #[error("{0} not found.")]
    NotFound(String),
    /// Requested too many items.
    #[error("Too many items requested: at least {0} requested, max is {1}")]
    TooManyRequested(u64, u64),
    #[error("Missing state root node at version {0}, probably pruned.")]
    MissingRootError(u64),
    /// Other non-classified error.
    #[error("AccudoDB Other Error: {0}")]
    Other(String),
    #[error("AccudoDB RocksDb Error: {0}")]
    RocksDbIncompleteResult(String),
    #[error("AccudoDB RocksDB Error: {0}")]
    OtherRocksDbError(String),
    #[error("AccudoDB bcs Error: {0}")]
    BcsError(String),
    #[error("AccudoDB IO Error: {0}")]
    IoError(String),
    #[error("AccudoDB Recv Error: {0}")]
    RecvError(String),
    #[error("AccudoDB ParseInt Error: {0}")]
    ParseIntError(String),
}

impl From<anyhow::Error> for AccudoDbError {
    fn from(error: anyhow::Error) -> Self {
        Self::Other(format!("{}", error))
    }
}

impl From<bcs::Error> for AccudoDbError {
    fn from(error: bcs::Error) -> Self {
        Self::BcsError(format!("{}", error))
    }
}

impl From<RecvError> for AccudoDbError {
    fn from(error: RecvError) -> Self {
        Self::RecvError(format!("{}", error))
    }
}

impl From<std::io::Error> for AccudoDbError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(format!("{}", error))
    }
}

impl From<std::num::ParseIntError> for AccudoDbError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::Other(format!("{}", error))
    }
}

impl From<AccudoDbError> for StateViewError {
    fn from(error: AccudoDbError) -> Self {
        match error {
            AccudoDbError::NotFound(msg) => StateViewError::NotFound(msg),
            AccudoDbError::Other(msg) => StateViewError::Other(msg),
            _ => StateViewError::Other(format!("{}", error)),
        }
    }
}

impl From<StateViewError> for AccudoDbError {
    fn from(error: StateViewError) -> Self {
        match error {
            StateViewError::NotFound(msg) => AccudoDbError::NotFound(msg),
            StateViewError::Other(msg) => AccudoDbError::Other(msg),
            StateViewError::BcsError(err) => AccudoDbError::BcsError(err.to_string()),
        }
    }
}
