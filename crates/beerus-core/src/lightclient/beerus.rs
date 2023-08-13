#[cfg(feature = "std")]
use std::{str::FromStr, thread, time};

#[cfg(not(feature = "std"))]
use gloo_timers::callback::Interval;
#[cfg(not(feature = "std"))]
use wasm_bindgen_futures::spawn_local;

use tokio::sync::{Mutex, RwLock};

#[cfg(not(feature = "std"))]
use core::str::FromStr;

use crate::stdlib::boxed::Box;
use crate::stdlib::string::{String, ToString};
use crate::stdlib::vec::Vec;
use crate::stdlib::{collections::BTreeMap, sync::Arc};

use super::{ethereum::EthereumLightClient, starknet::StarkNetLightClient};
use crate::{
    config::Config,
    ethers_helper,
    lightclient::{
        ethereum::helios_lightclient::HeliosLightClient, starknet::StarkNetLightClientImpl,
    },
};
use ethabi::Uint as U256;
use ethers::{abi::Abi, types::H160};
use eyre::Result as EyreResult;
use helios::types::{BlockTag, CallOpts};
#[cfg(feature = "std")]
use log::{debug, error, info, warn};
use starknet::core::types::{
    BlockHashAndNumber, BlockId, BlockStatus, BlockTag as StarknetBlockTag, BlockWithTxHashes,
    BlockWithTxs, BroadcastedTransaction, DeclareTransaction, DeployAccountTransaction,
    DeployTransaction, FeeEstimate, FieldElement, FunctionCall, InvokeTransaction,
    L1HandlerTransaction, MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs,
    MaybePendingTransactionReceipt, Transaction,
};
use starknet::providers::jsonrpc::JsonRpcError;

/// Enum representing the different synchronization status of the light client.
#[derive(Debug, Clone, PartialEq)]
pub enum SyncStatus {
    NotSynced,
    Syncing,
    Synced,
}

#[derive(Clone, Debug)]
pub struct NodeData {
    pub block_number: u64,
    pub state_root: String,
    pub payload: BTreeMap<u64, BlockWithTxs>,
}

impl NodeData {
    pub fn new() -> Self {
        NodeData {
            block_number: 0,
            state_root: "".to_string(),
            payload: BTreeMap::new(),
        }
    }
}

impl Default for NodeData {
    fn default() -> Self {
        Self::new()
    }
}

/// Beerus Light Client service.
pub struct BeerusLightClient {
    /// Global configuration.
    pub config: Config,
    /// Ethereum light client.
    pub ethereum_lightclient: Arc<Mutex<Box<dyn EthereumLightClient>>>,
    /// StarkNet light client.
    pub starknet_lightclient: Arc<Box<dyn StarkNetLightClient>>,
    /// Sync status.
    pub sync_status: SyncStatus,
    /// StarkNet core ABI.
    pub starknet_core_abi: Abi,
    /// StarkNet core contract address.
    pub starknet_core_contract_address: H160,
    /// Payload data
    pub node: Arc<RwLock<NodeData>>,
}

