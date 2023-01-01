use crate::model::CommandResponse;
use beerus_core::lightclient::beerus::BeerusLightClient;
use ethers::types::U256;
use ethers::utils::hex;
use ethers::{
    types::{Address, H256},
    utils,
};
use eyre::Result;
use helios::types::{BlockTag, CallOpts};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionObject {
    pub from: Option<String>,
    pub to: String,
    pub gas: Option<String>,
    pub gas_price: Option<String>,
    pub value: Option<String>,
    pub data: Option<String>,
    pub nonce: Option<String>,
}

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

/// Query the code of a contract
/// # Arguments
/// * `beerus` - The Beerus light client.
/// # Returns
/// * `Result<CommandResponse>` - Vector of 256bits
/// # Errors
/// * If the block number query fails.
pub async fn query_code(beerus: BeerusLightClient, address: String) -> Result<CommandResponse> {
    //TODO: Make this configurable
    let block = BlockTag::Latest;

    let addr = Address::from_str(&address)?;

    let code = beerus.ethereum_lightclient.get_code(&addr, block).await?;

    Ok(CommandResponse::EthereumQueryCode(code))
}

/// Query tx count of a given block Number
/// # Arguments
/// * `beerus` - The Beerus light client.
/// # Returns
/// * `Result<CommandResponse>` - u64 (txs counts)
/// # Errors
/// * If the block number query fails.
pub async fn query_block_transaction_count_by_number(
    beerus: BeerusLightClient,
    block: u64,
) -> Result<CommandResponse> {
    let block = BlockTag::Number(block);

    let tx_count = beerus
        .ethereum_lightclient
        .get_block_transaction_count_by_number(block)
        .await?;

    Ok(CommandResponse::EthereumQueryBlockTxCountByNumber(tx_count))
}

/// Query Tx data of a given Tx Hash
/// # Arguments
/// * `beerus` - The Beerus light client.
/// # Returns
/// * `Result<CommandResponse>` - Tx Data :
/// # Errors
/// * If the block number query fails.
pub async fn query_transaction_by_hash(
    beerus: BeerusLightClient,
    tx_hash: String,
) -> Result<CommandResponse> {
    let hash = H256::from_str(&tx_hash)?;

    let unformatted_tx_data = beerus
        .ethereum_lightclient
        .get_transaction_by_hash(&hash)
        .await?;
    let tx_data = format!("{unformatted_tx_data:?}");

    Ok(CommandResponse::EthereumQueryTxByHash(tx_data))
}

/// Query gas price from Ethereum
/// # Arguments
/// * `beerus` - The Beerus light client.
/// # Returns
/// * `Result<CommandResponse>` - Gas Price from the Ethereum Network :
/// # Errors
/// * If the block number query fails.
pub async fn query_gas_price(beerus: BeerusLightClient) -> Result<CommandResponse> {
    let gas_price = beerus.ethereum_lightclient.get_gas_price().await?;

    Ok(CommandResponse::EthereumQueryGasPrice(gas_price))
}

/// Query how much gas is necessary to allow the transaction to complete from Ethereum
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `transactionObject` - The transaction object
/// # Returns
/// * `Result<CommandResponse>` - Quantity of gas required from the Ethereum Network
/// # Errors
/// * If the query fails.
pub async fn query_estimate_gas(
    beerus: BeerusLightClient,
    params: String,
) -> Result<CommandResponse> {
    let transaction_object: TransactionObject = serde_json::from_str(&params)?;
    let call_opts = CallOpts {
        from: transaction_object
            .from
            .as_ref()
            .and_then(|v| Address::from_str(v).ok()),
        to: Address::from_str(&transaction_object.to)?,
        value: transaction_object
            .value
            .as_ref()
            .and_then(|v| U256::from_dec_str(v).ok()),
        gas: transaction_object
            .gas
            .as_ref()
            .and_then(|v| U256::from_dec_str(v).ok()),
        gas_price: transaction_object
            .gas_price
            .as_ref()
            .and_then(|v| U256::from_dec_str(v).ok()),
        data: transaction_object
            .data
            .as_ref()
            .and_then(|v| (hex::decode(v)).ok()),
    };

    let gas = beerus.ethereum_lightclient.estimate_gas(&call_opts).await?;

    Ok(CommandResponse::EthereumQueryEstimateGas(gas))
}
