pub mod api;
pub mod models;

use crate::api::{BeerusApiError, BeerusApiServer};
use crate::models::EventFilter;
use jsonrpsee::{
    core::{async_trait, Error},
    server::{ServerBuilder, ServerHandle},
    types::error::CallError,
};

use beerus_core::lightclient::beerus::BeerusLightClient;
use beerus_core::starknet_helper::block_id_string_to_block_id_type;
use ethers::types::U256;
use starknet::{
    core::types::FieldElement,
    providers::jsonrpc::models::{
        BlockHashAndNumber, BroadcastedDeclareTransaction, BroadcastedDeployTransaction,
        ContractClass, DeclareTransactionResult, DeployTransactionResult, EventsPage,
        MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs, MaybePendingTransactionReceipt,
        StateUpdate, SyncStatusType, Transaction,
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
        let server = ServerBuilder::default()
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
    // Ethereum functions
    async fn ethereum_block_number(&self) -> Result<u64, Error> {
        self.beerus
            .ethereum_lightclient
            .read()
            .await
            .get_block_number()
            .await
            .map_err(|_| Error::from(BeerusApiError::BlockNotFound))
    }

    async fn ethereum_chain_id(&self) -> Result<u64, Error> {
        self.beerus
            .ethereum_lightclient
            .read()
            .await
            .get_chain_id()
            .await
            .map_err(|_| Error::from(BeerusApiError::InternalServerError))
    }

    // Starknet functions
    async fn starknet_l2_to_l1_messages(&self, msg_hash: U256) -> Result<U256, Error> {
        Ok(self
            .beerus
            .starknet_l2_to_l1_messages(msg_hash)
            .await
            .unwrap())
    }

    async fn starknet_chain_id(&self) -> Result<String, Error> {
        let chain_id = self
            .beerus
            .starknet_lightclient
            .chain_id()
            .await
            .unwrap()
            .to_string();

        Ok(chain_id)
    }

    async fn starknet_block_number(&self) -> Result<u64, Error> {
        self.beerus
            .starknet_lightclient
            .block_number()
            .await
            .map_err(|_| Error::from(BeerusApiError::BlockNotFound))
    }

    async fn starknet_get_nonce(&self, contract_address: String) -> Result<String, Error> {
        let contract_address = FieldElement::from_hex_be(&contract_address).unwrap();
        let nonce = self
            .beerus
            .starknet_get_nonce(contract_address)
            .await
            .unwrap()
            .to_string();
        Ok(nonce)
    }

    async fn starknet_get_block_transaction_count(
        &self,
        block_id_type: String,
        block_id: String,
    ) -> Result<u64, Error> {
        let block_id = block_id_string_to_block_id_type(&block_id_type, &block_id).unwrap();
        let block_transaction_count = self
            .beerus
            .starknet_lightclient
            .get_block_transaction_count(&block_id)
            .await
            .unwrap();

        Ok(block_transaction_count)
    }

    async fn starknet_block_hash_and_number(&self) -> Result<BlockHashAndNumber, Error> {
        Ok(self
            .beerus
            .starknet_lightclient
            .block_hash_and_number()
            .await
            .unwrap())
    }

    async fn starknet_get_class_at(
        &self,
        block_id_type: String,
        block_id: String,
        contract_address: String,
    ) -> Result<ContractClass, Error> {
        let block_id = block_id_string_to_block_id_type(&block_id_type, &block_id).unwrap();
        let contract_address = FieldElement::from_str(&contract_address).unwrap();
        Ok(self
            .beerus
            .starknet_lightclient
            .get_class_at(&block_id, contract_address)
            .await
            .unwrap())
    }

    async fn starknet_get_block_with_tx_hashes(
        &self,
        block_id_type: String,
        block_id: String,
    ) -> Result<MaybePendingBlockWithTxHashes, Error> {
        let block_id = block_id_string_to_block_id_type(&block_id_type, &block_id).unwrap();
        self.beerus
            .starknet_lightclient
            .get_block_with_tx_hashes(&block_id)
            .await
            .map_err(|_| Error::from(BeerusApiError::BlockNotFound))
    }

    async fn starknet_get_transaction_by_block_id_and_index(
        &self,
        block_id_type: &str,
        block_id: &str,
        index: &str,
    ) -> Result<Transaction, Error> {
        let block_id =
            beerus_core::starknet_helper::block_id_string_to_block_id_type(block_id_type, block_id)
                .map_err(|e| {
                    Error::Call(CallError::InvalidParams(anyhow::anyhow!(e.to_string())))
                })?;
        let index = u64::from_str(index)
            .map_err(|e| Error::Call(CallError::InvalidParams(anyhow::anyhow!(e.to_string()))))?;
        let result = self
            .beerus
            .starknet_lightclient
            .get_transaction_by_block_id_and_index(&block_id, index)
            .await
            .map_err(|e| Error::Call(CallError::Failed(anyhow::anyhow!(e.to_string()))))?;
        Ok(result)
    }

    async fn starknet_get_block_with_txs(
        &self,
        block_id_type: &str,
        block_id: &str,
    ) -> Result<MaybePendingBlockWithTxs, Error> {
        let block_id =
            beerus_core::starknet_helper::block_id_string_to_block_id_type(block_id_type, block_id)
                .map_err(|e| {
                    Error::Call(CallError::InvalidParams(anyhow::anyhow!(e.to_string())))
                })?;
        let result = self
            .beerus
            .starknet_lightclient
            .get_block_with_txs(&block_id)
            .await
            .map_err(|e| Error::Call(CallError::Failed(anyhow::anyhow!(e.to_string()))))?;
        Ok(result)
    }

    async fn starknet_get_state_update(
        &self,
        block_id_type: String,
        block_id: String,
    ) -> Result<StateUpdate, Error> {
        let block_id = block_id_string_to_block_id_type(&block_id_type, &block_id).unwrap();
        Ok(self
            .beerus
            .starknet_lightclient
            .get_state_update(&block_id)
            .await
            .unwrap())
    }

    async fn starknet_syncing(&self) -> Result<SyncStatusType, Error> {
        let sync_status_type = self.beerus.starknet_lightclient.syncing().await.unwrap();
        Ok(sync_status_type)
    }

    async fn starknet_l1_to_l2_messages(&self, msg_hash: U256) -> Result<U256, Error> {
        Ok(self
            .beerus
            .starknet_l1_to_l2_messages(msg_hash)
            .await
            .unwrap())
    }

    async fn starknet_l1_to_l2_message_nonce(&self) -> Result<U256, Error> {
        let nonce = self.beerus.starknet_l1_to_l2_message_nonce().await.unwrap();
        Ok(nonce)
    }

    async fn starknet_l1_to_l2_message_cancellations(&self, msg_hash: U256) -> Result<U256, Error> {
        Ok(self
            .beerus
            .starknet_l1_to_l2_message_cancellations(msg_hash)
            .await
            .unwrap())
    }

    async fn starknet_get_transaction_receipt(
        &self,
        tx_hash: String,
    ) -> Result<MaybePendingTransactionReceipt, Error> {
        let tx_hash_felt = FieldElement::from_hex_be(&tx_hash).unwrap();
        Ok(self
            .beerus
            .starknet_lightclient
            .get_transaction_receipt(tx_hash_felt)
            .await
            .unwrap())
    }

    async fn starknet_get_class_hash(
        &self,
        block_id_type: String,
        block_id: String,
        contract_address: String,
    ) -> Result<FieldElement, Error> {
        let block_id = block_id_string_to_block_id_type(&block_id_type, &block_id).unwrap();
        let contract_address = FieldElement::from_str(&contract_address).unwrap();

        Ok(self
            .beerus
            .starknet_lightclient
            .get_class_hash_at(&block_id, contract_address)
            .await
            .unwrap())
    }

    async fn starknet_get_class(
        &self,
        block_id_type: String,
        block_id: String,
        class_hash: String,
    ) -> Result<ContractClass, Error> {
        let block_id = block_id_string_to_block_id_type(&block_id_type, &block_id)
            .map_err(|e| Error::Call(CallError::InvalidParams(anyhow::anyhow!(e.to_string()))))?;
        let class_hash = FieldElement::from_str(&class_hash)
            .map_err(|e| Error::Call(CallError::InvalidParams(anyhow::anyhow!(e.to_string()))))?;
        let result = self
            .beerus
            .starknet_lightclient
            .get_class(&block_id, class_hash)
            .await
            .map_err(|e| Error::Call(CallError::Failed(anyhow::anyhow!(e.to_string()))))?;

        Ok(result)
    }

    async fn starknet_add_deploy_transaction(
        &self,
        contract_class: String,
        version: String,
        contract_address_salt: String,
        constructor_calldata: Vec<String>,
    ) -> Result<DeployTransactionResult, Error> {
        let contract_class_bytes = contract_class.as_bytes();
        let contract_class = serde_json::from_slice(contract_class_bytes).unwrap();
        let version: u64 = version.parse().unwrap();
        let contract_address_salt: FieldElement =
            FieldElement::from_str(&contract_address_salt).unwrap();
        let constructor_calldata = constructor_calldata
            .iter()
            .map(|x| FieldElement::from_str(x).unwrap())
            .collect();
        let deploy_transaction = BroadcastedDeployTransaction {
            contract_class,
            version,
            contract_address_salt,
            constructor_calldata,
        };
        let result = self
            .beerus
            .starknet_lightclient
            .add_deploy_transaction(&deploy_transaction)
            .await
            .map_err(|e| Error::Call(CallError::Failed(anyhow::anyhow!(e.to_string()))))?;

        Ok(result)
    }

    async fn get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, Error> {
        let filter = filter.to_starknet_event_filter();
        Ok(self
            .beerus
            .starknet_lightclient
            .get_events(filter, continuation_token, chunk_size)
            .await
            .unwrap())
    }

    async fn starknet_add_declare_transaction(
        &self,
        version: String,
        max_fee: String,
        signature: Vec<String>,
        nonce: String,
        contract_class: String,
        sender_address: String,
    ) -> Result<DeclareTransactionResult, Error> {
        let max_fee: FieldElement = FieldElement::from_str(&max_fee).unwrap();
        let version: u64 = version.parse().unwrap();
        let signature = signature
            .iter()
            .map(|x| FieldElement::from_str(x).unwrap())
            .collect();
        let nonce: FieldElement = FieldElement::from_str(&nonce).unwrap();

        let contract_class_bytes = contract_class.as_bytes();
        let contract_class = serde_json::from_slice(contract_class_bytes)?;
        let sender_address: FieldElement = FieldElement::from_str(&sender_address).unwrap();

        let declare_transaction = BroadcastedDeclareTransaction {
            max_fee,
            version,
            signature,
            nonce,
            contract_class,
            sender_address,
        };

        Ok(self
            .beerus
            .starknet_lightclient
            .add_declare_transaction(&declare_transaction)
            .await
            .unwrap())
    }

    async fn starknet_pending_transactions(&self) -> Result<Vec<Transaction>, Error> {
        let transactions_result = self
            .beerus
            .starknet_lightclient
            .pending_transactions()
            .await
            .map_err(|_| Error::from(BeerusApiError::FailedToFetchPendingTransactions));
        Ok(transactions_result.unwrap())
    }
}
