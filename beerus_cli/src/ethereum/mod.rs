use beerus_core::lightclient::beerus::BeerusLightClient;
use ethers::{types::Address, utils};
use eyre::Result;
use helios::types::BlockTag;
use std::str::FromStr;

use crate::model::CommandResponse;

/// Query the balance of an Ethereum address.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `address` - The Ethereum address.
/// # Returns
/// * `Result<CommandResponse>` - The balance of the Ethereum address.
/// # Errors
/// * If the Ethereum address is invalid.
/// * If the balance query fails.
pub async fn query_balance(beerus: BeerusLightClient, address: String) -> Result<CommandResponse> {
    // Parse the Ethereum address.
    let addr = Address::from_str(&address)?;

    // TODO: Make the block tag configurable.
    let block = BlockTag::Latest;
    // Query the balance of the Ethereum address.
    let balance = beerus
        .ethereum_lightclient
        .get_balance(&addr, block)
        .await?;
    // Format the balance in Ether.
    let balance_in_eth = utils::format_units(balance, "ether")?;
    Ok(CommandResponse::EthereumQueryBalance(balance_in_eth))
}

/// Query the nonce of an Ethereum address.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `address` - The Ethereum address.
/// # Returns
/// * `Result<CommandResponse>` - The nonce of the address address.
/// # Errors
/// * If the Ethereum address is invalid.
/// * If the nonce query fails.
pub async fn query_nonce(beerus: BeerusLightClient, address: String) -> Result<CommandResponse> {
    // Parse the Ethereum address.
    let addr: Address = Address::from_str(&address)?;

    // TODO: Make the block tag configurable.
    let block = BlockTag::Latest;

    // Query the balance of the Ethereum address.
    let nonce = beerus.ethereum_lightclient.get_nonce(&addr, block).await?;

    Ok(CommandResponse::EthereumQueryNonce(nonce))
}

/// Query the block number of the latest block.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// # Returns
/// * `Result<CommandResponse>` - The block number of the latest block.
/// # Errors
/// * If the block number query fails.
pub async fn query_block_number(beerus: BeerusLightClient) -> Result<CommandResponse> {
    let block_number = beerus.ethereum_lightclient.get_block_number().await?;
    Ok(CommandResponse::EthereumQueryBlockNumber(block_number))
}

/// Query the chain id of the Ethereum network.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// # Returns
/// * `Result<CommandResponse>` - The chain id of the Ethereum network.
pub async fn query_chain_id(beerus: BeerusLightClient) -> Result<CommandResponse> {
    let chain_id = beerus.ethereum_lightclient.chain_id().await;
    Ok(CommandResponse::EthereumQueryChainId(chain_id))
}
