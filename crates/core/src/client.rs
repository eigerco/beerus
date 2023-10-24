use std::sync::Arc;
use std::{thread, time};

use async_std::sync::RwLock;
use ethabi::Uint as U256;
use ethers::abi::AbiEncode;
use ethers::prelude::{abigen, EthCall};
use ethers::types::{Address, SyncingStatus};
use eyre::{eyre, Result};
use helios::client::Client;
use helios::prelude::FileDB;
use helios::types::BlockTag;
use log::{debug, info, warn};
use serde_json::json;
use stark_hash::Felt;
use starknet::core::types::{BlockId, BlockTag as SnBlockTag, FieldElement, MaybePendingBlockWithTxHashes};
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::providers::Provider;

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

pub struct BeerusClient {
    /// Ethereum light client.
    pub helios_client: Client<FileDB>,
    /// StarkNet light client.
    pub starknet_client: JsonRpcClient<HttpTransport>,
    /// Proof client.
    pub proof_addr: String,
    /// Poll interval
    pub poll_secs: u64,
    /// StarkNet core contract address.
    pub core_contract_addr: Address,
    /// Payload data
    pub node: Arc<RwLock<NodeData>>,
}

#[derive(Clone, Debug)]
pub struct NodeData {
    pub l1_state_root: Felt,
    pub l1_block_num: u64,
}

impl NodeData {
    pub fn new() -> Self {
        NodeData { l1_state_root: Felt::ZERO, l1_block_num: 0 }
    }
}

impl BeerusClient {
    /// Create a new Beerus Light Client service.
    pub async fn new(config: Config) -> Self {
        Self {
            helios_client: config.to_helios_client().await,
            starknet_client: config.to_starknet_client(),
            proof_addr: config.starknet_rpc.clone(),
            poll_secs: config.poll_secs,
            core_contract_addr: config.get_core_contract_address(),
            node: Arc::new(RwLock::new(NodeData::new())),
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        self.helios_client.start().await?;

        while self.helios_client.syncing().await? != SyncingStatus::IsFalse {
            debug!("not in sync yet");

            thread::sleep(time::Duration::from_secs(1));
        }

        loop {
            let sn_root = self.sn_state_root().await?;
            let sn_block_num = self.sn_state_block_number().await?;
            let local_block_num = self.get_local_block_num().await;

            if local_block_num < sn_block_num {
                match self.starknet_client.get_block_with_tx_hashes(BlockId::Tag(SnBlockTag::Latest)).await? {
                    MaybePendingBlockWithTxHashes::Block(block) => {
                        info!(
                            "{} blocks behind - L1 block #({sn_block_num}) L2 block #({:?})",
                            block.block_number - sn_block_num,
                            block.block_number
                        );

                        let mut node_lock = self.node.write().await;
                        node_lock.l1_state_root = sn_root;
                        node_lock.l1_block_num = sn_block_num;
                    }
                    MaybePendingBlockWithTxHashes::PendingBlock(_) => warn!("expecting latest got pending"),
                };
            }

            thread::sleep(time::Duration::from_secs(self.poll_secs));
        }
    }

    pub async fn sn_state_root(&self) -> Result<Felt> {
        let data = StateRootCall::selector();
        let call_opts = simple_call_opts(self.core_contract_addr, data.into());

        let starknet_root =
            self.helios_client.call(&call_opts, BlockTag::Latest).await.map_err(CoreError::FetchL1Val)?;

        Felt::from_be_slice(&starknet_root).map_err(|e| eyre!(e))
    }

    pub async fn sn_state_block_number(&self) -> Result<u64, CoreError> {
        let data = StateBlockNumberCall::selector();
        let call_opts = simple_call_opts(self.core_contract_addr, data.into());

        let sn_block_num =
            self.helios_client.call(&call_opts, BlockTag::Latest).await.map_err(CoreError::FetchL1Val)?;

        // Convert the response bytes to a U256.
        Ok(U256::from_big_endian(&sn_block_num).as_u64())
    }

    pub async fn sn_state_block_hash(&self) -> Result<U256, CoreError> {
        let data = StateBlockHashCall::selector();
        let call_opts = simple_call_opts(self.core_contract_addr, data.into());

        let sn_block_hash =
            self.helios_client.call(&call_opts, BlockTag::Latest).await.map_err(CoreError::FetchL1Val)?;

        // Convert the response bytes to a U256.
        Ok(U256::from_big_endian(&sn_block_hash))
    }

    pub async fn get_local_root(&self) -> Felt {
        self.node.read().await.l1_state_root
    }

    pub async fn get_local_block_num(&self) -> u64 {
        self.node.read().await.l1_block_num
    }

    pub async fn get_local_block_id(&self) -> BlockId {
        BlockId::Number(self.get_local_block_num().await)
    }

    pub async fn get_contract_storage_proof(
        &self,
        contract_address: &FieldElement,
        keys: Vec<FieldElement>,
    ) -> Result<StorageProof, CoreError> {
        let client =
            reqwest::Client::builder().build().map_err(|e| CoreError::StorageProof(eyre!("build request: {e:?}")))?;
        let keys = keys.iter().map(|i| format!("{i:x}")).collect::<Vec<String>>();
        let addr = format!("{contract_address:x}");
        let block_id = self.get_local_block_num().await;

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

    pub async fn starknet_l1_to_l2_message_cancellations(&self, msg_hash: U256) -> Result<U256, CoreError> {
        let data = L1ToL2MessageCancellationsCall { msg_hash: msg_hash.into() }.encode();
        let call_opts = simple_call_opts(self.core_contract_addr, data.into());

        // Call the StarkNet core contract.
        let call_response =
            self.helios_client.call(&call_opts, BlockTag::Latest).await.map_err(CoreError::FetchL1Val)?;

        Ok(U256::from_big_endian(&call_response))
    }

    pub async fn starknet_l1_to_l2_messages(&self, msg_hash: U256) -> Result<U256, CoreError> {
        let data = L1ToL2MessagesCall { msg_hash: msg_hash.into() }.encode();
        let call_opts = simple_call_opts(self.core_contract_addr, data.into());

        let call_response =
            self.helios_client.call(&call_opts, BlockTag::Latest).await.map_err(CoreError::FetchL1Val)?;

        Ok(U256::from_big_endian(&call_response))
    }

    pub async fn starknet_l2_to_l1_messages(&self, msg_hash: U256) -> Result<U256, CoreError> {
        let data = L2ToL1MessagesCall { msg_hash: msg_hash.into() }.encode();
        let call_opts = simple_call_opts(self.core_contract_addr, data.into());

        // Call the StarkNet core contract.
        let call_response =
            self.helios_client.call(&call_opts, BlockTag::Latest).await.map_err(CoreError::FetchL1Val)?;

        Ok(U256::from_big_endian(&call_response))
    }

    pub async fn starknet_l1_to_l2_message_nonce(&self) -> Result<U256, CoreError> {
        let data = L1ToL2MessageNonceCall::selector();
        let call_opts = simple_call_opts(self.core_contract_addr, data.into());

        let call_response =
            self.helios_client.call(&call_opts, BlockTag::Latest).await.map_err(CoreError::FetchL1Val)?;

        Ok(U256::from_big_endian(&call_response))
    }
}
