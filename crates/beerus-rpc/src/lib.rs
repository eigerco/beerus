pub mod api;

use crate::api::{
    BeerusApiError, BeerusApiErrorWithMessage, BeerusApiServer, BLOCK_NOT_FOUND,
    CLASS_HASH_NOT_FOUND, CONTRACT_ERROR, CONTRACT_NOT_FOUND, INVALID_CALL_DATA,
    INVALID_CONTINUATION_TOKEN, INVALID_TXN_INDEX, PAGE_SIZE_TOO_BIG, TOO_MANY_KEYS_IN_FILTER,
};
use beerus_core::lightclient::starknet::storage_proof::GetProofOutput;

use jsonrpsee::{
    core::{async_trait, Error},
    server::{ServerBuilder, ServerHandle},
    types::error::CallError,
};

use beerus_core::lightclient::beerus::BeerusLightClient;
use ethers::types::U256;
use starknet::{
    core::types::FieldElement,
    providers::jsonrpc::models::{
        BlockHashAndNumber, BlockId, BroadcastedDeclareTransaction,
        BroadcastedDeclareTransactionV1, BroadcastedDeployTransaction,
        BroadcastedInvokeTransaction, BroadcastedTransaction, ContractClass,
        DeclareTransactionResult, DeployTransactionResult, EventFilter, EventsPage, FeeEstimate,
        FunctionCall, InvokeTransactionResult, MaybePendingBlockWithTxHashes,
        MaybePendingBlockWithTxs, MaybePendingTransactionReceipt, StateUpdate, SyncStatusType,
        Transaction,
    },
};
use std::net::SocketAddr;
use std::str::FromStr;

pub struct BeerusRpc {
    beerus: BeerusLightClient,
}

impl BeerusRpc {
    pub fn new(beerus: BeerusLightClient) -> Self {
        Self { beerus }
    }

    pub async fn run(self) -> Result<(SocketAddr, ServerHandle), Error> {
        let server = ServerBuilder::new()
            .build(self.beerus.config.beerus_rpc_address.unwrap())
            .await
            .map_err(|_| Error::from(BeerusApiError::InternalServerError))?;

        let addr = server.local_addr()?;
        let handle = server.start(self.into_rpc())?;

        Ok((addr, handle))
    }
}

#[async_trait]
impl BeerusApiServer for BeerusRpc {
    // Starknet functions
    async fn l2_to_l1_messages(&self, msg_hash: U256) -> Result<U256, Error> {
        self.beerus
            .starknet_l2_to_l1_messages(msg_hash)
            .await
            .map_err(|_| Error::from(BeerusApiError::ContractError))
    }

    async fn chain_id(&self) -> Result<String, Error> {
        let chain_id = self
            .beerus
            .starknet_lightclient
            .chain_id()
            .await
            .map_err(|_| Error::from(BeerusApiError::ContractError))?;

        Ok(chain_id.to_string())
    }

    async fn block_number(&self) -> Result<u64, Error> {
        self.beerus
            .starknet_lightclient
            .block_number()
            .await
            .map_err(|_| Error::from(BeerusApiError::BlockNotFound))
    }

    async fn get_nonce(&self, contract_address: String) -> Result<FieldElement, Error> {
        let contract_address = FieldElement::from_hex_be(&contract_address)
            .map_err(|_| invalid_call_data("contract_address"))?;

        self.beerus
            .starknet_get_nonce(contract_address)
            .await
            .map_err(|e| {
                let error_type: String = e.to_string();
                match error_type.as_str() {
                    BLOCK_NOT_FOUND => Error::from(BeerusApiError::BlockNotFound),
                    CONTRACT_NOT_FOUND => Error::from(BeerusApiError::ContractNotFound),
                    _ => Error::from(BeerusApiError::ContractError),
                }
            })
    }

    async fn get_transaction_by_hash(&self, tx_hash: &str) -> Result<Transaction, Error> {
        let tx_hash_felt =
            FieldElement::from_hex_be(tx_hash).map_err(|_| invalid_call_data("tx_hash"))?;

        self.beerus
            .starknet_lightclient
            .get_transaction_by_hash(tx_hash_felt)
            .await
            .map_err(|_| Error::from(BeerusApiError::TxnHashNotFound))
    }

