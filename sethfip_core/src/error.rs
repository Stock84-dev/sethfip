//! Sethfip Error

use thiserror::Error;

/// Sethfip `Result` type.
pub type Result<T> = std::result::Result<T, crate::error::Error>;

/// Errors which can occur when intercting with any functions of this crate.
#[derive(Error, Debug)]
#[error("{0:#?}")]
pub enum Error {
    /// File IO error
    Io(#[from] std::io::Error),
    /// IPFS error
    Ipfs(#[from] ipfs_api_backend_hyper::Error),
    /// Ethereum smart contract ABI error
    Abi(#[from] web3::ethabi::Error),
    /// Can occur when decoding hex from string
    HexDecode(#[from] hex::FromHexError),
    /// Smart contract error
    Contract(#[from] web3::contract::Error),
}
