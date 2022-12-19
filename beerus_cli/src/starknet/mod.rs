use std::str::FromStr;

use beerus_core::lightclient::beerus::BeerusLightClient;
use eyre::Result;
use primitive_types::U256;
use starknet::core::types::FieldElement;

use crate::model::CommandResponse;

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
        beerus.starknet_state_root().await?,
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
