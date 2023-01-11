use super::resp::{
    QueryBlockHashAndNumberResponse, QueryBlockNumberResponse, QueryChainIdResponse,
    QueryContractViewResponse, QueryGetBlockTransactionCountResponse, QueryGetClassAtResponse,
    QueryGetClassHashResponse, QueryGetClassResponse, QueryGetStorageAtResponse,
    QueryL1ToL2MessageCancellationsResponse, QueryL1ToL2MessageNonceResponse,
    QueryL1ToL2MessagesResponse, QueryNonceResponse, QueryStateRootResponse,
};
use crate::api::ApiResponse;

use crate::api::starknet::resp::QueryL2ToL1MessagesResponse;
use beerus_core::lightclient::beerus::BeerusLightClient;
use ethers::types::U256;
use eyre::Result;
use log::debug;
use rocket::{get, State};
use rocket_okapi::openapi;
use starknet::core::types::FieldElement;
use std::str::FromStr;

/// Query the state root of StarkNet.
#[openapi]
#[get("/starknet/state/root")]
pub async fn query_starknet_state_root(
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryStateRootResponse> {
    ApiResponse::from_result(query_starknet_state_root_inner(beerus).await)
}

/// Query a contract view.
#[openapi]
#[get("/starknet/view/<contract>/<selector>?<calldata>")]
pub async fn query_starknet_contract_view(
    beerus: &State<BeerusLightClient>,
    contract: String,
    selector: String,
    calldata: Option<String>,
) -> ApiResponse<QueryContractViewResponse> {
    ApiResponse::from_result(
        query_starknet_contract_view_inner(beerus, contract, selector, calldata).await,
    )
}

/// Query get_storage_at.
#[openapi]
#[get("/starknet/storage/<contract>/<key>")]
pub async fn query_starknet_get_storage_at(
    beerus: &State<BeerusLightClient>,
    contract: String,
    key: String,
) -> ApiResponse<QueryGetStorageAtResponse> {
    ApiResponse::from_result(query_starknet_get_storage_at_inner(beerus, contract, key).await)
}

/// Query get_nonce
#[openapi]
#[get("/starknet/nonce/<contract>")]
pub async fn query_starknet_get_nonce(
    beerus: &State<BeerusLightClient>,
    contract: String,
) -> ApiResponse<QueryNonceResponse> {
    ApiResponse::from_result(query_starknet_get_nonce_inner(beerus, contract).await)
}

#[openapi]
#[get("/starknet/chain_id")]
pub async fn query_starknet_chain_id(
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryChainIdResponse> {
    ApiResponse::from_result(query_chain_id_inner(beerus).await)
}

/// Query l1_to_l2_message_cancellations
#[openapi]
#[get("/starknet/messaging/l1_to_l2_message_cancellations/<msg_hash>")]
pub async fn query_l1_to_l2_message_cancellations(
    beerus: &State<BeerusLightClient>,
    msg_hash: String,
) -> ApiResponse<QueryL1ToL2MessageCancellationsResponse> {
    ApiResponse::from_result(query_l1_to_l2_message_cancellations_inner(beerus, msg_hash).await)
}

/// Query l1_to_l2_messages call
#[openapi]
#[get("/starknet/messaging/l1_to_l2_messages/<msg_hash>")]
pub async fn query_l1_to_l2_messages(
    beerus: &State<BeerusLightClient>,
    msg_hash: String,
) -> ApiResponse<QueryL1ToL2MessagesResponse> {
    ApiResponse::from_result(query_l1_to_l2_messages_inner(beerus, msg_hash).await)
}

/// Query the current block number.
#[openapi]
#[get("/starknet/block_number")]
pub async fn query_starknet_block_number(
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryBlockNumberResponse> {
    ApiResponse::from_result(query_starknet_block_number_inner(beerus).await)
}

/// Query the current block hash and number.
#[openapi]
#[get("/starknet/block_hash_and_number")]
pub async fn query_starknet_block_hash_and_number(
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryBlockHashAndNumberResponse> {
    ApiResponse::from_result(query_starknet_block_hash_and_number_inner(beerus).await)
}

/// Query the l2_to_l1_message call
#[openapi]
#[get("/starknet/messaging/l2_to_l1_messages/<msg_hash>")]
pub async fn query_l2_to_l1_messages(
    beerus: &State<BeerusLightClient>,
    msg_hash: String,
) -> ApiResponse<QueryL2ToL1MessagesResponse> {
    ApiResponse::from_result(query_l2_to_l1_messages_inner(beerus, msg_hash).await)
}

/// Query l1_to_l2_message_nonce call
#[openapi]
#[get("/starknet/messaging/l1_to_l2_message_nonce")]
pub async fn query_l1_to_l2_message_nonce(
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryL1ToL2MessageNonceResponse> {
    ApiResponse::from_result(query_l1_to_l2_message_nonce_inner(beerus).await)
}

/// Query the contract class definition in the given block associated with the given hash.
/// The contract class definition.
///
/// # Arguments
///
/// * `block_id_type` - Type of block identifier. eg. hash, number, tag
/// * `block_id` - The block identifier. eg. 0x123, 123, pending, or latest
/// * `class_hash` - The class hash.
///
/// # Returns
///
/// `Ok(ContractClass)` if the operation was successful.
/// `Err(eyre::Report)` if the operation failed.
#[openapi]
#[get("/starknet/contract/class/<class_hash>?<block_id>&<block_id_type>")]
pub async fn get_class(
    beerus: &State<BeerusLightClient>,
    block_id_type: String,
    block_id: String,
    class_hash: String,
) -> ApiResponse<QueryGetClassResponse> {
    ApiResponse::from_result(get_class_inner(beerus, block_id_type, block_id, class_hash).await)
}

/// Query the contract class hash in the given block associated with the contract address.

/// The contract class definition.
///
/// # Arguments
///
/// * `block_id_type` - Type of block identifier. eg. hash, number, tag
/// * `block_id` - The block identifier. eg. 0x123, 123, pending, or latest
/// * `contract_address` - The Contract Address
///
/// # Returns
///
/// `Ok(FieldElement)` if the operation was successful.
/// `Err(eyre::Report)` if the operation failed.
#[openapi]
#[get("/starknet/contract/class_hash/<contract_address>?<block_id>&<block_id_type>")]
pub async fn get_class_hash(
    beerus: &State<BeerusLightClient>,
    block_id_type: String,
    block_id: String,
    contract_address: String,
) -> ApiResponse<QueryGetClassHashResponse> {
    ApiResponse::from_result(
        get_class_hash_inner(beerus, block_id_type, block_id, contract_address).await,
    )
}

/// Query the contract class definition in the given block associated with the contract address.
/// The contract class definition.
///
/// # Arguments
///
/// * `block_id_type` - Type of block identifier. eg. hash, number, tag
/// * `block_id` - The block identifier. eg. 0x123, 123, pending, or latest

/// * `contract_address` - The contract address.
///
/// # Returns
///
/// `Ok(ContractClass)` if the operation was successful.
/// `Err(eyre::Report)` if the operation failed.
#[openapi]
#[get("/starknet/contract/class_at/<contract_address>?<block_id>&<block_id_type>")]
pub async fn get_class_at(
    beerus: &State<BeerusLightClient>,
    block_id_type: String,
    block_id: String,
    contract_address: String,
) -> ApiResponse<QueryGetClassAtResponse> {
    ApiResponse::from_result(
        get_class_at_inner(beerus, block_id_type, block_id, contract_address).await,
    )
}

/// Query the number of transactions in a block given a block id.
/// The number of transactions in a block.
///
/// # Arguments
///
/// * `block_id_type` - Type of block identifier. eg. hash, number, tag
/// * `block_id` - The block identifier. eg. 0x123, 123, pending, or latest
///
/// # Returns
///
/// `Ok(ContractClass)` if the operation was successful.
/// `Err(eyre::Report)` if the operation failed.
#[openapi]
#[get("/starknet/block_transaction_count?<block_id>&<block_id_type>")]
pub async fn get_block_transaction_count(
    beerus: &State<BeerusLightClient>,
    block_id_type: String,
    block_id: String,
) -> ApiResponse<QueryGetBlockTransactionCountResponse> {
    ApiResponse::from_result(
        get_block_transaction_count_inner(beerus, block_id_type, block_id).await,
    )
}

/// Query the state root of StarkNet.
///
/// # Arguments
///
/// * `beerus` - The Beerus light client.
///
/// # Returns
///
/// * `QueryStateRootResponse` - The state root response.
pub async fn query_starknet_state_root_inner(
    beerus: &State<BeerusLightClient>,
) -> Result<QueryStateRootResponse> {
    debug!("Querying StarkNet state root");
    // Call the StarkNet contract to get the state root.
    let state_root = beerus.starknet_state_root().await?;
    Ok(QueryStateRootResponse {
        state_root: state_root.to_string(),
    })
}

/// Query get_storage_at.
///
/// # Arguments
///
/// * `beerus` - The Beerus light client.
/// * `contract_address` - The contract address.
/// * `key` - The key.
///
/// # Returns
///
/// * `QueryGetStorageAtResponse` - The contract view response.
pub async fn query_starknet_get_storage_at_inner(
    beerus: &State<BeerusLightClient>,
    contract_address: String,
    key: String,
) -> Result<QueryGetStorageAtResponse> {
    debug!("Querying StarkNet get_storage_at");
    let contract_address = FieldElement::from_str(&contract_address)?;
    let key = FieldElement::from_str(&key)?;

    // Call the StarkNet contract to get the state root.
    Ok(QueryGetStorageAtResponse {
        result: beerus
            .starknet_get_storage_at(contract_address, key)
            .await?
            .to_string(),
    })
}
/// Query a contract view.
///
/// # Arguments
///
/// * `beerus` - The Beerus light client.
/// * `contract_address` - The contract address.
/// * `selector` - The selector.
/// * `calldata` - The calldata.
///
/// # Returns
///
/// * `QueryContractViewResponse` - The contract view response.
pub async fn query_starknet_contract_view_inner(
    beerus: &State<BeerusLightClient>,
    contract_address: String,
    selector: String,
    calldata: Option<String>,
) -> Result<QueryContractViewResponse> {
    debug!("Querying StarkNet contract view");
    let contract_address = FieldElement::from_str(&contract_address)?;
    let selector = FieldElement::from_str(&selector)?;
    let mut felt_calldata = vec![];
    if let Some(calldata) = calldata {
        let calldata: Vec<&str> = calldata.split(',').into_iter().collect();
        for item in calldata {
            felt_calldata.push(FieldElement::from_str(item)?);
        }
    }

    // Call the StarkNet contract to get the state root.
    let view = beerus
        .starknet_call_contract(contract_address, selector, felt_calldata)
        .await?;

    Ok(QueryContractViewResponse {
        result: view.into_iter().map(|x| x.to_string()).collect(),
    })
}

/// Query get_nonce.
///
/// # Arguments
///
/// * `beerus` - The Beerus light client.
/// * `contract_address` - The contract address.
///
///
/// # Returns
///
/// * `QueryNonceResponse` - The contract nonce response.
pub async fn query_starknet_get_nonce_inner(
    beerus: &State<BeerusLightClient>,
    contract_address: String,
) -> Result<QueryNonceResponse> {
    debug!("Querying Starknet contract nonce");
    let contract_address = FieldElement::from_str(&contract_address)?;

    Ok(QueryNonceResponse {
        result: beerus
            .starknet_get_nonce(contract_address)
            .await?
            .to_string(),
    })
}

/// Query l1_to_l2_message_cancellations.
///
/// # Arguments
///
/// * `beerus` - The Beerus light client.
/// * `msg_hash` - The hash of the message.
///
///
/// # Returns
///
/// * `L1ToL2MessageCancellations` - The timestamp at the time cancelL1ToL2Message was called with a message matching 'msg_hash'.
pub async fn query_l1_to_l2_message_cancellations_inner(
    beerus: &State<BeerusLightClient>,
    msg_hash: String,
) -> Result<QueryL1ToL2MessageCancellationsResponse> {
    debug!("Querying Starknet contract nonce");
    let msg_hash = U256::from_str(&msg_hash)?;

    Ok(QueryL1ToL2MessageCancellationsResponse {
        result: beerus
            .starknet_l1_to_l2_message_cancellations(msg_hash)
            .await?
            .to_string(),
    })
}

/// Query l1_to_l2_messages.
///
/// # Arguments
///
/// * `beerus` - The Beerus light client.
/// * `msg_hash` - The hash of the message.
///
///
/// # Returns
///
/// * `L1ToL2Messages` - The msg_fee + 1 for the message with the given L1ToL2Message hash.

pub async fn query_l1_to_l2_messages_inner(
    beerus: &State<BeerusLightClient>,
    msg_hash: String,
) -> Result<QueryL1ToL2MessagesResponse> {
    debug!("Querying Starknet contract nonce");
    let msg_hash = U256::from_str(&msg_hash)?;

    Ok(QueryL1ToL2MessagesResponse {
        result: beerus
            .starknet_l1_to_l2_messages(msg_hash)
            .await?
            .to_string(),
    })
}

/// Query l2_to_l1_messages.
///
/// # Arguments
///
/// * `beerus` - The Beerus light client.
/// * `msg_hash` - The hash of the message.
///
///
/// # Returns
///
/// * `L2ToL1Messages` - The msg_fee + 1 for the message with the given L2ToL1Message hash.

pub async fn query_l2_to_l1_messages_inner(
    beerus: &State<BeerusLightClient>,
    msg_hash: String,
) -> Result<QueryL2ToL1MessagesResponse> {
    debug!("Querying Starknet contract nonce");
    let msg_hash = U256::from_str(&msg_hash)?;
    Ok(QueryL2ToL1MessagesResponse {
        result: beerus
            .starknet_l2_to_l1_messages(msg_hash)
            .await?
            .to_string(),
    })
}

/// Query the chain ID of the Starknet chain.
/// # Returns
/// `chain_id` - The chain ID.
/// # Errors
/// Cannot fail.
/// # Examples
pub async fn query_chain_id_inner(
    beerus: &State<BeerusLightClient>,
) -> Result<QueryChainIdResponse> {
    debug!("Querying chain ID");
    Ok(QueryChainIdResponse {
        chain_id: beerus.starknet_lightclient.chain_id().await?.to_string(),
    })
}

/// Query the current block number of the Starknet chain.
/// # Returns
/// `block_number` - The current block number.
/// # Errors
/// Cannot fail.
/// # Examples: 123456
pub async fn query_starknet_block_number_inner(
    beerus: &State<BeerusLightClient>,
) -> Result<QueryBlockNumberResponse> {
    debug!("Querying current block number");
    Ok(QueryBlockNumberResponse {
        block_number: beerus
            .starknet_lightclient
            .block_number()
            .await?
            .to_string(),
    })
}

/// Query the current block hash and number of the Starknet chain.
/// # Returns
/// `block_hash` - The current block hash.
/// `block_number` - The current block number.
/// # Errors
/// Cannot fail.
/// # Examples: block_hash: 123456, number: 123456
pub async fn query_starknet_block_hash_and_number_inner(
    beerus: &State<BeerusLightClient>,
) -> Result<QueryBlockHashAndNumberResponse> {
    debug!("Querying current block hash and number");
    let response = beerus.starknet_lightclient.block_hash_and_number().await?;
    Ok(QueryBlockHashAndNumberResponse {
        block_hash: response.block_hash.to_string(),
        block_number: response.block_number.to_string(),
    })
}

/// Query the l1 to l2 message nonce
/// # Returns
/// `nonce` - The nonce.
/// # Errors
/// Cannot fail.
/// # Examples
pub async fn query_l1_to_l2_message_nonce_inner(
    beerus: &State<BeerusLightClient>,
) -> Result<QueryL1ToL2MessageNonceResponse> {
    debug!("Querying l1 to l2 message nonce");
    Ok(QueryL1ToL2MessageNonceResponse {
        result: beerus.starknet_l1_to_l2_message_nonce().await?.to_string(),
    })
}

/// Query the contract class
/// # Returns
/// `ContractClass` - The contract class definition.
pub async fn get_class_inner(
    beerus: &State<BeerusLightClient>,
    block_id_type: String,
    block_id: String,
    class_hash: String,
) -> Result<QueryGetClassResponse> {
    let block_id =
        beerus_core::starknet_helper::block_id_string_to_block_id_type(&block_id_type, &block_id)?;
    let class_hash = FieldElement::from_str(&class_hash)?;
    debug!("Querying Contract Class");
    let result = beerus
        .starknet_lightclient
        .get_class(&block_id, class_hash)
        .await?;
    Ok(QueryGetClassResponse {
        program: base64::encode(&result.program),
        entry_points_by_type: serde_json::value::to_value(&result.entry_points_by_type).unwrap(),
        abi: serde_json::value::to_value(result.abi.unwrap()).unwrap(),
    })
}

/// Query the contract class
/// # Returns
/// `ContractClassHash` - The contract class definition.
pub async fn get_class_hash_inner(
    beerus: &State<BeerusLightClient>,
    block_id_type: String,
    block_id: String,
    contract_address: String,
) -> Result<QueryGetClassHashResponse> {
    let block_id =
        beerus_core::starknet_helper::block_id_string_to_block_id_type(&block_id_type, &block_id)?;
    let contract_address = FieldElement::from_str(&contract_address)?;
    debug!("Querying Contract Class");
    let result = beerus
        .starknet_lightclient
        .get_class_hash_at(&block_id, contract_address)
        .await?;
    Ok(QueryGetClassHashResponse {
        class_hash: result.to_string(),
    })
}

/// Query the contract class
/// # Returns
/// `ContractClass` - The contract class definition.
pub async fn get_class_at_inner(
    beerus: &State<BeerusLightClient>,
    block_id_type: String,
    block_id: String,
    contract_address: String,
) -> Result<QueryGetClassAtResponse> {
    let block_id =
        beerus_core::starknet_helper::block_id_string_to_block_id_type(&block_id_type, &block_id)?;
    let contract_address = FieldElement::from_str(&contract_address)?;
    debug!("Querying Contract Class");
    let result = beerus
        .starknet_lightclient
        .get_class_at(&block_id, contract_address)
        .await?;
    Ok(QueryGetClassAtResponse {
        program: base64::encode(&result.program),
        entry_points_by_type: serde_json::value::to_value(&result.entry_points_by_type).unwrap(),
        abi: serde_json::value::to_value(result.abi.unwrap()).unwrap(),
    })
}

/// Query the number of transactions in a block given a block id.
/// # Returns
/// `block_transaction_count` - The number of transactions in a block.
pub async fn get_block_transaction_count_inner(
    beerus: &State<BeerusLightClient>,
    block_id_type: String,
    block_id: String,
) -> Result<QueryGetBlockTransactionCountResponse> {
    let block_id =
        beerus_core::starknet_helper::block_id_string_to_block_id_type(&block_id_type, &block_id)?;
    debug!("Querying block transaction count");
    Ok(QueryGetBlockTransactionCountResponse {
        block_transaction_count: beerus
            .starknet_lightclient
            .get_block_transaction_count(&block_id)
            .await?
            .to_string(),
    })
}
