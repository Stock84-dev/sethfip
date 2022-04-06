#![warn(missing_docs)]
#![deny(unused_must_use)]

use std::path::Path;

use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use serde_json::{Map, Value};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::{debug, info, trace};
use web3::api::Eth;
use web3::contract::{Contract, Options};
use web3::futures::StreamExt;
use web3::types::{Address, H256};
use web3::{ethabi, Transport};

pub mod error;
mod utils;
pub use utils::decode_hex;

pub type Result<T> = std::result::Result<T, crate::error::Error>;

/// Uploads a file to IPFS and saves CID in a smart contract.
///
/// Returns transaction hash and CID of a file.
///
/// # Examples
///
/// ```no_run
/// # async fn try_main() -> Result<(), Box<dyn std::error::Error>> {
/// use ipfs_api_backend_hyper::{IpfsClient, TryFromUri};
/// use sethfip_core::{build_contract, decode_hex, upload};
/// use web3::types::Address;
/// use web3::Web3;
///
/// let ipfs = IpfsClient::from_host_and_port(http::uri::Scheme::HTTP, "localhost", 5001)?;
/// let web3 = Web3::new(web3::transports::Http::new("http://localhost:8545")?);
/// let contract = build_contract(web3.eth(), "eaff8422d499714ffe4382f681c9087dde36d414")?;
/// let account = Address::from_slice(&decode_hex("0x084c7D6B56267b811748A1Af3b3973da95641f50")?);
/// upload(&contract, account, &ipfs, "/file/path").await?;
/// # Ok(())
/// # }
/// # tokio::runtime::Runtime::new().unwrap().block_on(try_main()).unwrap();
/// ```
pub async fn upload<T: Transport>(
    contract: &Contract<T>,
    account: Address,
    ipfs: &IpfsClient,
    path: impl AsRef<Path>,
) -> Result<UploadOutput> {
    trace!("Uploading file...");
    let path = path.as_ref();
    let response = ipfs.add_path(path).await?;
    debug!("{:#?}", response);
    let hash = &response[0].hash;
    info!("File `{:?}` uploaded, CID={}", path, hash);
    println!("{}", hash);
    trace!("Saving CID to smart contract...");
    let tx = contract
        .call("set", (hash.to_string(),), account, Options::default())
        .await?;
    Ok(UploadOutput {
        cid: response.into_iter().next().unwrap().hash,
        tx_hash: tx,
    })
}

/// Queries a smart contract for latest CID stored. Downloads a file from IPFS and stores it in a
/// provided path. If path is None then file will be saved in current working directory with CID as
/// its name.
///
/// Returns CID of a file.
///
/// # Examples
///
/// ```no_run
/// # async fn try_main() -> Result<(), Box<dyn std::error::Error>> {
/// use ipfs_api_backend_hyper::{IpfsClient, TryFromUri};
/// use sethfip_core::{build_contract, decode_hex, download};
/// use web3::types::Address;
/// use web3::Web3;
///
/// let ipfs = IpfsClient::from_host_and_port(http::uri::Scheme::HTTP, "localhost", 5001)?;
/// let web3 = Web3::new(web3::transports::Http::new("http://localhost:8545")?);
/// let contract = build_contract(web3.eth(), "eaff8422d499714ffe4382f681c9087dde36d414")?;
/// let account = Address::from_slice(&decode_hex("0x084c7D6B56267b811748A1Af3b3973da95641f50")?);
/// download(&contract, account, &ipfs, Some("output/path")).await?;
/// # Ok(())
/// # }
/// # tokio::runtime::Runtime::new().unwrap().block_on(try_main()).unwrap();
/// ```
pub async fn download<T: Transport>(
    contract: &Contract<T>,
    account: Address,
    ipfs: &IpfsClient,
    output_path: Option<impl AsRef<Path>>,
) -> Result<String> {
    let cid: String = contract
        .query("get", (), account, Options::default(), None)
        .await?;
    println!("{}", cid);
    let cid_path = Path::new(&cid);
    let file_name = match &output_path {
        None => &cid_path,
        Some(name) => name.as_ref(),
    };
    let mut file = File::create(file_name).await?;
    let mut stream = ipfs.cat(&cid);
    trace!("Downloading file");
    while let Some(data) = stream.next().await {
        let data = data?;
        file.write_all(data.as_ref()).await?;
    }

    Ok(cid)
}

/// Builds a contract object by providing transport and an address.
pub fn build_contract<T: Transport>(eth: Eth<T>, contract_address: &str) -> Result<Contract<T>> {
    let contract_artifact: Map<String, Value> =
        serde_json::from_slice(include_bytes!("../build/contracts/Storage.json"))
            .expect("Could not parse contract artifact");
    let abi = contract_artifact.get("abi").unwrap().to_string();
    let abi_contract = ethabi::Contract::load(&mut abi.as_bytes())?;
    let contract_address = Address::from_slice(&decode_hex(contract_address)?);
    let contract = Contract::new(eth, contract_address, abi_contract);
    Ok(contract)
}

pub struct UploadOutput {
    pub cid: String,
    pub tx_hash: H256,
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Seek, SeekFrom, Write};

    use anyhow::Result;
    use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
    use tempfile::NamedTempFile;
    use web3::contract::Options;
    use web3::futures::StreamExt;
    use web3::types::Address;
    use web3::Web3;

    use crate::{build_contract, decode_hex, download, upload};

    const ETH_NODE: &'static str = "http://localhost:8545";
    const CONTRACT_ADDRESS: &'static str = "eaff8422d499714ffe4382f681c9087dde36d414";
    const ACCOUNT_ADDRESS: &'static str = "0x084c7D6B56267b811748A1Af3b3973da95641f50";

    #[tokio::test]
    async fn up_down_works() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        let data = "ybkbShMmtbKdkPlcSJC22Q6njw0gbynk5D4N3JfKF1rps991DD3o8nTPy3zICg1OmB9PeuoN";
        file.write_all(data.as_bytes())?;

        let client = IpfsClient::default();
        let web3 = Web3::new(web3::transports::Http::new(ETH_NODE)?);
        let contract = build_contract(web3.eth(), CONTRACT_ADDRESS)?;
        let account = Address::from_slice(&decode_hex(ACCOUNT_ADDRESS)?);

        let output = upload(&contract, account, &client, file.path()).await?;

        let cid: String = contract
            .query("get", (), account, Options::default(), None)
            .await?;
        assert_eq!(output.cid, cid, "CID not stored in smart contract");

        let mut stored = Vec::new();
        let mut stream = client.cat(&cid);
        while let Some(data) = stream.next().await {
            let data = data?;
            stored.extend_from_slice(data.as_ref());
        }
        assert_eq!(data.as_bytes(), stored, "file not stored in IPFS");

        file.seek(SeekFrom::Start(0))?;
        let download_cid = download(&contract, account, &client, Some(file.path())).await?;
        assert_eq!(download_cid, cid, "wrong CID stored in smart contract");

        let mut downloaded = Vec::new();
        file.read_to_end(&mut downloaded)?;
        assert_eq!(downloaded, data.as_bytes(), "wrong file downloaded");

        Ok(())
    }
}