impl BeerusLightClient {
    /// Create a new Beerus Light Client service.
    pub async fn new(config: Config) -> EyreResult<Self> {
        info!("creating Ethereum(Helios) lightclient...");
        let ethereum_lightclient_raw = HeliosLightClient::new(config.clone()).await?;

        info!("creating Starknet lightclient...");
        let starknet_lightclient_raw = StarkNetLightClientImpl::new(&config)?;

        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_raw),
            Box::new(starknet_lightclient_raw),
        );
        Ok(beerus)
    }

    /// Create a new Beerus Light Client service with the provided
    /// custom Ethereum and Starknet clients
    pub fn new_from_clients(
        config: Config,
        ethereum_lightclient_raw: Box<dyn EthereumLightClient>,
        starknet_lightclient_raw: Box<dyn StarkNetLightClient>,
        //TODO: Check if we should just have &str as arguments
    ) -> Self {
        // Create a new Ethereum light client.
        let ethereum_lightclient = Arc::new(Mutex::new(ethereum_lightclient_raw));
        // Create a new StarkNet light client.
        let starknet_lightclient = Arc::new(starknet_lightclient_raw);
        let starknet_core_abi = include_str!("../resources/starknet_core_abi.json");
        // Deserialize the StarkNet core ABI.
        // For now we assume that the ABI is valid and that the deserialization will never fail.
        let starknet_core_abi: Abi = serde_json::from_str(starknet_core_abi).unwrap();
        let starknet_core_contract_address = config.starknet_core_contract_address;
        let node_raw = NodeData::new();
        let node = Arc::new(RwLock::new(node_raw));

        Self {
            config,
            ethereum_lightclient,
            starknet_lightclient,
            sync_status: SyncStatus::NotSynced,
            starknet_core_abi,
            starknet_core_contract_address,
            node,
        }
    }

    /// Start Beerus light client and synchronize with Ethereum and StarkNet.
    #[cfg(feature = "std")]

    pub async fn start(&mut self) -> EyreResult<()> {
        if let SyncStatus::NotSynced = self.sync_status {
            // Start the Ethereum light client.
            self.ethereum_lightclient.lock().await.start().await?;
            // Start the StarkNet light client.
            self.starknet_lightclient.start().await?;
            self.sync_status = SyncStatus::Synced;

            let ethereum_clone = self.ethereum_lightclient.clone();
            let starknet_clone = self.starknet_lightclient.clone();
            let node_clone = self.node.clone();
            let poll_interval_secs = self.config.get_poll_interval();

            // Define function that will loop
            let task = async move {
                loop {
                    let state_root = ethereum_clone
                        .lock()
                        .await
                        .starknet_state_root()
                        .await
                        .unwrap();

                    let last_proven_block = ethereum_clone
                        .lock()
                        .await
                        .starknet_last_proven_block()
                        .await
                        .unwrap();

                    // TODO: these logs don't get caught by the main thread
                    info!("State Root: {state_root}");
                    info!("Block Number: {last_proven_block}");

                    match starknet_clone
                        .get_block_with_txs(&BlockId::Tag(StarknetBlockTag::Latest))
                        .await
                    {
                        Ok(block) => {
                            debug!("block: {block:?}");
                            let mut data = node_clone.write().await;
                            match block {
                                MaybePendingBlockWithTxs::Block(block) => {
                                    // if block.block_number > data.block_number && block.block_number == last_proven_block
                                    if block.block_number > data.block_number
                                        && 0 < block.block_number
                                    {
                                        data.block_number = block.block_number;
                                        data.state_root = block.new_root.to_string();
                                        data.payload.insert(block.block_number, block);

                                        info!("New Block Added to Payload:");
                                        info!("Block Number {:?}", &data.block_number);
                                        info!("Block Root {:?}", &data.state_root);
                                    }
                                }
                                MaybePendingBlockWithTxs::PendingBlock(_) => {
                                    warn!("Pending Block");
                                }
                            }
                        }
                        Err(err) => {
                            error!("Error getting block: {}", err);
                        }
                    }
                    thread::sleep(time::Duration::from_secs(poll_interval_secs));
                }
            };
            // Spawn loop function
            #[cfg(feature = "std")]
            tokio::spawn(task);
        };
        Ok(())
    }

    #[cfg(not(feature = "std"))]
    pub async fn start(&mut self) -> Result<()> {
        if let SyncStatus::NotSynced = self.sync_status {
            // Start the Ethereum light client.
            //TODO: Change unwrap
            self.ethereum_lightclient.write().await.start().await?;
            // Start the StarkNet light client.
            //TODO: Change unwrap
            self.starknet_lightclient.start().await?;
            self.sync_status = SyncStatus::Synced;

            let ethereum_clone = self.ethereum_lightclient.clone();
            let starknet_clone = self.starknet_lightclient.clone();
            let node_clone = self.node.clone();

            Interval::new(12000, move || {
                let ethereum_clone = ethereum_clone.clone();
                let starknet_clone = starknet_clone.clone();
                let node_clone = node_clone.clone();

                spawn_local(async move {
                    loop {
                        //TODO:Fix starknet_state_root and last_proven_block call. (Helios calls are working fine, but these 2 functions arent)
                        // let state_root = ethereum_clone
                        //     .read()
                        //     .await
                        //     .starknet_state_root()
                        //     .await
                        //     .unwrap();
                        // let last_proven_block = ethereum_clone
                        //     .read()
                        //     .await
                        //     .starknet_last_proven_block()
                        //     .await
                        //     .unwrap();

                        //TODO:Remove this once starknet_state_root and last_proven_block call(This is just to valdiate that Helios Fetch are working fine within the thread)
                        let block_number = ethereum_clone
                            .read()
                            .await
                            .get_block_number()
                            .await
                            .unwrap();
                        // log::info!("Loop State Root, {state_root}");
                        // log::info!("Loop Block Number, {last_proven_block}");
                        log::info!("Ethereum Block Number, {block_number}");

                        match starknet_clone
                            .get_block_with_txs(&BlockId::Tag(StarknetBlockTag::Latest))
                            .await
                        {
                            Ok(block) => {
                                let mut data = node_clone.write().await;
                                match block {
                                    MaybePendingBlockWithTxs::Block(block) => {
                                        // TODO: change "0 < block.block_number" to "block.block_number == last_proven_block"
                                        if block.block_number > data.block_number
                                            && 0 < block.block_number
                                        {
                                            data.block_number = block.block_number;
                                            data.state_root = block.new_root.to_string();
                                            data.payload.insert(block.block_number, block);
                                            log::info!("New Block Added to Payload");
                                            log::info!("Block Number {:?}", &data.block_number);
                                            log::info!("Block Root {:?}", &data.state_root);
                                        }
                                    }
                                    MaybePendingBlockWithTxs::PendingBlock(_) => {
                                        log::info!("Pending Block");
                                    }
                                }
                            }
                            Err(err) => {
                                log::info!("Error getting block: {err:?}");
                            }
                        }
                    }
                });
            })
            .forget();
        };
        Ok(())
    }

    /// Return the current synchronization status.
    pub fn sync_status(&self) -> &SyncStatus {
        &self.sync_status
    }

    /// Get the storage at a given address/key.
    ///
    /// # Arguments
    ///
    /// * `contract_address` - The StarkNet contract address as a `FieldElement`.
    /// * `storage_key` - The storage key as a `FieldElement`.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the storage value as a `FieldElement`
    /// if the operation was successful, or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// # Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    pub async fn starknet_get_storage_at(
        &self,
        contract_address: FieldElement,
        storage_key: FieldElement,
        block_id: &BlockId,
    ) -> Result<FieldElement, JsonRpcError> {
        let last_proven_block = self
            .ethereum_lightclient
            .lock()
            .await
            .starknet_last_proven_block()
            .await
            .map_err(|e| rpc_unknown_error(e.to_string()))?
            .as_u64();

        if let BlockId::Number(block_number) = block_id {
            if block_number <= &last_proven_block {
                return self
                    .starknet_lightclient
                    .get_storage_at(contract_address, storage_key, block_id)
                    .await;
            }
        } else if let MaybePendingBlockWithTxHashes::Block(block) = self
            .starknet_lightclient
            .get_block_with_tx_hashes(block_id)
            .await?
        {
            if block.block_number <= last_proven_block {
                return self
                    .starknet_lightclient
                    .get_storage_at(contract_address, storage_key, block_id)
                    .await;
            }
        }
        Err(rpc_unknown_error("BlockId is not proven yet".to_string()))
    }

    /// Call a view function of a StarkNet contract.
    ///
    /// # Arguments
    ///
    /// * `contract_address` - The StarkNet contract address as a `FieldElement`.
    /// * `entry_point_selector` - The entry point selector as a `FieldElement`.
    /// * `calldata` - The calldata as a vector of `FieldElement`.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the result of the function call as a vector of `FieldElement`
    /// if the operation was successful, or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// # Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    pub async fn starknet_call_contract(
        &self,
        contract_address: FieldElement,
        entry_point_selector: FieldElement,
        calldata: Vec<FieldElement>,
    ) -> Result<Vec<FieldElement>, JsonRpcError> {
        let opts = FunctionCall {
            contract_address,
            entry_point_selector,
            calldata,
        };

        let last_block = self
            .ethereum_lightclient
            .lock()
            .await
            .starknet_last_proven_block()
            .await
            .map_err(|e| rpc_unknown_error(e.to_string()))?
            .as_u64();

        self.starknet_lightclient
            .call(opts, &BlockId::Number(last_block))
            .await
    }

    /// Estimate the fee for a given StarkNet transaction.
    ///
    /// # Arguments
    ///
    /// * `request` - The broadcasted transaction as a `BroadcastedTransaction`.
    /// * `block_id` - The block identifier indicating the block for fee estimation.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the fee estimate as a `FeeEstimate` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// # Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    pub async fn starknet_estimate_fee(
        &self,
        request: BroadcastedTransaction,
        block_id: &BlockId,
    ) -> Result<FeeEstimate, JsonRpcError> {
        self.starknet_lightclient
            .estimate_fee(request, block_id)
            .await
    }

    /// Get the nonce at a given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The StarkNet contract address as a `FieldElement`.
    /// * `block_id` - The block identifier indicating the block to retrieve the nonce from.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the nonce as a `FieldElement` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// # Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    pub async fn starknet_get_nonce(
        &self,
        address: FieldElement,
        block_id: &BlockId,
    ) -> Result<FieldElement, JsonRpcError> {
        self.starknet_lightclient.get_nonce(block_id, address).await
    }

    /// Get the timestamp at the time `cancelL1ToL2Message` was called with a message matching `msg_hash`,
    /// or 0 if `cancelL1ToL2Message` was never called.
    ///
    /// # Arguments
    ///
    /// * `msg_hash` - The message hash as a `U256`.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the timestamp as a `U256` if the operation was successful and there is a matching message hash,
    /// or `Ok(U256::zero())` if the operation was successful but there is no matching message hash,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// # Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    pub async fn starknet_l1_to_l2_message_cancellations(
        &self,
        msg_hash: U256,
    ) -> Result<U256, JsonRpcError> {
        // Convert the message hash to bytes32.
        let msg_hash_bytes32 = ethers_helper::u256_to_bytes32_type(msg_hash);
        // Encode the function data.
        let data = ethers_helper::encode_function_data(
            msg_hash_bytes32,
            self.starknet_core_abi.clone(),
            "l1ToL2MessageCancellations",
        )
        .map_err(|e| rpc_unknown_error(e.to_string()))?;

        let data = data.to_vec();

        // Build the call options.
        let call_opts = CallOpts {
            from: None,
            to: Some(self.starknet_core_contract_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let call_response = self
            .ethereum_lightclient
            .lock()
            .await
            .call(&call_opts, BlockTag::Latest)
            .await
            .map_err(|e| rpc_unknown_error(e.to_string()))?;

        Ok(U256::from_big_endian(&call_response))
    }

    /// Get the `msg_fee + 1` from the `L1ToL2Message` hash', or 0 if there is no matching `msg_hash`.
    /// The function returns 0 if `L1ToL2Message` was never called.
    ///
    /// # Arguments
    ///
    /// * `msg_hash` - The message hash as a `U256`.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `msg_fee + 1` as a `U256` if the operation was successful and there is a matching message hash,
    /// or `Ok(U256::zero())` if the operation was successful but there is no matching message hash,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// # Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    pub async fn starknet_l1_to_l2_messages(&self, msg_hash: U256) -> Result<U256, JsonRpcError> {
        let msg_hash_bytes32 = ethers_helper::u256_to_bytes32_type(msg_hash);
        let data = ethers_helper::encode_function_data(
            msg_hash_bytes32,
            self.starknet_core_abi.clone(),
            "l1ToL2Messages",
        )
        .map_err(|e| rpc_unknown_error(e.to_string()))?;

        let data = data.to_vec();

        let call_opts = CallOpts {
            from: None,
            to: Some(self.starknet_core_contract_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        let call_response = self
            .ethereum_lightclient
            .lock()
            .await
            .call(&call_opts, BlockTag::Latest)
            .await
            .map_err(|e| rpc_unknown_error(e.to_string()))?;

        Ok(U256::from_big_endian(&call_response))
    }

    /// Get the msg_fee + 1 for the message with the given `msg_hash`, or 0 if no message with such a hash is pending.
    /// The function returns 0 if `L2ToL1Message` was never called.
    ///
    /// # Arguments
    ///
    /// * `msg_hash` - The message hash as a `U256`.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `msg_fee + 1` as a `U256` if the operation was successful and there is a matching message hash,
    /// or `Ok(U256::zero())` if the operation was successful but there is no matching message hash,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// # Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    pub async fn starknet_l2_to_l1_messages(&self, msg_hash: U256) -> Result<U256, JsonRpcError> {
        let msg_hash_bytes32 = ethers_helper::u256_to_bytes32_type(msg_hash);
        let data = ethers_helper::encode_function_data(
            msg_hash_bytes32,
            self.starknet_core_abi.clone(),
            "l2ToL1Messages",
        )
        .map_err(|e| rpc_unknown_error(e.to_string()))?;

        let data = data.to_vec();

        let call_opts = CallOpts {
            from: None,
            to: Some(self.starknet_core_contract_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let call_response = self
            .ethereum_lightclient
            .lock()
            .await
            .call(&call_opts, BlockTag::Latest)
            .await
            .map_err(|e| rpc_unknown_error(e.to_string()))?;

        Ok(U256::from_big_endian(&call_response))
    }

    /// Get the nonce for the L1-to-L2 message in the StarkNet Core contract.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the nonce as a `U256` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// # Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    pub async fn starknet_l1_to_l2_message_nonce(&self) -> Result<U256, JsonRpcError> {
        let data = ethers_helper::encode_function_data(
            (),
            self.starknet_core_abi.clone(),
            "l1ToL2MessageNonce",
        )
        .map_err(|e| rpc_unknown_error(e.to_string()))?;

        let data = data.to_vec();

        let call_opts = CallOpts {
            from: None,
            to: Some(self.starknet_core_contract_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        let call_response = self
            .ethereum_lightclient
            .lock()
            .await
            .call(&call_opts, BlockTag::Latest)
            .await
            .map_err(|e| rpc_unknown_error(e.to_string()))?;

        Ok(U256::from_big_endian(&call_response))
    }

    /// Get the block with transactions for the specified block identifier.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The block identifier.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `MaybePendingBlockWithTxs` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// # Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    pub async fn get_block_with_txs(
        &self,
        block_id: &BlockId,
    ) -> Result<MaybePendingBlockWithTxs, JsonRpcError> {
        let block_number = match block_id {
            BlockId::Number(number) => *number,
            BlockId::Tag(_) => self.starknet_lightclient.block_number().await.unwrap(),
            BlockId::Hash(_) => self.starknet_lightclient.block_number().await.unwrap(),
        };
        let node_data = self.node.read().await.clone();

        if block_number <= node_data.block_number {
            let payload_block = node_data.payload.get(&block_number).unwrap();
            Ok(MaybePendingBlockWithTxs::Block(payload_block.clone()))
        } else {
            self.starknet_lightclient.get_block_with_txs(block_id).await
        }
    }

    /// Get the block hash and number of the current block.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `BlockHashAndNumber` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// # Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    pub async fn get_block_hash_and_number(&self) -> Result<BlockHashAndNumber, JsonRpcError> {
        let cloned_node = self.node.read().await;
        let payload = cloned_node.payload.clone();

        let block = payload.get(&cloned_node.block_number);
        match block {
            Some(block) => Ok(BlockHashAndNumber {
                block_hash: block.block_hash,
                block_number: block.block_number,
            }),
            _ => Err(JsonRpcError {
                code: 24,
                message: "Block not found".to_string(),
            }),
        }
    }

    /// Return the transaction receipt of a transaction.
    ///
    /// # Arguments
    ///
    /// * `tx_hash` - The transaction hash as a String.
    ///
    /// # Returns
    ///
    /// Returns `Ok(MaybePendingTransactionReceipt)` if the operation was successful, or an `Err(eyre::Report)` if the operation failed.
    ///
    /// # Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    pub async fn starknet_get_transaction_receipt(
        &self,
        tx_hash: String,
    ) -> Result<MaybePendingTransactionReceipt, JsonRpcError> {
        let cloned_node = self.node.read().await;
        let state_root = self
            .ethereum_lightclient
            .lock()
            .await
            .starknet_state_root()
            .await
            .map_err(|e| rpc_unknown_error(e.to_string()))?
            .to_string();

        if cloned_node.state_root != state_root {
            // TODO: Select a correct error code for "State root missmatch", now its UNKNOWN ERROR
            return Err(rpc_unknown_error("State root mismatch".to_string()));
        }

        let tx_hash_felt = FieldElement::from_hex_be(&tx_hash).unwrap();
        let tx_receipt = self
            .starknet_lightclient
            .get_transaction_receipt(tx_hash_felt)
            .await?;

        Ok(tx_receipt)
    }

    /// Get the block with transaction hashes for a given block identifier.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The block identifier.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `MaybePendingBlockWithTxHashes` if the operation was successful, or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// # Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure. Possible error codes include:
    ///
    /// - `24`: Block not found.
    pub async fn get_block_with_tx_hashes(
        &self,
        block_id: &BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes, JsonRpcError> {
        let cloned_node = self.node.read().await;
        let payload = cloned_node.payload.clone();

        let block = match block_id {
            BlockId::Number(block_number) => payload.get(block_number),
            BlockId::Hash(block_hash) => {
                let block = payload
                    .values()
                    .find(|block| block.block_hash == *block_hash);
                match block {
                    Some(block) => Some(block),
                    None => {
                        // TODO: Select a correct error code for "Block with hash {} not found in the payload.", now its BLOCK_NOT_FOUND
                        return Err(JsonRpcError {
                            code: 24,
                            message: format!(
                                "Block with hash {} not found in the payload.",
                                block_hash
                            ),
                        });
                    }
                }
            }
            BlockId::Tag(tag) => match tag {
                StarknetBlockTag::Latest => payload.get(&cloned_node.block_number),
                StarknetBlockTag::Pending => {
                    let block = payload
                        .values()
                        .find(|block| block.status == BlockStatus::Pending);
                    match block {
                        Some(block) => Some(block),
                        None => {
                            // TODO: Select a correct error code for "Block with pending status not found in the payload.", now its BLOCK NOT FOUND
                            return Err(JsonRpcError {
                                code: 24,
                                message: "Block with pending status not found in the payload."
                                    .to_string(),
                            });
                        }
                    }
                }
            },
        };

        match block {
            Some(block) => {
                let tx_hashes = block
                    .clone()
                    .transactions
                    .into_iter()
                    .map(|transaction| match transaction {
                        Transaction::Invoke(tx) => match tx {
                            InvokeTransaction::V0(v0_tx) => v0_tx.transaction_hash,
                            InvokeTransaction::V1(v1_tx) => v1_tx.transaction_hash,
                        },
                        Transaction::Declare(tx) => match tx {
                            DeclareTransaction::V1(v1_tx) => v1_tx.transaction_hash,
                            DeclareTransaction::V2(v2_tx) => v2_tx.transaction_hash,
                        },
                        Transaction::L1Handler(L1HandlerTransaction {
                            transaction_hash, ..
                        })
                        | Transaction::Deploy(DeployTransaction {
                            transaction_hash, ..
                        })
                        | Transaction::DeployAccount(DeployAccountTransaction {
                            transaction_hash,
                            ..
                        }) => transaction_hash,
                    })
                    .collect();
                let block_with_tx_hashes = BlockWithTxHashes {
                    transactions: tx_hashes,
                    status: block.status.clone(),
                    block_hash: block.block_hash,
                    parent_hash: block.parent_hash,
                    block_number: block.block_number,
                    new_root: block.new_root,
                    timestamp: block.timestamp,
                    sequencer_address: block.sequencer_address,
                };
                Ok(MaybePendingBlockWithTxHashes::Block(block_with_tx_hashes))
            }
            // TODO: Select a correct error code for "Error while retrieving block.", now its BLOCK NOT FOUND
            _ => Err(JsonRpcError {
                code: 24,
                message: "Error while retrieving block.".to_string(),
            }),
        }
    }

    /// Get a transaction by its hash.
    ///
    /// # Arguments
    ///
    /// * `tx_hash` - The transaction hash as a string.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `Transaction` if the operation was successful, or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// # Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    pub async fn get_transaction_by_hash(
        &self,
        tx_hash: String,
    ) -> Result<Transaction, JsonRpcError> {
        let hash = FieldElement::from_str(&tx_hash).map_err(|_| invalid_call_data("hash"))?;

        let transaction = self
            .starknet_lightclient
            .get_transaction_by_hash(hash)
            .await
            .unwrap();

        Ok(transaction)
    }

    /// Get a transaction by the block identifier and transaction index.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The identifier of the block.
    /// * `index` - The index of the transaction within the block.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `Transaction` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// # Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    pub async fn get_transaction_by_block_and_index(
        &self,
        block_id: &BlockId,
        index: u64,
    ) -> Result<Transaction, JsonRpcError> {
        let block_with_txs = self.get_block_with_txs(block_id).await.unwrap();

        let transactions = match block_with_txs {
            MaybePendingBlockWithTxs::Block(block) => block.transactions,
            MaybePendingBlockWithTxs::PendingBlock(block) => block.transactions,
        };

        Ok(transactions[index as usize].clone())
    }

    /// Get the transaction count of a requested block.
    ///
    /// # Arguments
    ///
    /// * `block_id` - The identifier of the block.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the transaction count as `usize` if the operation was successful,
    /// or an `Err` containing a `JsonRpcError` if the operation failed.
    ///
    /// # Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure
    pub async fn get_block_transaction_count(
        &self,
        block_id: &BlockId,
    ) -> Result<usize, JsonRpcError> {
        let block_with_txs = self
            .starknet_lightclient
            .get_block_with_txs(block_id)
            .await
            .unwrap();

        let transactions = match block_with_txs {
            MaybePendingBlockWithTxs::Block(block) => block.transactions,
            MaybePendingBlockWithTxs::PendingBlock(block) => block.transactions,
        };

        let transaction_count = transactions.len();

        Ok(transaction_count)
    }

    /// Returns the pending transactions in the StarkNet transaction pool.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of pending transactions if the operation was successful (`Ok`), or an `Err` containing a `JsonRpcError` if the operation failed - indicating that no pending transactions were found.
    ///
    /// # Errors
    ///
    /// This method can return a `JsonRpcError` in case of failure.
    pub async fn starknet_pending_transactions(&self) -> Result<Vec<Transaction>, JsonRpcError> {
        let transactions_result = self.starknet_lightclient.pending_transactions().await;

        match transactions_result {
            Ok(transactions) => Ok(transactions),
            Err(err) => Err(err),
        }
    }
}

fn invalid_call_data(param: &str) -> JsonRpcError {
    let message = format!("Invalid params: cannot parse '{}'.", param);
    JsonRpcError { code: 400, message }
}

fn rpc_unknown_error(message: String) -> JsonRpcError {
    JsonRpcError { code: 520, message }
}
