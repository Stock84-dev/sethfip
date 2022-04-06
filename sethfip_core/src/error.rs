use std::fmt::{Display, Formatter};

use thiserror::Error;

#[derive(Debug)]
pub enum InputKind {
    Ipfs,
    Eth,
    File,
}

impl Display for InputKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            InputKind::Ipfs => "IPFS",
            InputKind::Eth => "ETH",
            InputKind::File => "file",
        };
        write!(f, "{}", name)
    }
}

#[derive(Error, Debug)]
#[error("{0:#?}")]
pub enum Error {
    Io(#[from] std::io::Error),
    Ipfs(#[from] ipfs_api_backend_hyper::Error),
    Abi(#[from] web3::ethabi::Error),
    HexDecode(#[from] hex::FromHexError),
    Contract(#[from] web3::contract::Error),
}
