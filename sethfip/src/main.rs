//! Sethfip - a CLI utility to upload and download a file from IPFS while storing file hash (CID)
//! in a smart contract.

#![warn(missing_docs)]
#![deny(unused_must_use)]

use anyhow::Context;
use clap::Parser;
use http::uri::Scheme;
use ipfs_api_backend_hyper::{IpfsClient, TryFromUri};
use sethfip_core::{build_contract, decode_hex, download, upload};
use tracing::*;
use web3::types::Address;
use web3::Web3;

use crate::cli::{Args, Command};
use crate::error::AppError;

mod cli;
mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();
    match args.shared.verbose {
        0 => tracing_subscriber::fmt()
            .with_max_level(Level::ERROR)
            .init(),
        1 => tracing_subscriber::fmt().with_max_level(Level::INFO).init(),
        _ => tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .init(),
    };
    //    tracing_subscriber::fmt()
    //        .with_max_level(Level::TRACE)
    //        .init();
    let scheme = match args.shared.ipfs_node.scheme() {
        "http" => Scheme::HTTP,
        "https" => Scheme::HTTPS,
        _ => {
            return Err(
                AppError::InvalidIpfsInput(args.shared.ipfs_node.clone(), "url scheme").into(),
            )
        }
    };
    let host = args
        .shared
        .ipfs_node
        .host_str()
        .ok_or_else(|| AppError::InvalidIpfsInput(args.shared.ipfs_node.clone(), "host"))?;
    let port = args
        .shared
        .ipfs_node
        .port()
        .ok_or_else(|| AppError::InvalidIpfsInput(args.shared.ipfs_node.clone(), "port"))?;
    let client = IpfsClient::from_host_and_port(scheme, host, port)?;
    let web3 = Web3::new(web3::transports::Http::new(args.shared.eth_node.as_str())?);
    let contract = build_contract(web3.eth(), &args.shared.contract)?;
    let account = Address::from_slice(&decode_hex(&args.shared.account)?);

    match &args.command {
        Command::Upload(upload_args) => {
            let metadata = tokio::fs::metadata(&upload_args.input)
                .await
                .map_err(|_| AppError::MissingFile(upload_args.input.clone()))?;
            if !metadata.is_file() {
                Err(AppError::ExpectedFile(upload_args.input.clone()))?;
            }
            let output = upload(&contract, account, &client, &upload_args.input)
                .await
                .context("error encountered while uploading")?;
            println!("{}", output.cid);
            println!("{:?}", output.tx_hash);
        }
        Command::Download(download_args) => {
            let output = download_args.output_path.as_ref();
            let cid = download(&contract, account, &client, output)
                .await
                .context("error encountered while downloading")?;
            println!("{}", cid);
        }
    }
    Ok(())
}
