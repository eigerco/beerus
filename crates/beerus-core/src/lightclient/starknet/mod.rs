use crate::{config::Config, lightclient::starknet::storage_proof::GetProofOutput};

use crate::lightclient::starknet::errors::JsonRpcClientErrorWrapper;
use crate::stdlib::boxed::Box;
use crate::stdlib::format;
use crate::stdlib::string::String;
use crate::stdlib::vec::Vec;

use core::convert::TryFrom;

#[cfg(feature = "std")]
use mockall::automock;

use async_trait::async_trait;
use ethers::providers::{Http, Provider};
use eyre::Result as EyreResult;
use reqwest::Error as ReqwestError;
use serde::Serialize;
use starknet::providers::jsonrpc::{JsonRpcClientError, JsonRpcError};
use starknet::{
    core::types::FieldElement,
    providers::jsonrpc::{
        models::{
            BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction,
            BroadcastedDeployTransaction, BroadcastedInvokeTransaction, BroadcastedTransaction,
            ContractClass, DeclareTransactionResult, DeployTransactionResult, EventFilter,
            EventsPage, FeeEstimate, FunctionCall, InvokeTransactionResult,
            MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs,
            MaybePendingTransactionReceipt, StateUpdate, SyncStatusType, Transaction,
        },
        HttpTransport, JsonRpcClient,
    },
};
use url::Url;
mod errors;
pub mod storage_proof;

// #[cfg(feature="std")]
// #[automock]
#[cfg_attr(feature = "std", automock, async_trait)]
#[cfg_attr(not(feature = "std"), async_trait(?Send))]
pub trait StarkNetLightClient: Send + Sync {
    async fn start(&self) -> EyreResult<()>;

    async fn call(
        &self,
        opts: FunctionCall,
        block_number: u64,
    ) -> Result<Vec<FieldElement>, JsonRpcError>;

    async fn estimate_fee(
        &self,
        tx: BroadcastedTransaction,
        block_id: &BlockId,
    ) -> Result<FeeEstimate, JsonRpcError>;

    async fn get_storage_at(
        &self,
        address: FieldElement,
        key: FieldElement,
        block_number: u64,
    ) -> Result<FieldElement, JsonRpcError>;

    async fn get_nonce(
        &self,
        block_id: &BlockId,
        address: FieldElement,
    ) -> Result<FieldElement, JsonRpcError>;

    async fn chain_id(&self) -> Result<FieldElement, JsonRpcError>;

    async fn block_number(&self) -> Result<u64, JsonRpcError>;

    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, JsonRpcError>;

    async fn get_class(
        &self,
        block_id: &BlockId,
        class_hash: FieldElement,
    ) -> Result<ContractClass, JsonRpcError>;

