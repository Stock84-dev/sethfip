use std::path::PathBuf;

use clap::{Parser, Subcommand};
use url::Url;

#[derive(Parser)]
#[clap(version, about)]
/// Sethfip - a CLI utility to upload and download a file from IPFS while storing file hash (CID)
/// in a smart contract.
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
    #[clap(flatten)]
    pub shared: ArgsShared,
}

#[derive(clap::Args)]
pub struct ArgsShared {
    #[clap(long, short, default_value = "http://localhost:5001")]
    /// Url to IPFS node
    pub ipfs_node: Url,
    #[clap(long, short, default_value = "http://localhost:8545")]
    /// Url to Ethereum node
    pub eth_node: Url,
    #[clap(
        long,
        short,
        default_value = "eaff8422d499714ffe4382f681c9087dde36d414"
    )]
    /// The smart contract address to use
    pub contract: String,
    #[clap(long, short)]
    /// Account address to interact with
    pub account: String,
    #[clap(parse(from_occurrences))]
    /// -v warning level
    /// -vv info level
    /// -vvv show all
    pub verbose: u8,
}

#[derive(Subcommand)]
pub enum Command {
    /// Uploads a file to IPFS and stores CID in a smart contract.
    Upload(UploadArgs),
    /// Reads current CID from smart contract and downloads a file from IPFS.
    Download(DownloadArgs),
}

#[derive(clap::Args)]
pub struct UploadArgs {
    #[clap(long, short, parse(from_os_str))]
    /// The file to upload.
    pub input: PathBuf,
}

#[derive(clap::Args)]
pub struct DownloadArgs {
    #[clap(long, short, parse(from_os_str))]
    /// Output path for downloaded file, if ommited it will be placed in current directory with CID
    /// as its name.
    pub output_path: Option<PathBuf>,
}
