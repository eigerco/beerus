pub mod api;
use crate::api::{BeerusApiError, BeerusApiServer};
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
        BlockHashAndNumber, ContractClass, MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs,
        MaybePendingTransactionReceipt, StateUpdate, SyncStatusType, Transaction,
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
    async fn l2_to_l1_messages(&self, msg_hash: U256) -> Result<U256, Error> {
        Ok(self
            .beerus
            .starknet_l2_to_l1_messages(msg_hash)
            .await
            .unwrap())
    }

    async fn chain_id(&self) -> Result<String, Error> {
        let chain_id = self
            .beerus
            .starknet_lightclient
            .chain_id()
            .await
            .unwrap()
            .to_string();

        Ok(chain_id)
    }

    async fn block_number(&self) -> Result<u64, Error> {
        let block_number = self
            .beerus
            .starknet_lightclient
            .block_number()
            .await
            .unwrap();

        Ok(block_number)
    }

    async fn get_block_transaction_count(
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

    async fn block_hash_and_number(&self) -> Result<BlockHashAndNumber, Error> {
        Ok(self
            .beerus
            .starknet_lightclient
            .block_hash_and_number()
            .await
            .unwrap())
    }

    async fn get_class_at(
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

    async fn get_block_with_tx_hashes(
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

    async fn get_transaction_by_block_id_and_index(
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

    async fn get_block_with_txs(
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

    async fn get_state_update(
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

    async fn syncing(&self) -> Result<SyncStatusType, Error> {
        let sync_status_type = self.beerus.starknet_lightclient.syncing().await.unwrap();
        Ok(sync_status_type)
    }

    async fn l1_to_l2_messages(&self, msg_hash: U256) -> Result<U256, Error> {
        Ok(self
            .beerus
            .starknet_l1_to_l2_messages(msg_hash)
            .await
            .unwrap())
    }

    async fn l1_to_l2_message_nonce(&self) -> Result<U256, Error> {
        let nonce = self.beerus.starknet_l1_to_l2_message_nonce().await.unwrap();
        Ok(nonce)
    }

    async fn l1_to_l2_message_cancellations(&self, msg_hash: U256) -> Result<U256, Error> {
        Ok(self
            .beerus
            .starknet_l1_to_l2_message_cancellations(msg_hash)
            .await
            .unwrap())
    }

    async fn get_transaction_receipt(
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

    async fn get_class_hash(
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
