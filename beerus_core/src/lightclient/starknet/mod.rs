use crate::config::Config;
use async_trait::async_trait;
use eyre::Result;
use mockall::automock;
use starknet::{
    core::types::FieldElement,
    providers::jsonrpc::{models::FunctionCall, HttpTransport, JsonRpcClient},
};
use url::Url;

#[automock]
#[async_trait]
pub trait StarkNetLightClient: Send + Sync {
    async fn start(&mut self) -> Result<()>;
    async fn call(&self, opts: FunctionCall) -> Result<Vec<FieldElement>>;
}

pub struct StarkNetLightClientImpl {
    client: JsonRpcClient<HttpTransport>,
}

impl StarkNetLightClientImpl {
    pub fn new(config: &Config) -> Result<Self> {
        let url = Url::parse(config.starknet_rpc.clone().as_str())?;
        Ok(Self {
            client: JsonRpcClient::new(HttpTransport::new(url)),
        })
    }
}

#[async_trait]
impl StarkNetLightClient for StarkNetLightClientImpl {
    async fn start(&mut self) -> Result<()> {
        Ok(())
    }

    /// Call a contract on StarkNet.
    /// Returns the result of the call.
    ///
    /// # Arguments
    ///
    /// * `contract_address` - Address of the contract.
    /// * `selector` - Selector of the function to call.
    /// * `calldata` - Calldata of the function to call.
    ///
    /// # Returns
    ///
    /// `Ok(Vec<FieldElement>)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn call(&self, request: FunctionCall) -> Result<Vec<FieldElement>> {
        let response = self
            .client
            .call(
                request,
                &starknet::providers::jsonrpc::models::BlockId::Number(485441),
            )
            .await?;
        Ok(response)
    }
}
