use std::str::FromStr;

use crate::model::CommandResponse;
use beerus_core::lightclient::beerus::BeerusLightClient;
use ethers::types::U256;
use eyre::Result;
use serde::{Deserialize, Serialize};
use starknet::{
    core::types::FieldElement,
    providers::jsonrpc::models::{
        BroadcastedDeclareTransaction, BroadcastedDeployTransaction, BroadcastedInvokeTransaction,
        BroadcastedInvokeTransactionV0, EventFilter,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventsObject {
    pub from_block_id_type: Option<String>,
    pub from_block_id: Option<String>,
    pub to_block_id_type: Option<String>,
    pub to_block_id: Option<String>,
    pub address: Option<String>,
    pub keys: Option<Vec<String>>,
    pub continuation_token: Option<String>,
    pub chunk_size: u64,
}

/// Query the StarkNet state root.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// # Returns
/// * `Result<CommandResponse>` - The result of the query.
/// # Errors
/// * If the StarkNet state root query fails.
pub async fn query_starknet_state_root(beerus: BeerusLightClient) -> Result<CommandResponse> {
    // Call the StarkNet contract to get the state root.
    Ok(CommandResponse::StarkNetQueryStateRoot(
        beerus
            .ethereum_lightclient
            .read()
            .await
            .starknet_state_root()
            .await?,
    ))
}

/// Query starknet_storageAt
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `address` - The StarkNet address.
/// * `slot` - The StarkNet slot.
/// # Returns
/// * `Result<()>` - The result of the query.
/// # Errors
/// * If the StarkNet storageAt query fails.
/// * If the StarkNet address is invalid.
/// * If the StarkNet slot is invalid.
pub async fn query_starknet_get_storage_at(
    beerus: BeerusLightClient,
    address: String,
    slot: String,
) -> Result<CommandResponse> {
    // Convert address to FieldElement.
    let address = FieldElement::from_str(&address)?;
    // Convert slot to FieldElement.
    let slot = FieldElement::from_str(&slot)?;

    // Call the StarkNet contract to get the state root.
    Ok(CommandResponse::StarkNetQueryGetStorageAt(
        beerus.starknet_get_storage_at(address, slot).await?,
    ))
}

/// Query a StarkNet contract view.
/// WARNING: This is a very unsafe function. It is not recommended to use it.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `address` - The StarkNet address.
/// * `selector` - The StarkNet selector.
/// * `calldata` - The StarkNet calldata.
/// # Returns
/// * `Result<()>` - The result of the query.
/// # Errors
/// * If the StarkNet contract view query fails.
/// * If the StarkNet address is invalid.
/// * If the StarkNet selector is invalid.
/// * If the StarkNet calldata is invalid.
pub async fn query_starknet_contract_view(
    beerus: BeerusLightClient,
    address: String,
    selector: String,
    calldata: Vec<String>,
) -> Result<CommandResponse> {
    // Convert address to FieldElement.
    let address = FieldElement::from_str(&address)?;
    // Convert selector to FieldElement.
    let selector = FieldElement::from_str(&selector)?;
    // Convert calldata to FieldElements.
    let calldata = calldata
        .iter()
        .map(|x| FieldElement::from_str(x).unwrap())
        .collect();

    // Call the StarkNet contract to get the state root.
    Ok(CommandResponse::StarkNetQueryContract(
        beerus
            .starknet_call_contract(address, selector, calldata)
            .await?,
    ))
}

/// Query starknet_nonce
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `address` - The StarkNet address.
/// # Returns
/// * `Result<()>` - The result of the query.
/// # Errors
/// * If the StarkNet nonce query fails.
/// * If the StarkNet address is invalid.
pub async fn query_starknet_nonce(
    beerus: BeerusLightClient,
    address: String,
) -> Result<CommandResponse> {
    let addr = FieldElement::from_str(&address)?;

    Ok(CommandResponse::StarkNetQueryNonce(
        beerus.starknet_get_nonce(addr).await?,
    ))
}

/// Query L1 to L2 messages cancellation timestamp.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `msg_hash` - The message hash.
/// # Returns
/// * `Result<CommandResponse>` - The result of the query.
/// # Errors
/// * If the L1 to L2 messages cancellation timestamp query fails.
/// * If the message hash is invalid.
pub async fn query_starknet_l1_to_l2_messages_cancellation_timestamp(
    beerus: BeerusLightClient,
    msg_hash: String,
) -> Result<CommandResponse> {
    let msg_hash = U256::from_str(&msg_hash)?;
    Ok(CommandResponse::StarkNetL1ToL2MessageCancellations(
        beerus
            .starknet_l1_to_l2_message_cancellations(msg_hash)
            .await?,
    ))
}

/// Query L1 to L2 the msg_fee + 1 for the message with the given 'msgHash'
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `msg_hash` - The message hash.
/// # Returns
/// * `Result<CommandResponse>` - The result of the query.
/// # Errors
/// * If the L1 to L2 messages query fails.
/// * If the message hash is invalid.
pub async fn query_starknet_l1_to_l2_messages(
    beerus: BeerusLightClient,
    msg_hash: String,
) -> Result<CommandResponse> {
    let msg_hash = U256::from_str(&msg_hash)?;
    Ok(CommandResponse::StarkNetL1ToL2Messages(
        beerus.starknet_l1_to_l2_messages(msg_hash).await?,
    ))
}

/// Query (msg_fee+1) for the L2 to L1 message with the given 'msgHash'
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `msg_hash` - The message hash.
/// # Returns
/// * `Result<CommandResponse>` - The result of the query.
/// # Errors
/// * If the L2 to L1 messages query fails.
/// * If the message hash is invalid.
pub async fn query_starknet_l2_to_l1_messages(
    beerus: BeerusLightClient,
    msg_hash: String,
) -> Result<CommandResponse> {
    let msg_hash = U256::from_str(&msg_hash)?;
    Ok(CommandResponse::StarkNetL2ToL1Messages(
        beerus.starknet_l2_to_l1_messages(msg_hash).await?,
    ))
}

/// Query l1 to l2 message nonce
/// # Arguments
/// * `beerus` - The Beerus light client.
/// # Returns
/// * `Result<CommandResponse>` - The result of the query.
/// # Errors
/// * If the StarkNet nonce query fails.
pub async fn query_starknet_l1_to_l2_message_nonce(
    beerus: BeerusLightClient,
) -> Result<CommandResponse> {
    Ok(CommandResponse::StarkNetL1ToL2MessageNonce(
        beerus.starknet_l1_to_l2_message_nonce().await?,
    ))
}

/// Query the chain id of the StarkNet network.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// # Returns
/// * `Result<CommandResponse>` - The chain id of the StarkNet network.
pub async fn query_chain_id(beerus: BeerusLightClient) -> Result<CommandResponse> {
    let chain_id = beerus.starknet_lightclient.chain_id().await?;
    Ok(CommandResponse::StarknetQueryChainId(chain_id))
}

/// Query the current block number of the StarkNet network.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// # Returns
/// * `Result<CommandResponse>` - The current block number of the StarkNet network.
pub async fn query_block_number(beerus: BeerusLightClient) -> Result<CommandResponse> {
    let block_number = beerus.starknet_lightclient.block_number().await?;
    Ok(CommandResponse::StarknetQueryBlockNumber(block_number))
}

/// Query the current block hash and number of the StarkNet network.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// # Returns
/// * `Result<CommandResponse>` - The current block hash and number of the StarkNet network.
pub async fn query_block_hash_and_number(beerus: BeerusLightClient) -> Result<CommandResponse> {
    Ok(CommandResponse::StarknetQueryBlockHashAndNumber(
        beerus.starknet_lightclient.block_hash_and_number().await?,
    ))
}

/// Query the contract class definition in the given block associated with the given hash.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `block_id_type` - The type of block identifier.
/// * `block_id` - The block identifier.
/// * `class_hash` - The class hash.
/// # Returns
/// * `Result<CommandResponse>` - The contract class definition.
pub async fn get_class(
    beerus: BeerusLightClient,
    block_id_type: String,
    block_id: String,
    class_hash: String,
) -> Result<CommandResponse> {
    let block_id =
        beerus_core::starknet_helper::block_id_string_to_block_id_type(&block_id_type, &block_id)?;
    let class_hash = FieldElement::from_str(&class_hash)?;
    Ok(CommandResponse::StarknetQueryGetClass(
        beerus
            .starknet_lightclient
            .get_class(&block_id, class_hash)
            .await?,
    ))
}

/// Query the contract class hash given block associated with the contract address.

/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `block_id_type` - The type of block identifier.
/// * `block_id` - The block identifier.
/// * `contract_address` - The class hash.
/// # Returns
/// * `Result<CommandResponse>` - The contract class definition.
pub async fn get_class_hash(
    beerus: BeerusLightClient,
    block_id_type: String,
    block_id: String,
    contract_address: String,
) -> Result<CommandResponse> {
    let block_id =
        beerus_core::starknet_helper::block_id_string_to_block_id_type(&block_id_type, &block_id)?;
    let contract_address = FieldElement::from_str(&contract_address)?;
    Ok(CommandResponse::StarknetQueryGetClassHash(
        beerus
            .starknet_lightclient
            .get_class_hash_at(&block_id, contract_address)
            .await?,
    ))
}

/// Query the contract class definition in the given block associated with the contract address.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `block_id_type` - The type of block identifier.
/// * `block_id` - The block identifier.
/// * `contract_address` - The class hash.
/// # Returns
/// * `Result<CommandResponse>` - The contract class definition.

pub async fn get_class_at(
    beerus: BeerusLightClient,
    block_id_type: String,
    block_id: String,
    contract_address: String,
) -> Result<CommandResponse> {
    let block_id =
        beerus_core::starknet_helper::block_id_string_to_block_id_type(&block_id_type, &block_id)?;
    let contract_address = FieldElement::from_str(&contract_address)?;

    Ok(CommandResponse::StarknetQueryGetClass(
        beerus
            .starknet_lightclient
            .get_class_at(&block_id, contract_address)
            .await?,
    ))
}

/// Query the number of transactions in a block given a block id of the StarkNet network.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `block_id_type` - The type of block identifier.
/// * `block_id` - The block identifier.
/// # Returns
/// * `Result<CommandResponse>` - The number of transactions in a block.
pub async fn get_block_transaction_count(
    beerus: BeerusLightClient,
    block_id_type: String,
    block_id: String,
) -> Result<CommandResponse> {
    let block_id =
        beerus_core::starknet_helper::block_id_string_to_block_id_type(&block_id_type, &block_id)?;
    Ok(CommandResponse::StarknetQueryGetBlockTransactionCount(
        beerus
            .starknet_lightclient
            .get_block_transaction_count(&block_id)
            .await?,
    ))
}

/// Query information about the result of executing the requested block.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `block_id` - The block identifier.
/// # Returns
/// * `Result<CommandResponse>` - The state update.
pub async fn get_state_update(
    beerus: BeerusLightClient,
    block_id_type: String,
    block_id: String,
) -> Result<CommandResponse> {
    let block_id =
        beerus_core::starknet_helper::block_id_string_to_block_id_type(&block_id_type, &block_id)?;
    Ok(CommandResponse::StarknetQueryGetStateUpdate(
        beerus
            .starknet_lightclient
            .get_state_update(&block_id)
            .await?,
    ))
}

/// Query events on the StarkNet network.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `params` - The query filters.
/// # Returns
/// * `Result<CommandResponse>` - The events.
pub async fn get_events(beerus: BeerusLightClient, params: String) -> Result<CommandResponse> {
    let events_object: EventsObject = serde_json::from_str(&params)?;

    let from_block = match (
        events_object.from_block_id_type,
        events_object.from_block_id,
    ) {
        (Some(from_block_id_type_str), Some(from_block_id_str)) => {
            let result = beerus_core::starknet_helper::block_id_string_to_block_id_type(
                &from_block_id_type_str,
                &from_block_id_str,
            );
            Some(result?)
        }
        _ => None,
    };

    let to_block = match (events_object.to_block_id_type, events_object.to_block_id) {
        (Some(to_block_id_type_str), Some(to_block_id_str)) => {
            let result = beerus_core::starknet_helper::block_id_string_to_block_id_type(
                &to_block_id_type_str,
                &to_block_id_str,
            );
            Some(result?)
        }
        _ => None,
    };

    let address = match events_object.address {
        Some(address_str) => Some(FieldElement::from_str(&address_str)?),
        _ => None,
    };

    let keys = events_object.keys.as_ref().map(|keys| {
        keys.iter()
            .map(|s| FieldElement::from_str(s).unwrap())
            .collect()
    });

    let filter = EventFilter {
        from_block,
        to_block,
        address,
        keys,
    };

    Ok(CommandResponse::StarknetQueryGetEvents(
        beerus
            .starknet_lightclient
            .get_events(
                filter,
                events_object.continuation_token,
                events_object.chunk_size,
            )
            .await?,
    ))
}

/// Query if the node is synchronized on the StarkNet network.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// # Returns
/// * `Result<CommandResponse>` - If the node is synchronized on the StarkNet network.
pub async fn query_starknet_syncing(beerus: BeerusLightClient) -> Result<CommandResponse> {
    Ok(CommandResponse::StarknetQuerySyncing(
        beerus.starknet_lightclient.syncing().await?,
    ))
}

/// Query the estimated fee for the broadcasted transaction.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `block_id_type` - The type of block identifier.
/// * `block_id` - The block identifier.
/// * `broadcasted_transaction` - The broadcasted transaction to be estimated
/// # Returns
/// * `Result<CommandResponse>` - The estimated gas fee
pub async fn query_starknet_estimate_fee(
    beerus: BeerusLightClient,
    block_id: String,
    block_id_type: String,
    broadcasted_transaction: String,
) -> Result<CommandResponse> {
    let block_id =
        beerus_core::starknet_helper::block_id_string_to_block_id_type(&block_id_type, &block_id)?;
    let tx = serde_json::from_str(broadcasted_transaction.as_str())?;
    Ok(CommandResponse::StarknetQueryEstimateFee(
        beerus
            .starknet_lightclient
            .estimate_fee(tx, &block_id)
            .await?,
    ))
}

/// Add an Invoke transaction to the StarkNet network.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `max_fee` - The maximum fee.
/// * `signature` - The signature.
/// * `nonce` - The nonce.
/// * `contract_address` - The contract address.
/// * `entry_point_selector` - The entry point selector.
/// * `calldata` - The calldata.
///
/// # Returns
///
/// * `Result<CommandResponse>` - If the node is synchronized on the StarkNet network.
pub async fn add_invoke_transaction(
    beerus: BeerusLightClient,
    max_fee: String,
    signature: Vec<String>,
    nonce: String,
    contract_address: String,
    entry_point_selector: String,
    calldata: Vec<String>,
) -> Result<CommandResponse> {
    let max_fee: FieldElement = FieldElement::from_str(&max_fee).unwrap();
    let signature = signature
        .iter()
        .map(|x| FieldElement::from_str(x).unwrap())
        .collect();
    let nonce: FieldElement = FieldElement::from_str(&nonce).unwrap();
    let contract_address: FieldElement = FieldElement::from_str(&contract_address).unwrap();
    let entry_point_selector: FieldElement = FieldElement::from_str(&entry_point_selector).unwrap();
    let calldata = calldata
        .iter()
        .map(|x| FieldElement::from_str(x).unwrap())
        .collect();

    let transaction_data = BroadcastedInvokeTransactionV0 {
        max_fee,
        signature,
        nonce,
        contract_address,
        entry_point_selector,
        calldata,
    };

    let invoke_transaction = BroadcastedInvokeTransaction::V0(transaction_data);

    Ok(CommandResponse::StarknetAddInvokeTransaction(
        beerus
            .starknet_lightclient
            .add_invoke_transaction(&invoke_transaction)
            .await?,
    ))
}

/// Add an Deploy transaction to the StarkNet network.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `contract_class` - The contract class.
/// * `version` - The version.
/// * `contract_address_salt` - The contract address salt.
/// * `constructor_calldata` - The constructor calldata.
///
/// # Returns
///
/// * `Result<CommandResponse>` - The deploy transaction.
pub async fn add_deploy_transaction(
    beerus: BeerusLightClient,
    contract_class: String,
    version: String,
    contract_address_salt: String,
    constructor_calldata: Vec<String>,
) -> Result<CommandResponse> {
    let contract_class_bytes = contract_class.as_bytes();
    let contract_class = serde_json::from_slice(contract_class_bytes)?;
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

    Ok(CommandResponse::StarknetAddDeployTransaction(
        beerus
            .starknet_lightclient
            .add_deploy_transaction(&deploy_transaction)
            .await?,
    ))
}

/// Query Block with Txs array
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `block_id_type` - The type of block identifier.
/// * `block_id` - The block identifier.
/// # Returns
/// * `Result<CommandResponse>` - The contract class definition.
pub async fn query_block_with_txs(
    beerus: BeerusLightClient,
    block_id_type: String,
    block_id: String,
) -> Result<CommandResponse> {
    let block_id =
        beerus_core::starknet_helper::block_id_string_to_block_id_type(&block_id_type, &block_id)?;
    Ok(CommandResponse::StarknetQueryBlockWithTxs(
        beerus
            .starknet_lightclient
            .get_block_with_txs(&block_id)
            .await?,
    ))
}

/// Query a transaction by its hash.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `hash` - The hash, in hex-string format.
/// # Returns
/// * `Result<CommandResponse>` - The matching transaction, if it exists.
pub async fn get_transaction_by_hash(
    beerus: BeerusLightClient,
    hash: String,
) -> Result<CommandResponse> {
    let tx_hash = FieldElement::from_str(&hash);
    Ok(CommandResponse::StarknetQueryTransactionByHash(
        beerus
            .starknet_lightclient
            .get_transaction_by_hash(tx_hash?)
            .await?,
    ))
}
/// Query the number of transactions in a block given a block id of the StarkNet network.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `block_id_type` - The type of block identifier.
/// * `block_id` - The block identifier.
/// # Returns
/// * `Result<CommandResponse>` - The number of transactions in a block.
pub async fn get_transaction_by_block_id_and_index(
    beerus: BeerusLightClient,
    block_id_type: String,
    block_id: String,
    index: String,
) -> Result<CommandResponse> {
    let index = u64::from_str(&index)?;
    let block_id =
        beerus_core::starknet_helper::block_id_string_to_block_id_type(&block_id_type, &block_id)?;
    Ok(CommandResponse::StarknetQueryTransactionByBlockIdAndIndex(
        beerus
            .starknet_lightclient
            .get_transaction_by_block_id_and_index(&block_id, index)
            .await?,
    ))
}

/// Query the number of transactions in a block given a block id of the StarkNet network.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `block_id_type` - The type of block identifier.
/// * `block_id` - The block identifier.
/// # Returns
/// * `Result<CommandResponse>` - The number of transactions in a block.
pub async fn query_pending_transactions(beerus: BeerusLightClient) -> Result<CommandResponse> {
    Ok(CommandResponse::StarknetQueryPendingTransactions(
        beerus.starknet_lightclient.pending_transactions().await?,
    ))
}

/// Query Block with Txs Hashes
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `block_id_type` - The type of block identifier.
/// * `block_id` - The block identifier.
/// # Returns
/// * `Result<CommandResponse>` - The contract class definition.
pub async fn query_block_with_tx_hashes(
    beerus: BeerusLightClient,
    block_id_type: String,
    block_id: String,
) -> Result<CommandResponse> {
    let block_id =
        beerus_core::starknet_helper::block_id_string_to_block_id_type(&block_id_type, &block_id)?;
    Ok(CommandResponse::StarknetQueryBlockWithTxHashes(
        beerus
            .starknet_lightclient
            .get_block_with_tx_hashes(&block_id)
            .await?,
    ))
}

/// Query Tx Receipt
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `hash` - The transaction's hash, as a hex-string.
/// # Returns
/// * `Result<CommandResponse>` - The receipt.
pub async fn query_tx_receipt(beerus: BeerusLightClient, hash: String) -> Result<CommandResponse> {
    let hash = FieldElement::from_str(&hash)?;
    Ok(CommandResponse::StarknetQueryTxReceipt(
        beerus
            .starknet_lightclient
            .get_transaction_receipt(hash)
            .await?,
    ))
}

/// Query Contract Storage proof for a given contract and keys
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `block_id_type` - The type of block identifier.
/// * `block_id` - The block identifier.
/// * `contract_address` - The contract address.
/// * `keys` - The contract's storage slots.
/// # Returns
/// * `Result<CommandResponse>` - The contract & keys storage proofs
pub async fn query_contract_storage_proof(
    beerus: BeerusLightClient,
    block_id_type: String,
    block_id: String,
    contract_address: String,
    keys: &[String],
) -> Result<CommandResponse> {
    let block_id =
        beerus_core::starknet_helper::block_id_string_to_block_id_type(&block_id_type, &block_id)?;
    let contract_address = FieldElement::from_str(&contract_address)?;
    let keys: Result<Vec<FieldElement>, _> =
        keys.iter().map(|k| FieldElement::from_str(k)).collect();

    let proof = beerus
        .starknet_lightclient
        .get_contract_storage_proof(contract_address, keys?, &block_id)
        .await?;

    Ok(CommandResponse::StarknetQueryContractStorageProof(proof))
}

/// Add an Declare transaction to the StarkNet network.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `max_fee` - The maximum fee.
/// * `signature` - The signature.
/// * `nonce` - The nonce.
/// * `contract_class` - The contract class.
/// * `entry_point_selector` - The entry point selector.
/// * `calldata` - The calldata.
///
/// # Returns
///
/// * `Result<CommandResponse>` - If the node is synchronized on the StarkNet network.
pub async fn add_declare_transaction(
    beerus: BeerusLightClient,
    version: String,
    max_fee: String,
    signature: Vec<String>,
    nonce: String,
    contract_class: String,
    sender_address: String,
) -> Result<CommandResponse> {
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

    Ok(CommandResponse::StarknetAddDeclareTransaction(
        beerus
            .starknet_lightclient
            .add_declare_transaction(&declare_transaction)
            .await?,
    ))
}
