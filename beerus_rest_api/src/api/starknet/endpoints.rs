use super::resp::{
    QueryContractViewResponse, QueryGetStorageAtResponse, QueryL1ToL2MessageCancellationsResponse,
    QueryL1ToL2MessagesResponse, QueryNonceResponse, QueryStateRootResponse,
};
use crate::api::ApiResponse;

use crate::api::starknet::resp::QueryChainIdResponse;
use beerus_core::lightclient::beerus::BeerusLightClient;
use eyre::Result;
use log::debug;
use primitive_types::U256;
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
