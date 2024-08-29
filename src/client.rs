use std::sync::Arc;

use eyre::{Context, Result};
use tokio::sync::RwLock;

use crate::eth::{EthereumClient, Helios};
use crate::gen::client::Client as StarknetClient;
use crate::gen::{gen, BlockId, Felt, Rpc};
use crate::{config::Config, gen::FunctionCall};

#[derive(Debug, Clone)]
pub struct State {
    pub block_number: u64,
    pub block_hash: Felt,
    pub root: Felt,
}

#[derive(Clone)]
pub struct AsyncHttp(pub reqwest::Client);

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl gen::client::HttpClient for AsyncHttp {
    async fn post(
        &self,
        url: &str,
        request: &iamgroot::jsonrpc::Request,
    ) -> std::result::Result<
        iamgroot::jsonrpc::Response,
        iamgroot::jsonrpc::Error,
    > {
        let response = self
            .0
            .post(url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                iamgroot::jsonrpc::Error::new(0, format!("LOL: {e:?}"))
            })?
            .json()
            .await
            .map_err(|e| {
                iamgroot::jsonrpc::Error::new(0, format!("LOL: {e:?}"))
            })?;
        Ok(response)
    }
}

#[derive(Clone)]
pub struct SyncHttp;

impl gen::client::blocking::HttpClient for SyncHttp {
    fn post(
        &self,
        url: &str,
        request: &iamgroot::jsonrpc::Request,
    ) -> std::result::Result<
        iamgroot::jsonrpc::Response,
        iamgroot::jsonrpc::Error,
    > {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let response = ureq::post(url)
                .send_json(&request)
                .map_err(|e| {
                    iamgroot::jsonrpc::Error::new(0, format!("LOL: {e:?}"))
                })?
                .into_json()
                .map_err(|e| {
                    iamgroot::jsonrpc::Error::new(0, format!("LOL: {e:?}"))
                })?;
            Ok(response)
        }

        #[cfg(target_arch = "wasm32")]
        {
            unimplemented!(
                "TODO: FIXME: use wasm-friendly blocking http client"
            )
        }
    }
}

pub struct Client {
    starknet: StarknetClient<AsyncHttp>,
    ethereum: EthereumClient,
}

impl Client {
    pub async fn new(config: &Config) -> Result<Self> {
        let http = AsyncHttp(reqwest::Client::new());
        let starknet = StarknetClient::new(&config.starknet_rpc, http);
        let ethereum = EthereumClient::new(config).await?;
        Ok(Self { starknet, ethereum })
    }

    pub async fn start(&self) -> Result<()> {
        self.ethereum.start().await
    }

    pub fn ethereum(&self) -> Arc<RwLock<Helios>> {
        self.ethereum.helios()
    }

    pub fn starknet(&self) -> &StarknetClient<AsyncHttp> {
        &self.starknet
    }

    pub async fn call_starknet(
        &self,
        request: FunctionCall,
        block_id: BlockId,
    ) -> Result<Vec<Felt>> {
        let ret = self.starknet.call(request, block_id).await?;
        Ok(ret)
    }

    pub async fn get_state(&self) -> Result<State> {
        let (block_number, block_hash, state_root) = self
            .ethereum
            .starknet_state()
            .await
            .context("beerus: get starknet state")?;

        Ok(State {
            block_number,
            block_hash: as_felt(block_hash.as_bytes())?,
            root: as_felt(state_root.as_bytes())?,
        })
    }

    pub async fn spec_version(&self) -> Result<String> {
        let version = self.starknet.specVersion().await?;
        Ok(version)
    }
}

fn as_felt(bytes: &[u8]) -> Result<Felt> {
    // RPC spec FELT regex: leading zeroes are not allowed
    let hex = hex::encode(bytes);
    let hex = hex.chars().skip_while(|c| c == &'0').collect::<String>();
    let hex = format!("0x{hex}");
    let felt = Felt::try_new(&hex)?;
    Ok(felt)
}
