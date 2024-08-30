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

async fn post<Q: serde::Serialize, R: serde::de::DeserializeOwned>(
    client: &reqwest::Client,
    url: &str,
    request: Q,
) -> std::result::Result<R, iamgroot::jsonrpc::Error> {
    let response = client
        .post(url)
        .json(&request)
        .send()
        .await
        .map_err(|e| {
            iamgroot::jsonrpc::Error::new(
                32101,
                format!("request failed: {e:?}"),
            )
        })?
        .json()
        .await
        .map_err(|e| {
            iamgroot::jsonrpc::Error::new(
                32102,
                format!("invalid response: {e:?}"),
            )
        })?;
    Ok(response)
}

#[derive(Clone)]
pub struct Http(pub reqwest::Client);

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl gen::client::HttpClient for Http {
    async fn post(
        &self,
        url: &str,
        request: &iamgroot::jsonrpc::Request,
    ) -> std::result::Result<
        iamgroot::jsonrpc::Response,
        iamgroot::jsonrpc::Error,
    > {
        post(&self.0, url, request).await
    }
}

impl gen::client::blocking::HttpClient for Http {
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
                .send_json(request)
                .map_err(|e| {
                    iamgroot::jsonrpc::Error::new(
                        32101,
                        format!("request failed: {e:?}"),
                    )
                })?
                .into_json()
                .map_err(|e| {
                    iamgroot::jsonrpc::Error::new(
                        32102,
                        format!("invalid response: {e:?}"),
                    )
                })?;
            Ok(response)
        }

        #[cfg(target_arch = "wasm32")]
        {
            use tokio::sync::oneshot::channel;
            let (tx, rx) = channel();

            let client = self.0.clone();
            let url = url.to_owned();
            let request = request.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // TODO: FIXME: "Uncaught RuntimeError: unreachable executed"
                web_sys::console::log_1(
                    &format!("url: {url}, req: {request:?}").into(),
                );
                let ret = post(&client, &url, &request).await;
                web_sys::console::log_1(&format!("ret: {ret:?}").into());
                let _ = tx.send(ret);
            });

            rx.blocking_recv().map_err(|e| {
                iamgroot::jsonrpc::Error::new(
                    32102,
                    format!("invalid response: {e:?}"),
                )
            })?
        }
    }
}

pub struct Client {
    starknet: StarknetClient<Http>,
    ethereum: EthereumClient,
}

impl Client {
    pub async fn new(config: &Config) -> Result<Self> {
        let http = Http(reqwest::Client::new());
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

    pub fn starknet(&self) -> &StarknetClient<Http> {
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
