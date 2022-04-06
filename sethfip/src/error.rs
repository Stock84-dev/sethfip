use std::path::PathBuf;
use url::Url;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid IPFS input: '{0:}', expected {1:}")]
    InvalidIpfsInput(Url, &'static str),
    #[error("Expected a file '{0:?}'")]
    ExpectedFile(PathBuf),
    #[error("File '{0:?}' doesn't exist")]
    MissingFile(PathBuf),
}
