use crate::{config::Config, lightclient::starknet::storage_proof::GetProofOutput};

use crate::stdlib::boxed::Box;
use crate::stdlib::format;
use crate::stdlib::string::String;
use crate::stdlib::vec::Vec;

#[cfg(feature = "std")]
use mockall::automock;

use async_trait::async_trait;
use ethers::providers::{Http, Provider};
use eyre::Result;
use serde::Serialize;
use starknet::{
    core::types::FieldElement,
    providers::jsonrpc::{
        models::{
            BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction,
            BroadcastedDeployTransaction, BroadcastedInvokeTransaction, ContractClass,
            DeclareTransactionResult, DeployTransactionResult, EventFilter, EventsPage,
            InvokeTransactionResult, MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs,
            MaybePendingTransactionReceipt, SyncStatusType, Transaction,
        },
        models::{BroadcastedTransaction, FeeEstimate, FunctionCall, StateUpdate},
        {HttpTransport, JsonRpcClient},
    },
};
use url::Url;

pub mod storage_proof;

// #[cfg(feature="std")]
// #[automock]
#[cfg_attr(feature = "std", automock, async_trait)]
#[cfg_attr(not(feature = "std"), async_trait(?Send))]
pub trait StarkNetLightClient: Send + Sync {
    async fn start(&self) -> Result<()>;
    async fn call(&self, opts: FunctionCall, block_number: u64) -> Result<Vec<FieldElement>>;
    async fn estimate_fee(
        &self,
        tx: BroadcastedTransaction,
        block_id: &BlockId,
    ) -> Result<FeeEstimate>;
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
    async fn get_state_update(&self, block_id: &BlockId) -> Result<StateUpdate>;
    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage>;
    async fn syncing(&self) -> Result<SyncStatusType>;
    async fn add_invoke_transaction(
        &self,
        invoke_transaction: &BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult>;
    async fn add_deploy_transaction(
        &self,
        deploy_transaction: &BroadcastedDeployTransaction,
    ) -> Result<DeployTransactionResult>;

    async fn get_transaction_by_hash(&self, hash: FieldElement) -> Result<Transaction>;

    async fn get_block_with_txs(&self, block_id: &BlockId) -> Result<MaybePendingBlockWithTxs>;
    async fn get_block_with_tx_hashes(
        &self,
        block_id: &BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes>;

    async fn get_transaction_receipt(
        &self,
        hash: FieldElement,
    ) -> Result<MaybePendingTransactionReceipt>;

    async fn get_transaction_by_block_id_and_index(
        &self,
        block_id: &BlockId,
        index: u64,
    ) -> Result<Transaction>;
    async fn pending_transactions(&self) -> Result<Vec<Transaction>>;

    async fn get_contract_storage_proof(
        &self,
        contract_address: FieldElement,
        keys: Vec<FieldElement>,
        block: &BlockId,
    ) -> Result<GetProofOutput>;
    async fn add_declare_transaction(
        &self,
        declare_transaction: &BroadcastedDeclareTransaction,
    ) -> Result<DeclareTransactionResult>;
}

pub struct StarkNetLightClientImpl {
    client: JsonRpcClient<HttpTransport>,
    provider: Provider<Http>,
}

impl StarkNetLightClientImpl {
    pub fn new(config: &Config) -> Result<Self> {
        let url = Url::parse(config.starknet_rpc.clone().as_str())?;
        let provider = Provider::try_from(config.starknet_rpc.clone().as_str())?;
        Ok(Self {
            client: JsonRpcClient::new(HttpTransport::new(url)),
            provider,
        })
    }
}

#[cfg_attr(feature = "std", async_trait)]
#[cfg_attr(not(feature = "std"), async_trait(?Send))]
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

    /// Estimate the fee for a given StarkNet transaction
    /// Returns the fee estimate.
    ///
    /// # Arguments
    ///
    /// * `request` - The broadcasted transaction.
    /// * `block_id` - The block identifier.
    ///
    /// # Returns
    ///
    /// `Ok(FeeEstimate)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn estimate_fee(
        &self,
        tx: BroadcastedTransaction,
        block_id: &BlockId,
    ) -> Result<FeeEstimate> {
        self.client
            .estimate_fee(tx, block_id)
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

    /// Get the events.
    /// The list events.
    ///
    /// # Arguments
    ///
    /// * `params` - The query filters.
    ///
    /// # Returns
    ///
    /// `Ok(EventsPage)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage> {
        self.client
            .get_events(filter, continuation_token, chunk_size)
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    /// Get an object about the sync status, or false if the node is not synching.
    /// An object about the sync status, or false if the node is not synching.
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// `Ok(SyncStatusType)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn syncing(&self) -> Result<SyncStatusType> {
        self.client.syncing().await.map_err(|e| eyre::eyre!(e))
    }

