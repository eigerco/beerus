use std::sync::Arc;
use std::{thread, time};

use ethabi::ethereum_types::H160;
use ethabi::Uint as U256;
use ethers::prelude::{abigen, EthCall};
use ethers::types::{Address, SyncingStatus};
use eyre::{eyre, Context, Result};
use helios::client::Client;
#[cfg(target_arch = "wasm32")]
use helios::prelude::ConfigDB;
use helios::prelude::Database;
#[cfg(not(target_arch = "wasm32"))]
use helios::prelude::FileDB;
use helios::types::BlockTag;
use serde_json::json;
use starknet::core::types::{
    BlockId, BlockTag as StarknetBlockTag, FieldElement,
    MaybePendingBlockWithTxHashes,
};
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::providers::Provider;
use tokio::sync::RwLock;
#[cfg(not(target_arch = "wasm32"))]
use tokio::task;
use tracing::{debug, error, info};

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
    pub l1_block_number: u64,
    pub sync_root: FieldElement,
    pub sync_block_number: u64,
}

impl NodeData {
    pub fn new() -> Self {
        NodeData {
            l1_state_root: FieldElement::ZERO,
            l1_block_number: 0,
            sync_root: FieldElement::ZERO,
            sync_block_number: 0,
        }
    }

    pub fn update(
        &mut self,
        l1_block_number: u64,
        l1_state_root: FieldElement,
    ) {
        self.l1_block_number = l1_block_number;
        self.l1_state_root = l1_state_root;
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
    pub async fn new(config: Config) -> Result<Self> {
        #[cfg(not(target_arch = "wasm32"))]
        let mut helios_client: Client<FileDB> = config.to_helios_client().await;
        #[cfg(target_arch = "wasm32")]
        let mut helios_client: Client<ConfigDB> =
            config.to_helios_client().await;

        helios_client.start().await.context("failed to start helios client")?;

        while let SyncingStatus::IsSyncing(sync) = helios_client
            .syncing()
            .await
            .context("failed to fetch syncing status")?
        {
            debug!("{} syncing: head={}", config.network, sync.highest_block);
            thread::sleep(time::Duration::from_secs(1));
        }

        get_starknet_state_root(
            &helios_client,
            config.get_core_contract_address()?,
        )
        .await
        .context("failed to fetch starknet state root")?;

        Ok(Self {
            helios_client: Arc::new(helios_client),
            starknet_client: config.to_starknet_client(),
            proof_addr: config.starknet_rpc.clone(),
            poll_secs: config.poll_secs,
            core_contract_addr: config.get_core_contract_address()?,
            node: Arc::new(RwLock::new(NodeData::new())),
            config: config.clone(),
        })
    }

    #[cfg_attr(doc, aquamarine::aquamarine)]
    /// Start a async thread to query the last updated state of Starknet
    /// via the Core Contracts on L1(updated via the `updateState` call).
    ///
    /// The loop run in this thread will query these values at
    /// `config.poll_secs` seconds
    ///
    /// Synchronization process sequence:
    /// ```mermaid
    /// sequenceDiagram
    /// Beerus->>+L1: get_starknet_state_root
    /// L1-->>-Beerus: starknet state root
    /// Note over Beerus,L1: State root: hash of the state Merkle tree
    /// Beerus->>+L1: get_startknet_block_number
    /// L1-->>-Beerus: starknet block number synched in L1
    /// alt local L1 block number >= L1 block number
    ///     Beerus->>Beerus: return None
    ///     Note over Beerus,L1: Local state is at least as up-to-date as the remote node
    /// else
    ///     Beerus->>+L2: get_block_with_tx_hashes(latest)
    ///     L2-->>-Beerus: latest L2 block
    ///     Beerus->>Beerus: Update local state to the new, unproven state
    /// end
    /// ```
    pub async fn start(&mut self) -> Result<()> {
        let l1_client = self.helios_client.clone();
        let l2_client = self.config.to_starknet_client();
        let core_contract_addr = self.config.get_core_contract_address()?;
        let node = self.node.clone();
        let poll_interval = time::Duration::from_secs(self.config.poll_secs);

        let state_loop = async move {
            loop {
                match sync(
                    &l1_client,
                    &l2_client,
                    core_contract_addr,
                    node.clone(),
                )
                .await
                {
                    Ok(Some(block_number)) => {
                        info!("synced block: {block_number}")
                    }
                    Ok(None) => debug!("already at head block"),
                    Err(e) => error!("failed to pull block: {e}"),
                };
                debug!("state loop: delay {poll_interval:?}");
                thread::sleep(poll_interval);
            }
        };

        #[cfg(not(target_arch = "wasm32"))]
        task::spawn(state_loop);
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(state_loop);

        Ok(())
    }

    pub async fn state_root(&self) -> Result<FieldElement> {
        get_starknet_state_root(&self.helios_client, self.core_contract_addr)
            .await
    }

    pub async fn state_block_number(&self) -> Result<u64, CoreError> {
        get_starknet_state_block_number(
            &self.helios_client,
            self.core_contract_addr,
        )
        .await
    }

    pub async fn state_block_hash(&self) -> Result<FieldElement, CoreError> {
        let data = StateBlockHashCall::selector();
        let call_opts = simple_call_opts(self.core_contract_addr, data.into());

        let sn_block_hash = self
            .helios_client
            .call(&call_opts, BlockTag::Latest)
            .await
            .map_err(CoreError::FetchL1Val)?;

        FieldElement::from_byte_slice_be(&sn_block_hash)
            .map_err(|e| CoreError::FetchL1Val(eyre!("{e}")))
    }

    pub async fn get_local_root(&self) -> FieldElement {
        self.node.read().await.l1_state_root
    }

    pub async fn get_local_block_num(&self) -> u64 {
        self.node.read().await.l1_block_number
    }

    /// If user queries historical block i.e. provided block num < local block num
    /// Allow request to continue with that value.
    pub async fn get_local_block_id(
        &self,
        provided_block_id: BlockId,
    ) -> BlockId {
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
        let client = reqwest::Client::builder().build().map_err(|e| {
            CoreError::StorageProof(eyre!("build request: {e:?}"))
        })?;

        let keys =
            keys.iter().map(|i| format!("0x{i:x}")).collect::<Vec<String>>();
        let addr = format!("0x{contract_address:x}");
        let block_id = self.get_local_block_id(block_id).await;

        let params = json!({
            "jsonrpc": "2.0",
            "method": "pathfinder_getProof",
            "params": {"block_id": block_id, "contract_address": addr, "keys": keys},
            "id": 0
        });

        let request = client
            .request(reqwest::Method::POST, &self.proof_addr)
            .json(&params);

        let response = request.send().await.map_err(|e| {
            CoreError::StorageProof(eyre!("proof request: {e:?}"))
        })?;
        let body: StorageProofResponse =
            response.json().await.map_err(|e| {
                CoreError::StorageProof(eyre!("proof response: {e:?}"))
            })?;

        body.result.ok_or_else(|| {
            let error = body
                .error
                .map(|e| eyre!("failed to get proof: {e:?}"))
                .unwrap_or_else(|| eyre!("undefined"));
            CoreError::StorageProof(error)
        })
    }
}

async fn sync(
    l1_client: &Client<impl Database>,
    l2_client: &JsonRpcClient<HttpTransport>,
    core_contract_addr: H160,
    node: Arc<RwLock<NodeData>>,
) -> Result<Option<u64>> {
    let starknet_state_root =
        get_starknet_state_root(l1_client, core_contract_addr).await?;
    let l1_starknet_block_number =
        get_starknet_state_block_number(l1_client, core_contract_addr).await?;
    let local_block_number = node.read().await.l1_block_number;

    debug!("starknet block number: {l1_starknet_block_number}, local block number: {local_block_number}");
    // The local state is up to date with the remote node. Nothing more to do.
    if local_block_number >= l1_starknet_block_number {
        return Ok(None);
    }

    // The local state is out of date, retrieve the latest block from L2.
    match l2_client
        .get_block_with_tx_hashes(BlockId::Tag(StarknetBlockTag::Latest))
        .await
    {
        Ok(MaybePendingBlockWithTxHashes::Block(l2_latest_block)) => {
            let blocks_behind =
                l2_latest_block.block_number - l1_starknet_block_number;
            info!(
                "L1 block: {}, L2 block: {} (L1 is {} blocks behind)",
                l1_starknet_block_number,
                l2_latest_block.block_number,
                blocks_behind
            );

            let mut guard = node.write().await;
            guard.update(l1_starknet_block_number, starknet_state_root);
            Ok(Some(l2_latest_block.block_number))
        }
        Ok(MaybePendingBlockWithTxHashes::PendingBlock(_)) => {
            Err(eyre!("expecting latest got pending"))
        }
        Err(e) => Err(eyre!("failed to fetch last block: {e}")),
    }
}

async fn get_starknet_state_root(
    l1_client: &Client<impl Database>,
    contract_addr: Address,
) -> Result<FieldElement> {
    let data = StateRootCall::selector();
    let call_opts = simple_call_opts(contract_addr, data.into());

    let state_root = l1_client
        .call(&call_opts, BlockTag::Latest)
        .await
        .map_err(CoreError::FetchL1Val)?;
    FieldElement::from_byte_slice_be(&state_root).map_err(|e| eyre!(e))
}

async fn get_starknet_state_block_number(
    l1_client: &Client<impl Database>,
    core_contract_addr: Address,
) -> Result<u64, CoreError> {
    let data = StateBlockNumberCall::selector();
    let call_opts = simple_call_opts(core_contract_addr, data.into());

    let block_number = l1_client
        .call(&call_opts, BlockTag::Latest)
        .await
        .map_err(CoreError::FetchL1Val)?;
    Ok(U256::from_big_endian(&block_number).as_u64())
}