    async fn get_block_transaction_count(&self, block_id: BlockId) -> Result<u64, Error> {
        self.beerus
            .starknet_lightclient
            .get_block_transaction_count(&block_id)
            .await
            .map_err(|_| Error::from(BeerusApiError::BlockNotFound))
    }

    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, Error> {
        self.beerus
            .starknet_lightclient
            .block_hash_and_number()
            .await
            .map_err(|_| Error::from(BeerusApiError::NoBlocks))
    }

    async fn get_contract_storage_proof(
        &self,
        block_id: BlockId,
        contract_address: String,
        keys: Vec<String>,
    ) -> Result<GetProofOutput, Error> {
        let contract_address = FieldElement::from_str(&contract_address)
            .map_err(|_| invalid_call_data("contract_address"))?;

        let keys: Result<Vec<FieldElement>, _> =
            keys.iter().map(|k| FieldElement::from_str(k)).collect();

        self.beerus
            .starknet_lightclient
            .get_contract_storage_proof(
                contract_address,
                keys.map_err(|_| invalid_call_data("keys"))?,
                &block_id,
            )
            .await
            .map_err(|_| Error::from(BeerusApiError::ContractError))
    }

    async fn get_class_at(
        &self,
        block_id: BlockId,
        contract_address: String,
    ) -> Result<ContractClass, Error> {
        let contract_address = FieldElement::from_str(&contract_address)
            .map_err(|_| invalid_call_data("contract_address"))?;

        self.beerus
            .starknet_lightclient
            .get_class_at(&block_id, contract_address)
            .await
            .map_err(|e| {
                let error_type: String = e.to_string();
                match error_type.as_str() {
                    BLOCK_NOT_FOUND => Error::from(BeerusApiError::BlockNotFound),
                    CONTRACT_NOT_FOUND => Error::from(BeerusApiError::ContractNotFound),
                    _ => Error::from(BeerusApiError::ContractError),
                }
            })
    }

