use eyre::Result;

use crate::config::{get_gateway_url, Config};
use crate::feeder::GatewayClient;
use crate::gen::client::Client as StarknetClient;
use crate::gen::{gen, Felt, FunctionCall, Rpc};

const RPC_SPEC_VERSION: &str = "0.7.1";

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

impl PartialEq<State> for State {
    fn eq(&self, other: &State) -> bool {
        self.block_number == other.block_number
            && self.root.as_ref() == other.root.as_ref()
            && self.block_hash.as_ref() == other.block_hash.as_ref()
    }
}

#[derive(Clone)]
pub struct Http(pub reqwest::Client);

impl Http {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(reqwest::Client::new())
    }
}

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
        #[cfg(target_arch = "wasm32")]
        unreachable!("Blocking HTTP attempt: url={url} request={request:?}");

        #[cfg(not(target_arch = "wasm32"))]
        {
            ureq::post(url)
                .send_json(request)
                .map_err(|e| {
                    iamgroot::jsonrpc::Error::new(33101, e.to_string())
                })?
                .into_json()
                .map_err(|e| {
                    iamgroot::jsonrpc::Error::new(33102, e.to_string())
                })
        }
    }
}

pub struct Client<
    T: gen::client::HttpClient
        + gen::client::blocking::HttpClient
        + Clone
        + 'static,
> {
    starknet: StarknetClient<T>,
    gateway: GatewayClient,
    http: T,
}

impl<
        T: gen::client::HttpClient
            + gen::client::blocking::HttpClient
            + Clone
            + 'static,
    > Client<T>
{
    pub async fn new(config: &Config, http: T) -> Result<Self> {
        let starknet = StarknetClient::new(&config.starknet_rpc, http.clone());
        let rpc_spec_version = starknet.specVersion().await?;
        if rpc_spec_version != RPC_SPEC_VERSION {
            eyre::bail!("RPC spec version mismatch: expected {RPC_SPEC_VERSION} but got {rpc_spec_version}");
        }
        let url = if let Some(url) = config.gateway_url.as_ref() {
            url.as_str()
        } else {
            get_gateway_url(&config.starknet_rpc).await?
        };
        let gateway = GatewayClient::new(url)?;
        Ok(Self { starknet, gateway, http })
    }

    pub fn starknet(&self) -> &StarknetClient<T> {
        &self.starknet
    }

    pub fn execute(
        &self,
        request: FunctionCall,
        state: State,
    ) -> Result<Vec<Felt>> {
        let client = gen::client::blocking::Client::new(
            &self.starknet.url,
            self.http.clone(),
        );
        let call_info = crate::exe::call(client, request, state)?;
        call_info
            .execution
            .retdata
            .0
            .into_iter()
            .map(|felt| as_felt(&felt.to_bytes_be()))
            .collect()
    }

    pub async fn get_state(&self) -> Result<State> {
        self.gateway.get_state().await
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