    async fn get_class_hash_at(
        &self,
        block_id: &BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, JsonRpcError>;

    async fn get_class_at(
        &self,
        block_id: &BlockId,
        contract_address: FieldElement,
    ) -> Result<ContractClass, JsonRpcError>;

    async fn get_state_update(&self, block_id: &BlockId) -> Result<StateUpdate, JsonRpcError>;

    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, JsonRpcError>;

    async fn syncing(&self) -> Result<SyncStatusType, JsonRpcError>;

    async fn add_invoke_transaction(
        &self,
        invoke_transaction: &BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult, JsonRpcError>;
    async fn add_deploy_transaction(
        &self,
        deploy_transaction: &BroadcastedDeployTransaction,
    ) -> Result<DeployTransactionResult, JsonRpcError>;

    async fn get_transaction_by_hash(
        &self,
        hash: FieldElement,
    ) -> Result<Transaction, JsonRpcError>;

    async fn get_block_with_tx_hashes(
        &self,
        block_id: &BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes, JsonRpcError>;

    async fn get_transaction_receipt(
        &self,
        hash: FieldElement,
    ) -> Result<MaybePendingTransactionReceipt, JsonRpcError>;

    async fn get_transaction_by_block_id_and_index(
        &self,
        block_id: &BlockId,
        index: u64,
    ) -> Result<Transaction, JsonRpcError>;

    async fn pending_transactions(&self) -> Result<Vec<Transaction>, JsonRpcError>;

    async fn get_contract_storage_proof(
        &self,
        contract_address: FieldElement,
        keys: Vec<FieldElement>,
        block: &BlockId,
    ) -> Result<GetProofOutput, JsonRpcError>;

    async fn get_block_with_txs(
        &self,
        block_id: &BlockId,
    ) -> Result<MaybePendingBlockWithTxs, JsonRpcError>;

    async fn get_block_transaction_count(&self, block_id: &BlockId) -> Result<u64, JsonRpcError>;

    async fn add_declare_transaction(
        &self,
        declare_transaction: &BroadcastedDeclareTransaction,
    ) -> Result<DeclareTransactionResult, JsonRpcError>;
}

pub struct StarkNetLightClientImpl {
    client: JsonRpcClient<HttpTransport>,
    provider: Provider<Http>,
}

impl StarkNetLightClientImpl {
    pub fn new(config: &Config) -> EyreResult<Self> {
        let url = Url::parse(config.starknet_rpc.clone().as_str())?;
        let provider = Provider::try_from(config.starknet_rpc.clone().as_str())?;
        Ok(Self {
            client: JsonRpcClient::new(HttpTransport::new(url)),
            provider,
        })
    }

    /// Maps a `JsonRpcClientError` to a `JsonRpcError`.
    ///
    /// # Arguments
    ///
    /// * `method_name` - The name of the method where the error occurred.
    /// * `client_error` - The `JsonRpcClientError` to be mapped.
    ///
    /// # Returns
    ///
    /// The mapped `JsonRpcError`.
    fn map_to_rpc_error(
        method_name: &str,
        client_error: JsonRpcClientError<ReqwestError>,
    ) -> JsonRpcError {
        let error = JsonRpcError::try_from(JsonRpcClientErrorWrapper::from(client_error));
        match error {
            Ok(rpc_error) => rpc_error,
            Err(unknown_error) => JsonRpcError {
                code: 520,
                message: format!("[{}] {}", method_name, unknown_error),
            },
        }
    }
}

#[cfg_attr(feature = "std", async_trait)]
#[cfg_attr(not(feature = "std"), async_trait(?Send))]
impl StarkNetLightClient for StarkNetLightClientImpl {
    async fn start(&self) -> EyreResult<()> {
        Ok(())
    }

    /// Call a contract on StarkNet.
    ///
    /// # Arguments
    ///
    /// * `request` - The function call request.
    /// * `block_number` - The block number.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the result of the call if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn call(
        &self,
        request: FunctionCall,
        block_number: u64,
    ) -> Result<Vec<FieldElement>, JsonRpcError> {
        self.client
            .call(request, &BlockId::Number(block_number))
            .await
            .map_err(|e| Self::map_to_rpc_error("call", e))
    }

    /// Estimate the fee for a given StarkNet transaction.
    ///
    /// # Arguments
    ///
    /// * `tx` - The broadcasted transaction.
    /// * `block_id` - The block identifier.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the fee estimate if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn estimate_fee(
        &self,
        tx: BroadcastedTransaction,
        block_id: &BlockId,
    ) -> Result<FeeEstimate, JsonRpcError> {
        self.client
            .estimate_fee(tx, block_id)
            .await
            .map_err(|e| Self::map_to_rpc_error("estimate_fee", e))
    }

    /// Get the value at a specific key in a contract's storage.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the contract.
    /// * `key` - The key of the storage.
    /// * `block_number` - The block number.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the value at the key if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn get_storage_at(
        &self,
        address: FieldElement,
        key: FieldElement,
        block_number: u64,
    ) -> Result<FieldElement, JsonRpcError> {
        self.client
            .get_storage_at(address, key, &BlockId::Number(block_number))
            .await
            .map_err(|e| Self::map_to_rpc_error("get_storage_at", e))
    }

