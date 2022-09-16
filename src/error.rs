// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::store::LocalStore;
use canonical::CanonError;
use phoenix_core::Error as PhoenixError;
//use prost::encoding::bytes;
use rand_core::Error as RngError;
use std::{
    fmt,
    io::{self, Stderr},
};
use tonic::codegen::http;

use super::clients;
/// Wallet core error
pub(crate) type CoreError =
    dusk_wallet_core::Error<LocalStore, clients::State, clients::Prover>;

use thiserror::Error;
//use thiserror_no_std::Error;

/// Errors returned by this library

#[derive(Debug, thiserror::Error)]

pub enum Error {
    ///
    #[error("State Client errors")]
    State(StateError),
    /// Prover Client errors
    #[error("ProverError")]
    Prover(ProverError),
    /// Local Store errors
    #[error("ProverError")]
    Store(StoreError),
    /// Network error
    #[error("Network error")]
    Network(tonic::transport::Error),
    /// Rusk uri failure
    #[error("Rusk uri failure")]
    RuskURI(http::uri::InvalidUri),
    /// Rusk connection failure
    #[error(" Rusk connection failure")]
    RuskConn(tonic::transport::Error),
    /// Prover cluster connection failure
    #[error("Prover cluster connection failure")]
    ProverConn(tonic::transport::Error),
    /// Command not available in offline mode
    #[error("Command not available in offline mode")]
    Offline,
    /// Unauthorized to access this wallet
    #[error("Unauthorized to access this wallet")]
    Unauthorized,
    /// Filesystem errors
    #[error("Filesystem errors")]
    IO(io::Error),
    /// JSON serialization errors
    #[error("JSON serialization errors")]
    Json(serde_json::Error),
    /// Bytes encoding errors
    #[error("Bytes encoding errors")]
    Bytes(dusk_bytes::Error),
    /// Base58 errors
    #[error("Base58 errors")]
    Base58(bs58::decode::Error),
    /// Canonical errors
    #[error("Canonical errors")]
    Canon(canonical::CanonError),
    /// Random number generator errors
    #[error("Random number generator errors")]
    Rng(RngError),
    /// Transaction model errors
    #[error("Transaction model errors")]
    Phoenix(PhoenixError),
    /// Not enough balance to perform transaction
    #[error("Not enough balance to perform transaction")]
    NotEnoughBalance,
    /// Amount to transfer/stake cannot be zero
    #[error("Amount to transfer/stake cannot be zero")]
    AmountIsZero,
    /// Note combination for the given value is impossible given the maximum
    #[error(
        "Note combination for the given value is impossible given 
    the maximum amount of inputs in a transaction"
    )]
    NoteCombinationProblem,
    /// Not enough gas to perform this transaction
    #[error("Not enough gas to perform this transaction")]
    NotEnoughGas,
    /// Staking is only allowed when you're running your own local Rusk
    /// instance (Tip: Point `rusk_addr` to "localhost" or "127.0.0.1")
    #[error("Staking is only allowed when you're running your own local Rusk instance Tip: Point `rusk_addr` to localhost")]
    StakingNotAllowed,
    /// A stake already exists for this key
    #[error("A stake already exists for this key ")]
    AlreadyStaked,
    /// A stake does not exist for this key
    #[error("A stake does not exist for this key ")]
    NotStaked,
    /// No reward available for this key
    #[error("No reward available for this key ")]
    NoReward,
    /// Invalid address
    #[error("Invalid address ")]
    BadAddress,
    /// Address does not belong to this wallet
    #[error("Address does not belong to this wallet ")]
    AddressNotOwned,
    /// Recovery phrase is not valid
    #[error("Recovery phrase is not valid ")]
    InvalidMnemonicPhrase,
    /// Path provided is not a directory
    #[error("Path provided is not a directory ")]
    NotDirectory,
    /// Wallet file content is not valid
    #[error("Wallet file content is not valid ")]
    WalletFileCorrupted,
    /// File version not recognized
    #[error(" File version not recognized ")]
    UnknownFileVersion(u8, u8),
    /// Wallet file not found on disk
    #[error("Wallet file not found on disk ")]
    WalletFileNotExists,
    /// A wallet file with this name already exists
    #[error("A wallet file with this name already exists ")]
    WalletFileExists,
    /// Wallet file is missing
    #[error("Wallet file is missing ")]
    WalletFileMissing,
    /// Wrong wallet password
    #[error("Wrong wallet password ")]
    InvalidPassword,
    /// Socket connection is not available on Windows
    #[error("Socket connection is not available on Windows ")]
    SocketsNotSupported(String),
    /// Status callback needs to be set before connecting
    #[error("Status callback needs to be set before connecting ")]
    StatusWalletConnected,
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<dusk_bytes::Error> for Error {
    fn from(e: dusk_bytes::Error) -> Self {
        Self::Bytes(e)
    }
}

