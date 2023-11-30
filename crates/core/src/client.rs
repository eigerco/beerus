use std::sync::Arc;
use std::{thread, time};

use async_std::sync::RwLock;
#[cfg(not(target_arch = "wasm32"))]
use async_std::task;
use ethabi::Uint as U256;
use ethers::prelude::{abigen, EthCall};
use ethers::types::{Address, SyncingStatus};
use eyre::{eyre, Result};
use helios::client::Client;
#[cfg(target_arch = "wasm32")]
use helios::prelude::ConfigDB;
use helios::prelude::Database;
#[cfg(not(target_arch = "wasm32"))]
use helios::prelude::FileDB;
use helios::types::BlockTag;
use serde_json::json;
use starknet::core::types::{BlockId, BlockTag as SnBlockTag, FieldElement, MaybePendingBlockWithTxHashes};
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::providers::Provider;
use tracing::{debug, info, warn};

use crate::config::Config;
use crate::storage_proofs::types::StorageProofResponse;
use crate::storage_proofs::StorageProof;
use crate::utils::*;
use crate::CoreError;

abigen!(
    StarknetCoreContracts,
    r#"[
        function stateRoot() external view returns (uint256)
        function stateBlockNumber() external view returns (int256)
        function stateBlockHash() external view returns (uint256)
        function l1ToL2Messages(bytes32 msgHash) external view returns (uint256)
        function l2ToL1Messages(bytes32 msgHash) external view returns (uint256)
        function l1ToL2MessageNonce() public view returns (uint256)
        function l1ToL2MessageCancellations(bytes32 msgHash) external view returns (uint256) 
    ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);

#[derive(Clone, Debug)]
pub struct NodeData {
    pub l1_state_root: FieldElement,
    pub l1_block_num: u64,
    pub sync_root: FieldElement,
    pub sync_block_num: u64,
}

impl NodeData {
    pub fn new() -> Self {
        NodeData {
            l1_state_root: FieldElement::ZERO,
            l1_block_num: 0,
            sync_root: FieldElement::ZERO,
            sync_block_num: 0,
        }
    }
}

impl Default for NodeData {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BeerusClient {
    /// Helios Light Client
    ///
    /// Initialized w/ valid L1 execution rpc. This wraps the
    /// helios client as well as the db which stores
    /// valid L1 weak subjectivity checkpoints used to sync.
    #[cfg(not(target_arch = "wasm32"))]
    pub helios_client: Arc<Client<FileDB>>,
    #[cfg(target_arch = "wasm32")]
    pub helios_client: Arc<Client<ConfigDB>>,
    /// Starknet Client
    ///
    /// Untrusted Starknet L2 rpc.
    pub starknet_client: JsonRpcClient<HttpTransport>,
    /// Proof Address
    ///
    /// Clone of the starknet client dedicated for fetching proofs.
    pub proof_addr: String,
    /// Poll Interval Seconds
    ///
    /// Interval at which beerus will check if the L1 State Root
    /// has been updated.
    pub poll_secs: u64,
    /// Core Contract Address
    ///
    /// Address of the L1 core contracts for starknet.
    /// Depends on the network beerus is initialized w/.
    pub core_contract_addr: Address,
    /// Node Data
    ///
    /// Mutex guarded node information including:
    /// - latest l1 state root
    /// - latest l1 block number
    /// - current local root(updated via cache of unproven block data)
    /// - current block number(updated via cache of unproven block data)
    pub node: Arc<RwLock<NodeData>>,
    /// Beerus Config
    ///
    /// Configuration for Beerus client including l1 + l2
    /// untrusted rpc addresses, and network information
    pub config: Config,
}

impl BeerusClient {
    pub async fn new(config: Config) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let mut helios_client: Client<FileDB> = config.to_helios_client().await;
        #[cfg(target_arch = "wasm32")]
        let mut helios_client: Client<ConfigDB> = config.to_helios_client().await;

        helios_client.start().await.expect("could not init helios client");

        while helios_client.syncing().await.expect("could not init helios client") != SyncingStatus::IsFalse {
            debug!("{} not in sync yet", config.network);

            thread::sleep(time::Duration::from_secs(1));
        }

        if let Err(err) = sn_state_root_inner(&helios_client, config.get_core_contract_address()).await {
            panic!("execution client err: \n\n{err:#?}");
        }

        Self {
            helios_client: Arc::new(helios_client),
            starknet_client: config.to_starknet_client(),
            proof_addr: config.starknet_rpc.clone(),
            poll_secs: config.poll_secs,
            core_contract_addr: config.get_core_contract_address(),
            node: Arc::new(RwLock::new(NodeData::new())),
            config: config.clone(),
        }
    }