    async fn add_invoke_transaction(
        &self,
        invoke_transaction: BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult, Error> {
        self.beerus
            .starknet_lightclient
            .add_invoke_transaction(&invoke_transaction)
            .await
            .map_err(|_| Error::from(BeerusApiError::InvalidCallData))
    }

    async fn get_block_with_tx_hashes(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes, Error> {
        self.beerus
            .starknet_lightclient
            .get_block_with_tx_hashes(&block_id)
            .await
            .map_err(|_| Error::from(BeerusApiError::BlockNotFound))
    }

    async fn get_transaction_by_block_id_and_index(
        &self,
        block_id: BlockId,
        index: &str,
    ) -> Result<Transaction, Error> {
        let index = u64::from_str(index).map_err(|_| invalid_call_data("index"))?;

        self.beerus
            .starknet_lightclient
            .get_transaction_by_block_id_and_index(&block_id, index)
            .await
            .map_err(|e| {
                let error_type: String = e.to_string();
                match error_type.as_str() {
                    BLOCK_NOT_FOUND => Error::from(BeerusApiError::BlockNotFound),
                    INVALID_TXN_INDEX => Error::from(BeerusApiError::InvalidTxnIndex),
                    _ => Error::from(BeerusApiError::ContractError),
                }
            })
    }

    async fn get_block_with_txs(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxs, Error> {
        self.beerus
            .starknet_lightclient
            .get_block_with_txs(&block_id)
            .await
            .map_err(|_| Error::from(BeerusApiError::BlockNotFound))
    }

    async fn get_state_update(&self, block_id: BlockId) -> Result<StateUpdate, Error> {
        self.beerus
            .starknet_lightclient
            .get_state_update(&block_id)
            .await
            .map_err(|_| Error::from(BeerusApiError::BlockNotFound))
    }

    async fn syncing(&self) -> Result<SyncStatusType, Error> {
        self.beerus
            .starknet_lightclient
            .syncing()
            .await
            .map_err(|_| Error::from(BeerusApiError::ContractError))
    }

    async fn l1_to_l2_messages(&self, msg_hash: U256) -> Result<U256, Error> {
        self.beerus
            .starknet_l1_to_l2_messages(msg_hash)
            .await
            .map_err(|_| Error::from(BeerusApiError::ContractError))
    }

    async fn l1_to_l2_message_nonce(&self) -> Result<U256, Error> {
        self.beerus
            .starknet_l1_to_l2_message_nonce()
            .await
            .map_err(|_| Error::from(BeerusApiError::ContractError))
    }

    async fn l1_to_l2_message_cancellations(&self, msg_hash: U256) -> Result<U256, Error> {
        self.beerus
            .starknet_l1_to_l2_message_cancellations(msg_hash)
            .await
            .map_err(|_| Error::from(BeerusApiError::ContractError))
    }

    async fn get_transaction_receipt(
        &self,
        tx_hash: String,
    ) -> Result<MaybePendingTransactionReceipt, Error> {
        let tx_hash_felt =
            FieldElement::from_hex_be(&tx_hash).map_err(|_| invalid_call_data("tx_hash"))?;

        self.beerus
            .starknet_lightclient
            .get_transaction_receipt(tx_hash_felt)
            .await
            .map_err(|_| Error::from(BeerusApiError::TxnHashNotFound))
    }

    async fn get_class_hash_at(
        &self,
        block_id: BlockId,
        contract_address: String,
    ) -> Result<FieldElement, Error> {
        let contract_address = FieldElement::from_str(&contract_address)
            .map_err(|_| invalid_call_data("contract_address"))?;

        self.beerus
            .starknet_lightclient
            .get_class_hash_at(&block_id, contract_address)
            .await
            .map_err(|e| {
                let error_type: String = e.to_string();
                match error_type.as_str() {
                    BLOCK_NOT_FOUND => Error::from(BeerusApiError::BlockNotFound),
                    CONTRACT_NOT_FOUND => Error::from(BeerusApiError::ContractNotFound),
                    _ => Error::from(BeerusApiError::ContractError),
                }
            })
    }

    async fn get_class(
        &self,
        block_id: BlockId,
        class_hash: String,
    ) -> Result<ContractClass, Error> {
        let class_hash =
            FieldElement::from_str(&class_hash).map_err(|_| invalid_call_data("class_hash"))?;

        self.beerus
            .starknet_lightclient
            .get_class(&block_id, class_hash)
            .await
            .map_err(|e| {
                let error_type: String = e.to_string();
                match error_type.as_str() {
                    BLOCK_NOT_FOUND => Error::from(BeerusApiError::BlockNotFound),
                    CLASS_HASH_NOT_FOUND => Error::from(BeerusApiError::ClassHashNotFound),
                    _ => Error::from(BeerusApiError::ContractError),
                }
            })
    }

    async fn add_deploy_account_transaction(
        &self,
        contract_class: String,
        version: String,
        contract_address_salt: String,
        constructor_calldata: Vec<String>,
    ) -> Result<DeployTransactionResult, Error> {
        let contract_class_bytes = contract_class.as_bytes();
        let contract_class = serde_json::from_slice(contract_class_bytes)
            .map_err(|_| invalid_call_data("contract_class"))?;

        let version: u64 = version.parse().map_err(|_| invalid_call_data("version"))?;

        let contract_address_salt: FieldElement = FieldElement::from_str(&contract_address_salt)
            .map_err(|_| invalid_call_data("contract_address_salt"))?;

        let constructor_calldata = constructor_calldata
            .iter()
            .map(|x| {
                FieldElement::from_str(x)
                    .map_err(|_| invalid_call_data("constructor_calldata"))
                    .unwrap()
            })
            .collect();

        let deploy_transaction = BroadcastedDeployTransaction {
            contract_class,
            version,
            contract_address_salt,
            constructor_calldata,
        };

        self.beerus
            .starknet_lightclient
            .add_deploy_transaction(&deploy_transaction)
            .await
            .map_err(|e| {
                let error_type: String = e.to_string();
                match error_type.as_str() {
                    CLASS_HASH_NOT_FOUND => Error::from(BeerusApiError::ClassHashNotFound),
                    _ => Error::from(BeerusApiError::ContractError),
                }
            })
    }

    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, Error> {
        self.beerus
            .starknet_lightclient
            .get_events(filter, continuation_token, chunk_size)
            .await
            .map_err(|e| {
                let error_type: String = e.to_string();
                match error_type.as_str() {
                    PAGE_SIZE_TOO_BIG => Error::from(BeerusApiError::PageSizeTooBig),
                    INVALID_CONTINUATION_TOKEN => {
                        Error::from(BeerusApiError::InvalidContinuationToken)
                    }
                    BLOCK_NOT_FOUND => Error::from(BeerusApiError::BlockNotFound),
                    TOO_MANY_KEYS_IN_FILTER => Error::from(BeerusApiError::TooManyKeysInFilter),
                    _ => Error::from(BeerusApiError::ContractError),
                }
            })
    }

    async fn add_declare_transaction(
        &self,
        version: String,
        max_fee: String,
        signature: Vec<String>,
        nonce: String,
        contract_class: String,
        sender_address: String,
    ) -> Result<DeclareTransactionResult, Error> {
        let max_fee: FieldElement =
            FieldElement::from_str(&max_fee).map_err(|_| invalid_call_data("max_fee"))?;

        let _version: u64 = version.parse().map_err(|_| invalid_call_data("version"))?;

        let signature = signature
            .iter()
            .map(|x| {
                FieldElement::from_str(x)
                    .map_err(|_| invalid_call_data("signature"))
                    .unwrap()
            })
            .collect();

        let nonce: FieldElement =
            FieldElement::from_str(&nonce).map_err(|_| invalid_call_data("nonce"))?;

        let contract_class_bytes = contract_class.as_bytes();

        let contract_class = serde_json::from_slice(contract_class_bytes)
            .map_err(|_| invalid_call_data("contract_class"))?;

        let sender_address: FieldElement = FieldElement::from_str(&sender_address)
            .map_err(|_| invalid_call_data("sender_address"))?;

        let declare_transaction =
            BroadcastedDeclareTransaction::V1(BroadcastedDeclareTransactionV1 {
                max_fee,
                signature,
                nonce,
                contract_class,
                sender_address,
            });

        self.beerus
            .starknet_lightclient
            .add_declare_transaction(&declare_transaction)
            .await
            .map_err(|_| Error::from(BeerusApiError::InvalidContractClass))
    }

    async fn pending_transactions(&self) -> Result<Vec<Transaction>, Error> {
        self.beerus
            .starknet_lightclient
            .pending_transactions()
            .await
            .map_err(|_| Error::from(BeerusApiError::FailedToFetchPendingTransactions))
    }

    async fn estimate_fee(
        &self,
        block_id: BlockId,
        broadcasted_transaction: String,
    ) -> Result<FeeEstimate, Error> {
        let broadcasted_transaction: BroadcastedTransaction =
            serde_json::from_str(&broadcasted_transaction).map_err(|e| {
                Error::Call(CallError::InvalidParams(anyhow::anyhow!(e.to_string())))
            })?;

        let estimate_fee = self
            .beerus
            .starknet_lightclient
            .estimate_fee(broadcasted_transaction, &block_id)
            .await
            .map_err(|e| Error::Call(CallError::Failed(anyhow::anyhow!(e.to_string()))))?;
        Ok(estimate_fee)
    }

    async fn call(
        &self,
        request: FunctionCall,
        block_number: u64,
    ) -> Result<Vec<FieldElement>, Error> {
        self.beerus
            .starknet_lightclient
            .call(request, block_number)
            .await
            .map_err(|e| {
                let error_type: String = e.to_string();
                match error_type.as_str() {
                    CONTRACT_NOT_FOUND => Error::from(BeerusApiError::ContractNotFound),
                    CONTRACT_ERROR => Error::from(BeerusApiError::ContractError),
                    BLOCK_NOT_FOUND => Error::from(BeerusApiError::BlockNotFound),
                    _ => Error::from(BeerusApiError::ContractError),
                }
            })
    }

    async fn get_storage_at(
        &self,
        contract_address: String,
        key: String,
    ) -> Result<FieldElement, Error> {
        let contract_address = FieldElement::from_hex_be(&contract_address)
            .map_err(|_| invalid_call_data("contract_class"))?;
        let key = FieldElement::from_hex_be(&key).map_err(|_| invalid_call_data("key"))?;

        self.beerus
            .starknet_get_storage_at(contract_address, key)
            .await
            .map_err(|_| Error::from(BeerusApiError::ContractError))
    }
}

fn invalid_call_data(param: &str) -> Error {
    let message = INVALID_CALL_DATA.to_string() + ": cannot parse '" + param + "'";
    Error::from(BeerusApiErrorWithMessage {
        error: BeerusApiError::InvalidCallData,
        message,
    })
}