impl From<http::uri::InvalidUri> for Error {
    fn from(e: http::uri::InvalidUri) -> Self {
        Self::RuskURI(e)
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(e: tonic::transport::Error) -> Self {
        Self::Network(e)
    }
}

impl From<bs58::decode::Error> for Error {
    fn from(e: bs58::decode::Error) -> Self {
        Self::Base58(e)
    }
}

impl From<block_modes::InvalidKeyIvLength> for Error {
    fn from(_: block_modes::InvalidKeyIvLength) -> Self {
        Self::WalletFileCorrupted
    }
}

impl From<block_modes::BlockModeError> for Error {
    fn from(_: block_modes::BlockModeError) -> Self {
        Self::InvalidPassword
    }
}

impl From<bip39::ErrorKind> for Error {
    fn from(_: bip39::ErrorKind) -> Self {
        Self::InvalidMnemonicPhrase
    }
}

impl From<StateError> for Error {
    fn from(e: StateError) -> Self {
        Self::State(e)
    }
}

impl From<ProverError> for Error {
    fn from(e: ProverError) -> Self {
        Self::Prover(e)
    }
}

impl From<StoreError> for Error {
    fn from(e: StoreError) -> Self {
        Self::Store(e)
    }
}

impl From<CoreError> for Error {
    fn from(e: CoreError) -> Self {
        use dusk_wallet_core::Error as CoreErr;
        match e {
            CoreErr::Store(err) => Self::Store(err),
            CoreErr::State(err) => Self::State(err),
            CoreErr::Prover(err) => Self::Prover(err),
            CoreErr::Canon(err) => Self::Canon(err),
            CoreErr::Rng(err) => Self::Rng(err),
            CoreErr::Bytes(err) => Self::Bytes(err),
            CoreErr::Phoenix(err) => Self::Phoenix(err),
            CoreErr::NotEnoughBalance => Self::NotEnoughBalance,
            CoreErr::NoteCombinationProblem => Self::NoteCombinationProblem,
            CoreErr::AlreadyStaked { key: _, stake: _ } => Self::AlreadyStaked,
            CoreErr::NotStaked { key: _, stake: _ } => Self::NotStaked,
            CoreErr::NoReward { key: _, stake: _ } => Self::NoReward,
        }
    }
}

impl From<CanonError> for StateError {
    fn from(e: CanonError) -> Self {
        Self::Canon(e)
    }
}
impl From<dusk_bytes::Error> for StateError {
    fn from(e: dusk_bytes::Error) -> Self {
        Self::Bytes(e)
    }
}

/// State client errors
#[derive(Error, Debug)]
pub enum StateError {
    /// Status of a Rusk request
    #[error("Rusk returned an error:\n{}", self)]
    Rusk {
        #[from]
        source: tonic::Status,
    },
    /// Bytes encoding errors
    #[error("Rusk returned an error:\n{}", self)]
    Bytes(dusk_bytes::Error),

    /// Canonical errors
    #[error("A serialization error occurred:\n{:?}", self)]
    Canon(canonical::CanonError),

    /// Cache errors
    #[error("Failed to read/write cache:\n{:?}", self)]
    Cache {
        #[from]
        source: microkelvin::PersistError,
    },

    /// I/O errors
    #[error("An I/O error occurred\n{:?}", self)]
    Io {
        #[from]
        source: io::Error,
    },
}

/// Prover client errors
#[derive(Error, Debug)]
pub enum ProverError {
    /// Status of a Rusk request
    #[error("Rusk returned an error:\n{}", self)]
    Rusk {
        #[from]
        source: tonic::Status,
    },
    #[error("Rusk returned an error:\n{}", self)]
    /// Bytes encoding errors
    Bytes(dusk_bytes::Error),

    /// Canonical errors
    #[error("A serialization error occurred:\n{:?}", self)]
    Canon(canonical::CanonError),
    /// Transaction verification errors
    #[error("Transaction failed: {}", self)]
    Transaction(String),
}

impl From<dusk_bytes::Error> for ProverError {
    fn from(e: dusk_bytes::Error) -> Self {
        Self::Bytes(e)
    }
}

/// Store errors
pub enum StoreError {}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred in the store")
    }
}

impl fmt::Debug for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred in the store")
    }
}