    /// Get information about the result of executing the requested block.
    /// # Arguments
    ///
    /// * `block_id` - The block identifier.
    ///
    /// # Returns
    ///
    /// `Ok(StateUpdate)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn get_state_update(&self, block_id: &BlockId) -> Result<StateUpdate> {
        self.client
            .get_state_update(block_id)
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    /// Add an invoke transaction
    ///
    /// # Arguments
    ///
    /// invoke_transaction : Transaction data
    ///  
    ///
    /// # Returns
    ///
    /// Result : Invoke Transaction Result
    ///
    /// `Ok(InvokeTransactionResult)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn add_invoke_transaction(
        &self,
        invoke_transaction: &BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult> {
        self.client
            .add_invoke_transaction(invoke_transaction)
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    /// Add an invoke transaction
    ///
    /// # Arguments
    ///
    /// deploy_transaction : Transaction data
    ///
    ///
    /// # Returns
    ///
    /// Result : Deploy Transaction Result
    ///
    /// `Ok(DeployTransactionResult)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn add_deploy_transaction(
        &self,
        deploy_transaction: &BroadcastedDeployTransaction,
    ) -> Result<DeployTransactionResult> {
        self.client
            .add_deploy_transaction(deploy_transaction)
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    /// Get the transactions of a given block.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The block identifier.
    ///
    /// # Returns
    ///
    /// `Ok(MaybePendingBlockWithTxs)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn get_block_with_txs(&self, block_id: &BlockId) -> Result<MaybePendingBlockWithTxs> {
        self.client
            .get_block_with_txs(block_id)
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    /// Get the transaction that matches the
    /// given hash.
    /// # Arguments
    /// * `hash` - Transaction hash.
    /// # Returns
    /// `Ok(Transaction)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn get_transaction_by_hash(&self, hash: FieldElement) -> Result<Transaction> {
        self.client
            .get_transaction_by_hash(hash)
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    /// Get the transaction given a block id and index
    /// The number of transactions in a block.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The block identifier.
    /// * `index` - Transaction index
    /// # Returns
    ///
    /// `Ok(Transaction)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn get_transaction_by_block_id_and_index(
        &self,
        block_id: &BlockId,
        index: u64,
    ) -> Result<Transaction> {
        self.client
            .get_transaction_by_block_id_and_index(block_id, index)
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    /// Get the pending transactions.
    ///
    /// # Arguments
    /// # Returns
    ///
    /// Ok(Vec<Transaction>) if the operation was successful.
    /// Err(eyre::Report) if the operation failed.
    async fn pending_transactions(&self) -> Result<Vec<Transaction>> {
        self.client
            .pending_transactions()
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    /// Get a transaction's receipt, querying
    /// the transaction by its hash.
    /// # Arguments
    ///
    /// * `hash` - Hash of the transaction.
    ///
    /// # Returns
    ///
    /// `Ok(TransactionReceipt)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn get_transaction_receipt(
        &self,
        hash: FieldElement,
    ) -> Result<MaybePendingTransactionReceipt> {
        self.client
            .get_transaction_receipt(hash)
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    /// Get the block with tx hashes of a given block.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The block identifier.
    ///
    /// # Returns
    ///
    /// `Ok(MaybePendingBlockWithTxHashes)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn get_block_with_tx_hashes(
        &self,
        block_id: &BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes> {
        self.client
            .get_block_with_tx_hashes(block_id)
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    /// Get a contract storage storage proof
    ///
    /// # Arguments
    ///
    /// contract_address: Address of the contract
    /// keys: Storage slots of the contract keys that needs a proof
    /// block_id : ID of the block the proof is needed for
    ///
    /// # Returns
    ///
    /// Result: Storage proof for each keys requested.
    ///
    /// `Ok(InvokeTransactionResult)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn get_contract_storage_proof(
        &self,
        contract_address: FieldElement,
        keys: Vec<FieldElement>,
        block_id: &BlockId,
    ) -> Result<GetProofOutput> {
        let contract_address_str = format!("0x{contract_address:x}");
        let keys_str = keys.iter().map(|k| format!("0x{k:x}")).collect();

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        enum Param<'a> {
            Block(&'a BlockId),
            ContractAddress(String),
            Keys(Vec<String>),
        }

        let params = [
            Param::Block(block_id),
            Param::ContractAddress(contract_address_str),
            Param::Keys(keys_str),
        ];

        self.provider
            .request::<Vec<Param>, GetProofOutput>("pathfinder_getProof", Vec::from(params))
            .await
            .map_err(|e| eyre::eyre!(e))
    }

    /// Add an Declare transaction
    ///
    /// # Arguments
    ///
    /// declare_transaction : Transaction data
    ///  
    ///
    /// # Returns
    ///
    /// Result : Declare Transaction Result
    ///
    /// `Ok(DeclareTransactionResult)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn add_declare_transaction(
        &self,
        declare_transaction: &BroadcastedDeclareTransaction,
    ) -> Result<DeclareTransactionResult> {
        self.client
            .add_declare_transaction(declare_transaction)
            .await
            .map_err(|e| eyre::eyre!(e))
    }
}