    /// Get the nonce of a contract.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The block identifier.
    /// * `address` - The address of the contract.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the nonce value if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn get_nonce(
        &self,
        block_id: &BlockId,
        address: FieldElement,
    ) -> Result<FieldElement, JsonRpcError> {
        self.client
            .get_nonce(block_id, address)
            .await
            .map_err(|e| Self::map_to_rpc_error("get_nonce", e))
    }

    /// Get the chain ID of the blockchain network.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the chain ID if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn chain_id(&self) -> Result<FieldElement, JsonRpcError> {
        self.client
            .chain_id()
            .await
            .map_err(|e| Self::map_to_rpc_error("chain_id", e))
    }

    /// Get the block number of the latest block.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the block number if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn block_number(&self) -> Result<u64, JsonRpcError> {
        self.client
            .block_number()
            .await
            .map_err(|e| Self::map_to_rpc_error("block_number", e))
    }

    /// Get the block hash and number of the latest block.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `BlockHashAndNumber` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, JsonRpcError> {
        self.client
            .block_hash_and_number()
            .await
            .map_err(|e| Self::map_to_rpc_error("block_hash_and_number", e))
    }

    /// Get the contract class definition in the given block associated with the given hash.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The block identifier.
    /// * `class_hash` - The class hash.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `ContractClass` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn get_class(
        &self,
        block_id: &BlockId,
        class_hash: FieldElement,
    ) -> Result<ContractClass, JsonRpcError> {
        self.client
            .get_class(block_id, class_hash)
            .await
            .map_err(|e| Self::map_to_rpc_error("get_class", e))
    }

    /// Get the contract class hash given a block ID and contract address.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The block identifier.
    /// * `contract_address` - The contract address.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `FieldElement` representing the class hash if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn get_class_hash_at(
        &self,
        block_id: &BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, JsonRpcError> {
        self.client
            .get_class_hash_at(block_id, contract_address)
            .await
            .map_err(|e| Self::map_to_rpc_error("get_class_hash_at", e))
    }

    /// Get the contract class definition in the given block associated with the contract address.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The block identifier.
    /// * `contract_address` - The contract address.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `ContractClass` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn get_class_at(
        &self,
        block_id: &BlockId,
        contract_address: FieldElement,
    ) -> Result<ContractClass, JsonRpcError> {
        self.client
            .get_class_at(block_id, contract_address)
            .await
            .map_err(|e| Self::map_to_rpc_error("get_class_at", e))
    }

    /// Get information about the result of executing the requested block.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The block identifier.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `StateUpdate` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn get_state_update(&self, block_id: &BlockId) -> Result<StateUpdate, JsonRpcError> {
        self.client
            .get_state_update(block_id)
            .await
            .map_err(|e| Self::map_to_rpc_error("get_state_update", e))
    }

    /// Get events based on the provided filters.
    ///
    /// # Arguments
    ///
    /// * `filter` - The query filters.
    /// * `continuation_token` - Optional continuation token for pagination.
    /// * `chunk_size` - The number of events to retrieve in each chunk.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `EventsPage` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, JsonRpcError> {
        self.client
            .get_events(filter, continuation_token, chunk_size)
            .await
            .map_err(|e| Self::map_to_rpc_error("get_events", e))
    }

    /// Get information about the sync status of the node.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `SyncStatusType` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn syncing(&self) -> Result<SyncStatusType, JsonRpcError> {
        self.client
            .syncing()
            .await
            .map_err(|e| Self::map_to_rpc_error("syncing", e))
    }

    /// Add an invoke transaction.
    ///
    /// # Arguments
    ///
    /// * `invoke_transaction`: Transaction data.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `InvokeTransactionResult` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn add_invoke_transaction(
        &self,
        invoke_transaction: &BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult, JsonRpcError> {
        self.client
            .add_invoke_transaction(invoke_transaction)
            .await
            .map_err(|e| Self::map_to_rpc_error("add_invoke_transaction", e))
    }

    /// Add an deploy transaction.
    ///
    /// # Arguments
    ///
    /// * `deploy_transaction`: Transaction data.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `DeployTransactionResult` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn add_deploy_transaction(
        &self,
        deploy_transaction: &BroadcastedDeployTransaction,
    ) -> Result<DeployTransactionResult, JsonRpcError> {
        self.client
            .add_deploy_transaction(deploy_transaction)
            .await
            .map_err(|e| Self::map_to_rpc_error("add_deploy_transaction", e))
    }

    /// Get the transaction that matches the given hash.
    ///
    /// # Arguments
    ///
    /// * `hash`: Transaction hash.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `Transaction` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn get_transaction_by_hash(
        &self,
        hash: FieldElement,
    ) -> Result<Transaction, JsonRpcError> {
        self.client
            .get_transaction_by_hash(hash)
            .await
            .map_err(|e| Self::map_to_rpc_error("get_transaction_by_hash", e))
    }

    /// Get the block with transaction hashes of a given block.
    ///
    /// # Arguments
    ///
    /// * `block_id`: The block identifier.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `MaybePendingBlockWithTxHashes` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn get_block_with_tx_hashes(
        &self,
        block_id: &BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes, JsonRpcError> {
        self.client
            .get_block_with_tx_hashes(block_id)
            .await
            .map_err(|e| Self::map_to_rpc_error("get_block_with_tx_hashes", e))
    }

    /// Get a transaction's receipt by querying the transaction using its hash.
    ///
    /// # Arguments
    ///
    /// * `hash`: Hash of the transaction.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `MaybePendingTransactionReceipt` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn get_transaction_receipt(
        &self,
        hash: FieldElement,
    ) -> Result<MaybePendingTransactionReceipt, JsonRpcError> {
        self.client
            .get_transaction_receipt(hash)
            .await
            .map_err(|e| Self::map_to_rpc_error("get_transaction_receipt", e))
    }

    /// Get the transaction given a block ID and index.
    ///
    /// # Arguments
    ///
    /// * `block_id`: The block identifier.
    /// * `index`: Transaction index.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `Transaction` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn get_transaction_by_block_id_and_index(
        &self,
        block_id: &BlockId,
        index: u64,
    ) -> Result<Transaction, JsonRpcError> {
        self.client
            .get_transaction_by_block_id_and_index(block_id, index)
            .await
            .map_err(|e| Self::map_to_rpc_error("get_transaction_by_block_id_and_index", e))
    }

    /// Get the pending transactions.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `Transaction` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn pending_transactions(&self) -> Result<Vec<Transaction>, JsonRpcError> {
        self.client
            .pending_transactions()
            .await
            .map_err(|e| Self::map_to_rpc_error("pending_transactions", e))
    }

    /// Get a contract storage proof.
    ///
    /// # Arguments
    ///
    /// * `contract_address`: Address of the contract.
    /// * `keys`: Storage slots of the contract keys that need a proof.
    /// * `block_id`: ID of the block the proof is needed for.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `GetProofOutput` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn get_contract_storage_proof(
        &self,
        contract_address: FieldElement,
        keys: Vec<FieldElement>,
        block_id: &BlockId,
    ) -> Result<GetProofOutput, JsonRpcError> {
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
            .map_err(|e| {
                let error = JsonRpcError::try_from(JsonRpcClientErrorWrapper::from(e));
                match error {
                    Ok(rpc_error) => rpc_error,
                    Err(unknown_error) => JsonRpcError {
                        code: 520,
                        message: "[add_declare_transaction] ".to_owned()
                            + &unknown_error.to_string(),
                    },
                }
            })
    }

    /// Get the transactions of a given block.
    ///
    /// # Arguments
    ///
    /// * `block_id`: The block identifier.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `MaybePendingBlockWithTxs` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn get_block_with_txs(
        &self,
        block_id: &BlockId,
    ) -> Result<MaybePendingBlockWithTxs, JsonRpcError> {
        self.client
            .get_block_with_txs(block_id)
            .await
            .map_err(|e| Self::map_to_rpc_error("get_block_with_txs", e))
    }

    /// Get the number of transactions in a block given a block ID.
    ///
    /// # Arguments
    ///
    /// * `block_id`: The block identifier.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the number of transactions (`u64`) if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn get_block_transaction_count(&self, block_id: &BlockId) -> Result<u64, JsonRpcError> {
        self.client
            .get_block_transaction_count(block_id)
            .await
            .map_err(|e| Self::map_to_rpc_error("get_block_transaction_count", e))
    }

    /// Add a Declare transaction.
    ///
    /// # Arguments
    ///
    /// * `declare_transaction`: Transaction data.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `DeclareTransactionResult` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// ## Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    async fn add_declare_transaction(
        &self,
        declare_transaction: &BroadcastedDeclareTransaction,
    ) -> Result<DeclareTransactionResult, JsonRpcError> {
        self.client
            .add_declare_transaction(declare_transaction)
            .await
            .map_err(|e| Self::map_to_rpc_error("add_declare_transaction", e))
    }
}
