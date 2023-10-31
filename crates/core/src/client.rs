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
use starknet::core::types::{
    BlockHashAndNumber, BlockId, BlockTag as SnBlockTag, BroadcastedDeclareTransaction,
    BroadcastedDeployAccountTransaction, BroadcastedInvokeTransaction, BroadcastedTransaction, ContractClass,
    DeclareTransactionResult, DeployAccountTransactionResult, EventFilter, EventsPage, FeeEstimate, FieldElement,
    FunctionCall, InvokeTransactionResult, MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs,
    MaybePendingStateUpdate, MaybePendingTransactionReceipt, MsgFromL1, SyncStatusType, Transaction,
};
use starknet::macros::selector;
use starknet::providers::jsonrpc::{HttpTransport, HttpTransportError, JsonRpcClient, JsonRpcClientError};
use starknet::providers::ProviderError::StarknetError;
use starknet::providers::{AnyProviderError, Provider, ProviderError, StarknetErrorWithMessage};
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
    /// Ethereum light client
    #[cfg(not(target_arch = "wasm32"))]
    helios_client: Arc<Client<FileDB>>,
    #[cfg(target_arch = "wasm32")]
    helios_client: Arc<Client<ConfigDB>>,
    /// StarkNet light client
    starknet_client: JsonRpcClient<HttpTransport>,
    /// Proof client
    pub proof_addr: String,
    /// Poll interval
    pub poll_secs: u64,
    /// StarkNet core contract address
    pub core_contract_addr: Address,
    /// Payload data
    pub node: Arc<RwLock<NodeData>>,
    /// Beerus client config
    pub config: Config,
}

impl BeerusClient {
    /// Create a new Beerus Light Client service.
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
                    // TODO: Fetch and hash the headers for the amout of blocks l1 is behind l2
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

