use super::{ethereum::EthereumLightClient, starknet::StarkNetLightClient};
use crate::{config::Config, ethers_helper};
use ethers::{
    abi::Abi,
    types::{H160, U256},
};
use eyre::Result;
use helios::types::{BlockTag, CallOpts};
use starknet::{
    core::types::FieldElement,
    providers::jsonrpc::models::{FunctionCall, InvokeTransaction, Transaction},
};

use starknet::providers::jsonrpc::models::{
    BlockId, BlockTag as StarknetBlockTag, BlockWithTxHashes, BlockWithTxs,
    DeclareTransaction, DeployAccountTransaction, DeployTransaction, L1HandlerTransaction,
    MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs,
};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio;

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
    pub ethereum_lightclient: Box<dyn EthereumLightClient>,
    /// StarkNet light client.
    pub starknet_lightclient: Box<dyn StarkNetLightClient>,
    /// Sync status.
    pub sync_status: SyncStatus,
    /// StarkNet core ABI.
    pub starknet_core_abi: Abi,
    /// StarkNet core contract address.
    pub starknet_core_contract_address: H160,

    pub node_data: Arc<Mutex<NodeData>>,
}

impl BeerusLightClient {
    /// Create a new Beerus Light Client service.
    pub fn new(
        config: Config,
        ethereum_lightclient: Box<dyn EthereumLightClient>,
        starknet_lightclient: Box<dyn StarkNetLightClient>,
    ) -> Self {
        let starknet_core_abi = include_str!("../resources/starknet_core_abi.json");
        // Deserialize the StarkNet core ABI.
        // For now we assume that the ABI is valid and that the deserialization will never fail.
        let starknet_core_abi: Abi = serde_json::from_str(starknet_core_abi).unwrap();
        let starknet_core_contract_address = config.starknet_core_contract_address;

        Self {
            config,
            ethereum_lightclient,
            starknet_lightclient,
            sync_status: SyncStatus::NotSynced,
            starknet_core_abi,
            starknet_core_contract_address,
            node_data: Arc::new(Mutex::new(NodeData::new())),
        }
    }