    /// Start a async thread to query the last updated state of Starknet
    /// via the Core Contracts on L1(updated via the `updateState` call).
    ///
    /// The loop run in this thread will query these values at
    /// `config.poll_secs` seconds
    pub async fn start(&mut self) -> Result<()> {
        let (config, l1_client, node) = (self.config.clone(), self.helios_client.clone(), self.node.clone());

        let state_loop = async move {
            let l2_client = config.to_starknet_client();
            let core_contract_addr = config.get_core_contract_address();

            loop {
                let sn_root = sn_state_root_inner(&l1_client, core_contract_addr).await.unwrap();
                let sn_block_num = sn_state_block_number_inner(&l1_client, core_contract_addr).await.unwrap();
                let local_block_num = node.read().await.l1_block_num;

                if local_block_num < sn_block_num {
                    // TODO: Issue #550 - feat: sync from proven root
                    match l2_client.get_block_with_tx_hashes(BlockId::Tag(SnBlockTag::Latest)).await.unwrap() {
                        MaybePendingBlockWithTxHashes::Block(block) => {
                            info!(
                                "{} blocks behind - L1 block #({sn_block_num}) L2 block #({:?})",
                                block.block_number - sn_block_num,
                                block.block_number
                            );

                            let mut node_lock = node.write().await;
                            node_lock.l1_state_root = sn_root;
                            node_lock.l1_block_num = sn_block_num;
                        }
                        MaybePendingBlockWithTxHashes::PendingBlock(_) => warn!("expecting latest got pending"),
                    };
                }

                thread::sleep(time::Duration::from_secs(config.poll_secs));
            }
        };
        #[cfg(not(target_arch = "wasm32"))]
        task::spawn(state_loop);
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(state_loop);

        Ok(())
    }

    pub async fn sn_state_root(&self) -> Result<FieldElement> {
        sn_state_root_inner(&self.helios_client, self.core_contract_addr).await
    }

    pub async fn sn_state_block_number(&self) -> Result<u64, CoreError> {
        sn_state_block_number_inner(&self.helios_client, self.core_contract_addr).await
    }

    pub async fn sn_state_block_hash(&self) -> Result<FieldElement, CoreError> {
        let data = StateBlockHashCall::selector();
        let call_opts = simple_call_opts(self.core_contract_addr, data.into());

        let sn_block_hash =
            self.helios_client.call(&call_opts, BlockTag::Latest).await.map_err(CoreError::FetchL1Val)?;

        FieldElement::from_byte_slice_be(&sn_block_hash).map_err(|e| CoreError::FetchL1Val(eyre!("{e}")))
    }

    pub async fn get_local_root(&self) -> FieldElement {
        self.node.read().await.l1_state_root
    }

    pub async fn get_local_block_num(&self) -> u64 {
        self.node.read().await.l1_block_num
    }

    /// If user queries historical block i.e. provided block num < local block num
    /// Allow request to continue with that value.
    ///
    /// Until Issue #550 is implemented blocks in the range block num > local block num
    /// will default to local block num
    pub async fn get_local_block_id(&self, provided_block_id: BlockId) -> BlockId {
        let local_block_num = self.get_local_block_num().await;

        match provided_block_id {
            BlockId::Number(num) => {
                if local_block_num > num {
                    provided_block_id
                } else {
                    BlockId::Number(local_block_num)
                }
            }
            _ => BlockId::Number(local_block_num),
        }
    }

    /// Fetch the storage proof at block `block_id`
    /// that proves a known value exists at a known key.
    ///
    /// Request is exposed by untrusted rpc's `pathfinder_getProof`
    /// endpoint.
    pub async fn get_proof(
        &self,
        block_id: BlockId,
        contract_address: &FieldElement,
        keys: &[FieldElement],
    ) -> Result<StorageProof, CoreError> {
        let client =
            reqwest::Client::builder().build().map_err(|e| CoreError::StorageProof(eyre!("build request: {e:?}")))?;

        let keys = keys.iter().map(|i| format!("0x{i:x}")).collect::<Vec<String>>();
        let addr = format!("0x{contract_address:x}");
        let block_id = self.get_local_block_id(block_id).await;

        let params = json!({
            "jsonrpc": "2.0",
            "method": "pathfinder_getProof",
            "params": {"block_id": block_id, "contract_address": addr, "keys": keys},
            "id": 0
        });

        let request = client.request(reqwest::Method::POST, &self.proof_addr).json(&params);

        let response = request.send().await.map_err(|e| CoreError::StorageProof(eyre!("proof request: {e:?}")))?;
        let body: StorageProofResponse =
            response.json().await.map_err(|e| CoreError::StorageProof(eyre!("proof response: {e:?}")))?;

        match body.error {
            Some(e) => Err(CoreError::StorageProof(eyre!("error in proof request: {e:?}"))),
            None => Ok(body.result.unwrap()),
        }
    }
}

async fn sn_state_root_inner(l1_client: &Client<impl Database>, contract_addr: Address) -> Result<FieldElement> {
    let data = StateRootCall::selector();
    let call_opts = simple_call_opts(contract_addr, data.into());

    let starknet_root = l1_client.call(&call_opts, BlockTag::Latest).await.map_err(CoreError::FetchL1Val)?;

    FieldElement::from_byte_slice_be(&starknet_root).map_err(|e| eyre!(e))
}

async fn sn_state_block_number_inner(
    l1_client: &Client<impl Database>,
    core_contract_addr: Address,
) -> Result<u64, CoreError> {
    let data = StateBlockNumberCall::selector();
    let call_opts = simple_call_opts(core_contract_addr, data.into());

    let sn_block_num = l1_client.call(&call_opts, BlockTag::Latest).await.map_err(CoreError::FetchL1Val)?;

    Ok(U256::from_big_endian(&sn_block_num).as_u64())
}