    pub async fn get_local_block_id(&self, provided_block_id: BlockId) -> BlockId {
        let local_block_num = self.get_local_block_num().await;

        // allow for historical block ids
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

    pub async fn get_contract_storage_proof(
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

    // Convert the response bytes to a U256.
    Ok(U256::from_big_endian(&sn_block_num).as_u64())
}

type StarknetErr = ProviderError<JsonRpcClientError<HttpTransportError>>;
impl BeerusClient {
    // ------------------- Starknet Provider Endpoints -------------------
    //
    pub async fn starknet_get_block_with_tx_hashes(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxHashes, StarknetErr> {
        let l1_block_num = self.get_local_block_id(block_id).await;
        self.starknet_client.get_block_with_tx_hashes(l1_block_num).await
    }

    pub async fn starknet_get_block_with_txs(
        &self,
        block_id: BlockId,
    ) -> Result<MaybePendingBlockWithTxs, StarknetErr> {
        let l1_block_num = self.get_local_block_id(block_id).await;
        self.starknet_client.get_block_with_txs(l1_block_num).await
    }

    pub async fn starknet_get_state_update(&self, block_id: BlockId) -> Result<MaybePendingStateUpdate, StarknetErr> {
        let l1_block_num = self.get_local_block_id(block_id).await;
        self.starknet_client.get_state_update(l1_block_num).await
    }

    pub async fn starknet_get_storage_at(
        &self,
        contract_address: FieldElement,
        key: FieldElement,
        block_id: BlockId,
    ) -> Result<FieldElement, StarknetErr> {
        let l1_block_num = self.get_local_block_id(block_id).await;
        let fetched_val = self.starknet_client.get_storage_at(contract_address, key, l1_block_num).await?;
        let mut proof =
            self.get_contract_storage_proof(block_id, contract_address.as_ref(), &[*key.as_ref()]).await.unwrap();

        let l1_root = self.get_local_root().await;
        proof.verify(l1_root, *contract_address.as_ref(), *key.as_ref(), fetched_val).unwrap();

        Ok(fetched_val)
    }

    pub async fn starknet_get_transaction_by_hash(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<Transaction, StarknetErr> {
        self.starknet_client.get_transaction_by_hash(transaction_hash).await
    }

    pub async fn starknet_get_transaction_by_block_id_and_index(
        &self,
        block_id: BlockId,
        index: u64,
    ) -> Result<Transaction, StarknetErr> {
        let l1_block_num = self.get_local_block_id(block_id).await;
        self.starknet_client.get_transaction_by_block_id_and_index(l1_block_num, index).await
    }

    pub async fn starknet_get_transaction_receipt(
        &self,
        transaction_hash: FieldElement,
    ) -> Result<MaybePendingTransactionReceipt, StarknetErr> {
        self.starknet_client.get_transaction_receipt(transaction_hash).await
    }

    pub async fn starknet_get_class(
        &self,
        block_id: BlockId,
        class_hash: FieldElement,
    ) -> Result<ContractClass, StarknetErr> {
        let l1_block_num = self.get_local_block_id(block_id).await;
        self.starknet_client.get_class(l1_block_num, class_hash).await
    }

    pub async fn starknet_get_class_hash_at(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, StarknetErr> {
        let l1_block_num = self.get_local_block_id(block_id).await;
        self.starknet_client.get_class_hash_at(l1_block_num, contract_address).await
    }

    pub async fn starknet_get_class_at(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<ContractClass, StarknetErr> {
        let l1_block_num = self.get_local_block_id(block_id).await;
        self.starknet_client.get_class_at(l1_block_num, contract_address).await
    }

    pub async fn starknet_get_block_transaction_count(&self, block_id: BlockId) -> Result<u64, StarknetErr> {
        let l1_block_num = self.get_local_block_id(block_id).await;
        self.starknet_client.get_block_transaction_count(l1_block_num).await
    }

    pub async fn starknet_call(
        &self,
        request: FunctionCall,
        block_id: BlockId,
    ) -> Result<Vec<FieldElement>, StarknetErr> {
        let l1_block_num = self.get_local_block_id(block_id).await;
        self.starknet_client.call(request, l1_block_num).await
    }

    pub async fn starknet_estimate_fee(
        &self,
        request: BroadcastedTransaction,
        block_id: BlockId,
    ) -> Result<Vec<FeeEstimate>, StarknetErr> {
        let l1_block_num = self.get_local_block_id(block_id).await;
        self.starknet_client.estimate_fee(vec![request], l1_block_num).await
    }

    pub async fn starknet_estimate_message_fee(
        &self,
        message: MsgFromL1,
        block_id: BlockId,
    ) -> Result<FeeEstimate, StarknetErr> {
        let l1_block_num = self.get_local_block_id(block_id).await;
        self.starknet_client.estimate_message_fee(message, l1_block_num).await
    }

    pub async fn starknet_block_number(&self) -> Result<u64, StarknetErr> {
        Ok(self.get_local_block_num().await)
    }

    pub async fn starknet_block_hash_and_number(&self) -> Result<BlockHashAndNumber, CoreError> {
        let block_hash = self.sn_state_block_hash().await?;
        let block_number = self.sn_state_block_number().await?;
        Ok(BlockHashAndNumber { block_hash, block_number })
    }

    pub async fn starknet_chain_id(&self) -> Result<FieldElement, StarknetErr> {
        self.starknet_client.chain_id().await
    }

    pub async fn starknet_pending_transactions(&self) -> Result<Vec<Transaction>, StarknetErr> {
        self.starknet_client.pending_transactions().await
    }

    pub async fn starknet_syncing(&self) -> Result<SyncStatusType, StarknetErr> {
        self.starknet_client.syncing().await
    }

    pub async fn starknet_get_events(
        &self,
        filter: EventFilter,
        continuation_token: Option<String>,
        chunk_size: u64,
    ) -> Result<EventsPage, StarknetErr> {
        self.starknet_client.get_events(filter, continuation_token, chunk_size).await
    }

    pub async fn starknet_get_nonce(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<FieldElement, StarknetErr> {
        let l1_block_num = self.get_local_block_id(block_id).await;
        self.starknet_client.get_nonce(l1_block_num, contract_address).await
    }

    pub async fn starknet_add_invoke_transaction(
        &self,
        invoke_transaction: BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult, StarknetErr> {
        self.starknet_client.add_invoke_transaction(invoke_transaction).await
    }

    pub async fn starknet_add_declare_transaction(
        &self,
        declare_transaction: BroadcastedDeclareTransaction,
    ) -> Result<DeclareTransactionResult, StarknetErr> {
        self.starknet_client.add_declare_transaction(declare_transaction).await
    }

    pub async fn starknet_add_deploy_account_transaction(
        &self,
        deploy_account_transaction: BroadcastedDeployAccountTransaction,
    ) -> Result<DeployAccountTransactionResult, StarknetErr> {
        self.starknet_client.add_deploy_account_transaction(deploy_account_transaction).await
    }

    pub async fn starknet_estimate_fee_single(
        &self,
        request: BroadcastedTransaction,
        block_id: BlockId,
    ) -> Result<FeeEstimate, StarknetErr> {
        let l1_block_num = self.get_local_block_id(block_id).await;
        self.starknet_client.estimate_fee_single(request, l1_block_num).await
    }

    // ------------------- Extended Starknet Provider Endpoints -------------------
    //
    pub async fn starknet_get_contract_storage_proof(
        &self,
        block_id: BlockId,
        contract_address: FieldElement,
        keys: Vec<FieldElement>,
    ) -> Result<StorageProof, CoreError> {
        self.get_contract_storage_proof(block_id, &contract_address, &keys).await
    }

    pub async fn starknet_proven_state_root(&self) -> Result<FieldElement> {
        self.sn_state_root().await
    }

    pub async fn starknet_proven_block_number(&self) -> Result<u64, CoreError> {
        self.sn_state_block_number().await
    }
    //     pub async fn starknet_get_balance(
    //         &self,
    //         block_id: BlockId,
    //         contract_address: FieldElement,
    //     ) -> Result<FieldElement, CoreError> { // get local block number and root to verify proof
    //       with let l1_block_num = self.get_local_block_id(block_id).await; let root =
    //       self.get_local_root().await;
    //
    //         // get the storage key for the queried contract address
    //         let balance_key = get_balance_key(contract_address);
    //
    //         // get the proof for the contracts erc20 balance in the fee token contract
    //         let mut proof = self
    //             .get_contract_storage_proof(block_id, &self.config.fee_token_addr, &[balance_key])
    //             .await
    //             ?;
    //
    //         // call the untrusted RPC for the value to check via the storage proof
    //         let balance = self
    //             .starknet_client
    //             .call(
    //                 FunctionCall {
    //                     contract_address: self.config.fee_token_addr,
    //                     entry_point_selector: selector!("balanceOf"),
    //                     calldata: vec![contract_address],
    //                 },
    //                 l1_block_num,
    //             )
    //             .await
    //             .map_err(CoreError::FetchL1Val); // todo
    //
    //         // verify the storage proof w/ the untrusted value
    //         proof.verify(root, self.config.fee_token_addr, balance_key,
    // balance[0]).map_err(CoreError::FetchL1Val);
    //
    //         Ok(balance[0])
    //     }
}
