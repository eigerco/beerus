use crate::model::CommandResponse;
use beerus_core::lightclient::beerus::BeerusLightClient;
use core::str::FromStr;
use ethers::types::U256;
use ethers::utils::hex;
use ethers::{
    types::{Address, H256},
    utils,
};
use eyre::Result;
use helios::types::{BlockTag, CallOpts};
use serde::{Deserialize, Serialize};

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

/// Send Raw Transaction on Ethereum
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `bytes` - Raw Transactions Bytes.
/// # Returns
/// * `Result<CommandResponse>` - Raw tx bytes response.
/// # Errors
/// * If the Ethereum address is invalid.
/// * If the balance query fails.
pub async fn send_raw_transaction(
    beerus: BeerusLightClient,
    bytes: String,
) -> Result<CommandResponse> {
    // Parse the Ethereum address.
    let bytes: Vec<u8> = bytes[2..]
        .chars()
        .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
        .collect();
    let bytes_slice: &[u8] = bytes.as_ref();

    // Query the balance of the Ethereum address.
    let transaction_response = beerus
        .ethereum_lightclient
        .send_raw_transaction(bytes_slice)
        .await?;

    Ok(CommandResponse::EthereumSendRawTransaction(
        transaction_response,
    ))
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

/// Query information about a block by block hash.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `hash` - The block number or tag.
/// * `full_tx` - Whether to return full transaction objects or just the transaction hashes.
/// # Returns
/// * `Result<CommandResponse>` - The block information.
/// # Errors
/// * If the block query fails.
pub async fn query_block_by_hash(
    beerus: BeerusLightClient,
    hash: String,
    full_tx: bool,
) -> Result<CommandResponse> {
    let hash: Vec<u8> = hash[2..]
        .chars()
        .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
        .collect();
    let block = beerus
        .ethereum_lightclient
        .get_block_by_hash(&hash, full_tx)
        .await?;
    Ok(CommandResponse::EthereumQueryBlockByHash(block))
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

/// Query tx count of a given block Hash
/// # Arguments
/// * `beerus` - The Beerus light client.
/// # Returns
/// * `Result<CommandResponse>` - u64 (txs counts)
/// # Errors
/// * If the block number query fails.
pub async fn query_block_transaction_count_by_hash(
    beerus: BeerusLightClient,
    hash: String,
) -> Result<CommandResponse> {
    let hash: Vec<u8> = hash[2..]
        .chars()
        .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
        .collect();

    let tx_count = beerus
        .ethereum_lightclient
        .get_block_transaction_count_by_hash(&hash)
        .await?;

    Ok(CommandResponse::EthereumQueryBlockTxCountByHash(tx_count))
}

/// Query max priority fee per gas from Ethereum
/// # Arguments
/// * `beerus` - The Beerus light client.
/// # Returns
/// * `Result<CommandResponse>` - Gas Price from the Ethereum Network :
/// # Errors
/// * If the block number query fails.
pub async fn query_get_priority_fee(beerus: BeerusLightClient) -> Result<CommandResponse> {
    let get_priority_fee = beerus.ethereum_lightclient.get_priority_fee().await?;

    Ok(CommandResponse::EthereumQueryGetPriorityFee(
        get_priority_fee,
    ))
}

/// Query information about a block by block number.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `block` - The block number or tag.
/// * `full_tx` - Whether to return full transaction objects or just the transaction hashes.
/// # Returns
/// * `Result<CommandResponse>` - The block information.
/// # Errors
/// * If the block query fails.
pub async fn query_block_by_number(
    beerus: BeerusLightClient,
    block: BlockTag,
    full_tx: bool,
) -> Result<CommandResponse> {
    let block = beerus
        .ethereum_lightclient
        .get_block_by_number(block, full_tx)
        .await?;
    Ok(CommandResponse::EthereumQueryBlockByNumber(block))
}
pub async fn query_logs(
    beerus: BeerusLightClient,
    from_block: &Option<String>,
    to_block: &Option<String>,
    address: &Option<String>,
    topics: &Option<Vec<String>>,
    block_hash: &Option<String>,
) -> Result<CommandResponse> {
    let logs = beerus
        .ethereum_lightclient
        .get_logs(from_block, to_block, address, topics, block_hash)
        .await?;
    Ok(CommandResponse::EthereumQueryLogs(logs))
}