    /// Start Beerus light client and synchronize with Ethereum and StarkNet.
    pub async fn start(
        &mut self,
        config: Config,
        ethereum_lightclient: Box<dyn EthereumLightClient>,
        starknet_lightclient: Box<dyn StarkNetLightClient>,
    ) -> Result<()> {
        if let SyncStatus::NotSynced = self.sync_status {
            // Start the Ethereum light client.
            self.ethereum_lightclient.start().await?;

            // Start the StarkNet light client.
            self.starknet_lightclient.start().await?;

            self.sync_status = SyncStatus::Synced;

            let node = self.node_data.clone();
            let mut beerus_client =
                BeerusLightClient::new(config, ethereum_lightclient, starknet_lightclient);
            beerus_client.ethereum_lightclient.start().await?;

            let task = async move {
                loop {
                    //TODO: Fix last_proven_block and implement if condition
                    // let last_proven_block = beerus_client
                    //     .starknet_last_proven_block()
                    //     .await
                    //     .unwrap()
                    //     .as_u64();

                    //TODO: Fix starknet_state_root and implement if condition
                    // let last_starknet_state =
                    //     beerus_client.starknet_state_root().await.unwrap().as_u64();

                    match beerus_client
                        .starknet_lightclient
                        .get_block_with_txs(&BlockId::Tag(StarknetBlockTag::Latest))
                        .await
                    {
                        Ok(block) => {
                            let mut data = node.lock().unwrap();
                            match block {
                                MaybePendingBlockWithTxs::Block(block) => {
                                    // TODO: change "0 < block.block_number" to "block.block_number == last_proven_block"
                                    if block.block_number > data.block_number
                                        && 0 < block.block_number
                                    {
                                        data.block_number = block.block_number;
                                        data.state_root = block.new_root.to_string();
                                        data.payload.insert(block.block_number, block);
                                        println!("New Block Added to Payload");
                                        println!("Block Number {:?}", &data.block_number);
                                        println!("Block Root {:?}", &data.state_root);
                                    }
                                }
                                MaybePendingBlockWithTxs::PendingBlock(_) => {
                                    println!("Pending Block");
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("Error getting block: {err:?}");
                        }
                    }
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            };

            tokio::spawn(task);
        }
        Ok(())
    }

    /// Return the current synchronization status.
    pub fn sync_status(&self) -> &SyncStatus {
        &self.sync_status
    }

    /// Get the StarkNet state root.
    pub async fn starknet_state_root(&self) -> Result<U256> {
        // Get the StarkNet core contract address.
        let starknet_core_contract_address = &self.config.starknet_core_contract_address;

        // Corresponds to the StarkNet core contract function `stateRoot`.
        // The function signature is `stateRoot() -> (uint256)`.
        // The function selector is `0x95d8ecA2`.
        let data = vec![0x95, 0x88, 0xec, 0xa2];

        // Build the call options.
        let call_opts = CallOpts {
            from: None,
            to: *starknet_core_contract_address,
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let starknet_root = self
            .ethereum_lightclient
            .call(&call_opts, BlockTag::Latest)
            .await?;

        // Convert the response bytes to a U256.
        let starknet_root = U256::from_big_endian(&starknet_root);

        Ok(starknet_root)
    }

    /// Get the StarkNet last proven block number.
    /// This function is used to get the last proven block number of the StarkNet network.
    ///
    /// # Returns
    /// `Ok(U256)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn starknet_last_proven_block(&self) -> Result<U256> {
        // Get the StarkNet core contract address.
        let starknet_core_contract_address = &self.config.starknet_core_contract_address;

        let data = vec![53, 190, 250, 93];

        // Build the call options.
        let call_opts = CallOpts {
            from: None,
            to: *starknet_core_contract_address,
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let starknet_root = self
            .ethereum_lightclient
            .call(&call_opts, BlockTag::Latest)
            .await?;

        // Convert the response bytes to a U256.
        let starknet_root = U256::from_big_endian(&starknet_root);

        Ok(starknet_root)
    }

    /// Get the storage at a given address/key.
    /// This function is used to get the storage at a given address and key.
    ///
    /// # Arguments
    ///
    /// * `contract_address` - The StarkNet contract address.
    /// * `storage_key` - The storage key.
    ///
    /// # Returns
    ///
    /// `Ok(FieldElement)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn starknet_get_storage_at(
        &self,
        contract_address: FieldElement,
        storage_key: FieldElement,
    ) -> Result<FieldElement> {
        let last_block = self.starknet_last_proven_block().await?.as_u64();
        self.starknet_lightclient
            .get_storage_at(contract_address, storage_key, last_block)
            .await
    }

    /// Call starknet contract view.
    /// This function is used to call a view function of a StarkNet contract.
    /// WARNING: This function is untrusted as there's no access list on StarkNet (yet @Avihu).
    ///
    /// # Arguments
    /// * `contract_address` - The StarkNet contract address.
    /// * `entry_point_selector` - The entry point selector.
    /// * `calldata` - The calldata.
    ///
    /// # Returns
    ///
    /// `Ok(Vec<FieldElement>)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn starknet_call_contract(
        &self,
        contract_address: FieldElement,
        entry_point_selector: FieldElement,
        calldata: Vec<FieldElement>,
    ) -> Result<Vec<FieldElement>> {
        let opts = FunctionCall {
            contract_address,
            entry_point_selector,
            calldata,
        };

        let last_block = self.starknet_last_proven_block().await?.as_u64();
        // Call the StarkNet light client.
        self.starknet_lightclient.call(opts, last_block).await
    }

    /// Get the nonce at a given address.
    /// This function is used to get the nonce at a given address.
    ///
    /// # Arguments
    ///
    /// * `contract_address` - The StarkNet contract address.
    ///
    /// # Returns
    ///
    /// `Ok(FieldElement)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn starknet_get_nonce(&self, address: FieldElement) -> Result<FieldElement> {
        let last_block = self.starknet_last_proven_block().await?.as_u64();

        self.starknet_lightclient
            .get_nonce(last_block, address)
            .await
    }

    /// Return the timestamp at the time cancelL1ToL2Message was called with a message matching 'msg_hash'.
    /// The function returns 0 if cancelL1ToL2Message was never called.
    /// See https://github.com/starknet-io/starknet-addresses for the StarkNet core contract address on different networks.
    /// # Arguments
    /// * `msg_hash` - The message hash as bytes32.
    /// # Returns
    /// `Ok(U256)` if the operation was successful - The timestamp at the time cancelL1ToL2Message was called with a message matching 'msg_hash'.
    /// `Ok(U256::zero())` if the operation was successful - The function returns 0 if cancelL1ToL2Message was never called.
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn starknet_l1_to_l2_message_cancellations(&self, msg_hash: U256) -> Result<U256> {
        // Convert the message hash to bytes32.
        let msg_hash_bytes32 = ethers_helper::u256_to_bytes32_type(msg_hash);
        // Encode the function data.
        let data = ethers_helper::encode_function_data(
            msg_hash_bytes32,
            self.starknet_core_abi.clone(),
            "l1ToL2MessageCancellations",
        )?;
        let data = data.to_vec();

        // Build the call options.
        let call_opts = CallOpts {
            from: None,
            to: self.starknet_core_contract_address,
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let call_response = self
            .ethereum_lightclient
            .call(&call_opts, BlockTag::Latest)
            .await?;
        Ok(U256::from_big_endian(&call_response))
    }

    /// Return the msg_fee + 1 from the L1ToL2Message hash'. 0 if there is no matching msg_hash
    /// The function returns 0 if L1ToL2Message was never called.
    /// See https://github.com/starknet-io/starknet-addresses for the StarkNet core contract address on different networks.
    /// # Arguments
    /// * `msg_hash` - The message hash as bytes32.
    /// # Returns
    /// `Ok(U256)` if the operation was successful - The msg_fee + 1 from the L1ToL2Message hash'.
    /// `Ok(U256::zero())` if the operation was successful - The function returns 0 if there is no match on the message hash
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn starknet_l1_to_l2_messages(&self, msg_hash: ethers::types::U256) -> Result<U256> {
        // Convert the message hash to bytes32.
        let msg_hash_bytes32 = ethers_helper::u256_to_bytes32_type(msg_hash);
        // Encode the function data.
        let data = ethers_helper::encode_function_data(
            msg_hash_bytes32,
            self.starknet_core_abi.clone(),
            "l1ToL2Messages",
        )?;
        let data = data.to_vec();

        // Build the call options.
        let call_opts = CallOpts {
            from: None,
            to: self.starknet_core_contract_address,
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let call_response = self
            .ethereum_lightclient
            .call(&call_opts, BlockTag::Latest)
            .await?;
        Ok(U256::from_big_endian(&call_response))
    }

    ///  Returns the msg_fee + 1 for the message with the given 'msgHash', or 0 if no message with such a hash is pending.
    /// The function returns 0 if L2ToL1Message was never called.
    /// See https://github.com/starknet-io/starknet-addresses for the StarkNet core contract address on different networks.
    /// # Arguments
    /// * `msg_hash` - The message hash as bytes32.
    /// # Returns
    /// `Ok(U256)` if the operation was successful - The msg_fee + 1 from the L2ToL1Message hash'.
    /// `Ok(U256::zero())` if the operation was successful - The function returns 0 if there is no matching message hash
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn starknet_l2_to_l1_messages(&self, msg_hash: U256) -> Result<U256> {
        // Convert the message hash to bytes32.
        let msg_hash_bytes32 = ethers_helper::u256_to_bytes32_type(msg_hash);
        // Encode the function data.
        let data = ethers_helper::encode_function_data(
            msg_hash_bytes32,
            self.starknet_core_abi.clone(),
            "l2ToL1Messages",
        )?;
        let data = data.to_vec();

        // Build the call options.
        let call_opts = CallOpts {
            from: None,
            to: self.starknet_core_contract_address,
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let call_response = self
            .ethereum_lightclient
            .call(&call_opts, BlockTag::Latest)
            .await?;
        Ok(U256::from_big_endian(&call_response))
    }

    /// Return the nonce for the L1ToL2Message bridge.
    /// See https://github.com/starknet-io/starknet-addresses for the StarkNet core contract address on different networks.
    /// # Arguments
    /// # Returns
    /// `Ok(U256)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn starknet_l1_to_l2_message_nonce(&self) -> Result<U256> {
        // Encode the function data.
        let data = ethers_helper::encode_function_data(
            (),
            self.starknet_core_abi.clone(),
            "l1ToL2MessageNonce",
        )?;
        let data = data.to_vec();

        // Build the call options.
        let call_opts = CallOpts {
            from: None,
            to: self.starknet_core_contract_address,
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let call_response = self
            .ethereum_lightclient
            .call(&call_opts, BlockTag::Latest)
            .await?;
        Ok(U256::from_big_endian(&call_response))
    }

    /// Return block with transactions.
    /// See https://github.com/starknet-io/starknet-addresses for the StarkNet core contract address on different networks.
    /// # Arguments
    /// BlockId
    /// # Returns
    /// `Ok(MaybePendingBlockWithTxs)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn get_block_with_txs(&self, block_id: &BlockId) -> Result<MaybePendingBlockWithTxs> {
        // Get block_number from block_id
        let block_number = match block_id {
            BlockId::Number(number) => *number,
            BlockId::Tag(_) => self.starknet_lightclient.block_number().await.unwrap(),
            BlockId::Hash(_) => self.starknet_lightclient.block_number().await.unwrap(),
        };
        // Clone the node_data
        let node_data = self.node_data.lock().unwrap().clone();

        // Check if block_number its smaller or equal payload
        if block_number <= node_data.block_number {
            // Get state_root for current block_number
            let payload_block = node_data.payload.get(&block_number).unwrap();
            Ok(MaybePendingBlockWithTxs::Block(payload_block.to_owned()))
        } else {
            self.starknet_lightclient.get_block_with_txs(block_id).await
        }
    }

    /// Return block with transaction hashes.
    /// See https://github.com/starknet-io/starknet-addresses for the StarkNet core contract address on different networks.
    /// # Arguments
    /// BlockId
    /// # Returns
    /// `Ok(MaybePendingBlockWithTxHashes)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    pub async fn get_block_with_tx_hashes(
        &self,
        block_id: &BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes> {
        let block = self.get_block_with_txs(block_id).await?;
        match block {
            MaybePendingBlockWithTxs::Block(block) => {
                let tx_hashes = block
                    .transactions
                    .into_iter()
                    .map(|transaction| match transaction {
                        Transaction::Invoke(tx) => match tx {
                            InvokeTransaction::V0(v0_tx) => v0_tx.transaction_hash,
                            InvokeTransaction::V1(v1_tx) => v1_tx.transaction_hash,
                        },
                        Transaction::L1Handler(L1HandlerTransaction {
                            transaction_hash, ..
                        })
                        | Transaction::Declare(DeclareTransaction {
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
                    status: block.status,
                    block_hash: block.block_hash,
                    parent_hash: block.parent_hash,
                    block_number: block.block_number,
                    new_root: block.new_root,
                    timestamp: block.timestamp,
                    sequencer_address: block.sequencer_address,
                };
                Ok(MaybePendingBlockWithTxHashes::Block(block_with_tx_hashes))
            }
            // todo update with get_block_with_txs_hashes RPC call when available
            // self.starknet_lightclient.get_block_with_txs_hashes(block_id).await,
            _ => unimplemented!(),
        }
    }
}
