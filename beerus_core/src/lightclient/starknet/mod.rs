use crate::config::Config;
use async_trait::async_trait;
use eyre::Result;
use mockall::automock;
use starknet::{
    core::types::FieldElement,
    providers::jsonrpc::{
        models::FunctionCall,
        models::{BlockHashAndNumber, BlockId, ContractClass},
        HttpTransport, JsonRpcClient,
    },
};
use url::Url;

pub mod storage_proof;

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
    async fn get_nonce(&self, _block_number: u64, address: FieldElement) -> Result<FieldElement>;
    async fn chain_id(&self) -> Result<FieldElement>;
    async fn block_number(&self) -> Result<u64>;
    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber>;
    async fn get_class(
        &self,
        block_id: &BlockId,
        class_hash: FieldElement,
    ) -> Result<ContractClass>;
    async fn get_class_hash_at(
        &self,
        block_id: &BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement>;
    async fn get_class_at(
        &self,
        block_id: &BlockId,
        contract_address: FieldElement,
    ) -> Result<ContractClass>;
    async fn get_block_transaction_count(&self, block_id: &BlockId) -> Result<u64>;
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

    /// Get contract's nonce.
    /// Returns the nonce value.
    ///
    /// # Arguments
    ///
    /// * `address` - Address of the contract.
    ///
    ///
    /// # Returns
    ///
    /// `Ok(FieldElement)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn get_nonce(&self, _block_number: u64, address: FieldElement) -> Result<FieldElement> {
        self.client
            .get_nonce(
                &starknet::providers::jsonrpc::models::BlockId::Number(_block_number),
                address,
            )
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    async fn chain_id(&self) -> Result<FieldElement> {
        self.client.chain_id().await.map_err(|e| eyre::eyre!(e))
    }

    async fn block_number(&self) -> Result<u64> {
        self.client.block_number().await.map_err(|e| eyre::eyre!(e))
    }

    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber> {
        self.client
            .block_hash_and_number()
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    /// Get the contract class definition in the given block associated with the given hash.
    /// The contract class definition.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The block identifier.
    /// * `class_hash` - The class hash.
    ///
    /// # Returns
    ///
    /// `Ok(ContractClass)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn get_class(
        &self,
        block_id: &BlockId,
        class_hash: FieldElement,
    ) -> Result<ContractClass> {
        self.client
            .get_class(block_id, class_hash)
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    /// Get the contract class hash given a block Id and contract_address;

    ///
    /// # Arguments
    ///
    /// * `block_id` - The block identifier.
    /// * `contract_address` - The class hash.
    ///
    /// # Returns
    ///
    /// `Ok(FieldElement)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn get_class_hash_at(
        &self,
        block_id: &BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement> {
        self.client
            .get_class_hash_at(block_id, contract_address)
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    /// Get the contract class definition in the given block associated with the contract address.
    /// The contract class definition.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The block identifier.
    /// * `contract_address` - The contract address.
    ///
    /// # Returns
    ///
    /// `Ok(ContractClass)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn get_class_at(
        &self,
        block_id: &BlockId,
        contract_address: FieldElement,
    ) -> Result<ContractClass> {
        self.client
            .get_class_at(block_id, contract_address)
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    /// Get the number of transactions in a block given a block id.
    /// The number of transactions in a block.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The block identifier.
    ///
    /// # Returns
    ///
    /// `Ok(ContractClass)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn get_block_transaction_count(&self, block_id: &BlockId) -> Result<u64> {
        self.client
            .get_block_transaction_count(block_id)
            .await
            .map_err(|e| eyre::eyre!(e))
    }
}
