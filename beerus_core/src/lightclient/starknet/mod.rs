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
    async fn start(&self) -> Result<()>;
    async fn call(&self, opts: FunctionCall, block_number: u64) -> Result<Vec<FieldElement>>;
    async fn get_storage_at(
        &self,
        address: FieldElement,
        key: FieldElement,
        block_number: u64,
    ) -> Result<FieldElement>;
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
    async fn start(&self) -> Result<()> {
        Ok(())
    }

    /// Get the value at a specific key in a contract's storage.
    /// Returns the value at the key.
    ///
    /// # Arguments
    ///
    /// * `address` - Address of the contract.
    /// * `key` - Key of the storage.
    ///
    /// # Returns
    ///
    /// `Ok(FieldElement)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn get_storage_at(
        &self,
        address: FieldElement,
        key: FieldElement,
        block_number: u64,
    ) -> Result<FieldElement> {
        self.client
            .get_storage_at(
                address,
                key,
                &starknet::providers::jsonrpc::models::BlockId::Number(block_number),
            )
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    /// Call a contract on StarkNet.
    /// Returns the result of the call.
    /// WARNING: This function is untrusted as there's no access list on StarkNet (yet @Avihu).
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
    async fn call(&self, request: FunctionCall, block_number: u64) -> Result<Vec<FieldElement>> {
        self.client
            .call(
                request,
                &starknet::providers::jsonrpc::models::BlockId::Number(block_number),
            )
            .await
            .map_err(|e| eyre::eyre!(e))
    }
}
