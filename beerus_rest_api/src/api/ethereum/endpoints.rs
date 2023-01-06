use crate::api::ethereum::resp::{
    QueryBalanceResponse, QueryBlockNumberResponse, QueryBlockTxCountByBlockNumberResponse,

    QueryChainIdResponse, QueryCodeResponse, QueryEstimateGasResponse, QueryGasPriceResponse,
    QueryNonceResponse, QueryTransactionByHashResponse, TransactionObject,SendRawTransactionResponse, QueryBlockByHashResponse

};
use crate::api::ApiResponse;

use beerus_core::lightclient::beerus::BeerusLightClient;
use ethers::types::U256;

use ethers::{
    types::{Address, H256},
    utils,
};
use eyre::Result;
use helios::types::{BlockTag, CallOpts};
use log::debug;
use rocket::serde::json::Json;
use rocket::{get, State};
use rocket_okapi::openapi;
use std::str::FromStr;

#[openapi]
#[get("/ethereum/send_raw_transaction/<bytes>")]
pub async fn send_raw_transaction(
    bytes: &str,
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<SendRawTransactionResponse> {
    ApiResponse::from_result(send_raw_transaction_inner(beerus, bytes).await)
}

#[openapi]
#[get("/ethereum/balance/<address>")]
pub async fn query_balance(
    address: &str,
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryBalanceResponse> {
    ApiResponse::from_result(query_balance_inner(beerus, address).await)
}

#[openapi]
#[get("/ethereum/nonce/<address>")]
pub async fn query_nonce(
    address: &str,
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryNonceResponse> {
    ApiResponse::from_result(query_nonce_inner(beerus, address).await)
}

#[openapi]
#[get("/ethereum/block_number")]
pub async fn query_block_number(
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryBlockNumberResponse> {
    ApiResponse::from_result(query_block_number_inner(beerus).await)
}

#[openapi]
#[get("/ethereum/chain_id")]
pub async fn query_chain_id(
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryChainIdResponse> {
    ApiResponse::from_result(query_chain_id_inner(beerus).await)
}

#[openapi]
#[get("/ethereum/code/<address>")]
pub async fn query_code(
    address: &str,
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryCodeResponse> {
    ApiResponse::from_result(query_code_inner(address, beerus).await)
}

#[openapi]
#[get("/ethereum/tx_count_by_block_number/<block>")]
pub async fn get_block_transaction_count_by_number(
    block: u64,
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryBlockTxCountByBlockNumberResponse> {
    ApiResponse::from_result(query_block_transaction_count_by_number_inner(block, beerus).await)
}

#[openapi]
#[get("/ethereum/tx_by_hash/<hash>")]
pub async fn get_transaction_by_hash(
    hash: &str,
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryTransactionByHashResponse> {
    ApiResponse::from_result(query_transaction_by_hash_inner(hash, beerus).await)
}

#[openapi]
#[get("/ethereum/gas_price")]
pub async fn get_gas_price(
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryGasPriceResponse> {
    ApiResponse::from_result(query_gas_price_inner(beerus).await)
}

#[openapi]
#[post("/ethereum/estimate_gas", data = "<transaction_object>")]
pub async fn query_estimate_gas(
    beerus: &State<BeerusLightClient>,
    transaction_object: Json<TransactionObject>,
) -> ApiResponse<QueryEstimateGasResponse> {
    ApiResponse::from_result(query_estimate_gas_inner(beerus, transaction_object).await)
}

#[openapi]
#[get("/ethereum/get_block_by_hash/<hash>/<full_tx>")]
pub async fn get_block_by_hash(
    hash: &str,
    full_tx: &str,
    beerus: &State<BeerusLightClient>,
) -> ApiResponse<QueryBlockByHashResponse> {
    ApiResponse::from_result(query_block_by_hash_inner(beerus, hash, full_tx).await)
}

/// Query the balance of an Ethereum address.
/// # Arguments
/// * `address` - The Ethereum address.
/// # Returns
/// `Ok(query_balance_response)` - The query balance response.
/// `Err(error)` - An error occurred.
/// # Errors
/// If the Ethereum address is invalid or the block tag is invalid.
/// # Examples
pub async fn query_balance_inner(
    beerus: &State<BeerusLightClient>,
    address: &str,
) -> Result<QueryBalanceResponse> {
    debug!("Querying balance of address: {}", address);
    // Parse the Ethereum address.
    let addr = Address::from_str(address)?;
    // TODO: Make the block tag configurable.
    let block = BlockTag::Latest;
    // Query the balance of the Ethereum address.
    let balance = beerus
        .ethereum_lightclient
        .get_balance(&addr, block)
        .await?;
    // Format the balance in Ether.
    let balance_in_eth = utils::format_units(balance, "ether")?;
    Ok(QueryBalanceResponse {
        address: address.to_string(),
        balance: balance_in_eth,
        unit: "ETH".to_string(),
    })
}

/// Query the balance of an Ethereum address.
/// # Arguments
/// * `address` - The Ethereum address.
/// # Returns
/// `Ok(query_balance_response)` - The query balance response.
/// `Err(error)` - An error occurred.
/// # Errors
/// If the Ethereum address is invalid or the block tag is invalid.
/// # Examples

pub async fn query_nonce_inner(
    beerus: &State<BeerusLightClient>,
    address: &str,
) -> Result<QueryNonceResponse> {
    debug!("Querying nonce of address: {}", address);
    let addr = Address::from_str(address)?;
    let block = BlockTag::Latest;
    let nonce = beerus.ethereum_lightclient.get_nonce(&addr, block).await?;

    Ok(QueryNonceResponse {
        address: address.to_string(),
        nonce,
    })
}

/// Query the block number of the Ethereum chain.
/// # Returns
/// `Ok(block_number)` - The block number.
/// `Err(error)` - An error occurred.
/// # Errors
/// If the block number query fails.
/// # Examples
pub async fn query_block_number_inner(
    beerus: &State<BeerusLightClient>,
) -> Result<QueryBlockNumberResponse> {
    debug!("Querying block number");
    let block_number = beerus.ethereum_lightclient.get_block_number().await?;
    Ok(QueryBlockNumberResponse { block_number })
}

/// Query the chain ID of the Ethereum chain.
/// # Returns
/// `chain_id` - The chain ID.
/// # Errors
/// Cannot fail.
/// # Examples
pub async fn query_chain_id_inner(
    beerus: &State<BeerusLightClient>,
) -> Result<QueryChainIdResponse> {
    debug!("Querying chain ID");
    let chain_id = beerus.ethereum_lightclient.chain_id().await;
    Ok(QueryChainIdResponse { chain_id })
}

/// Query the Code of a contract from the the Ethereum chain.
/// # Returns
/// `Ok(get_code)` - 256bits vector (code)
/// `Err(error)` - An error occurred.
/// # Errors
/// If the code query fails.
/// # Examples
pub async fn query_code_inner(
    address: &str,
    beerus: &State<BeerusLightClient>,
) -> Result<QueryCodeResponse> {
    debug!("Querying contract code");
    let addr = Address::from_str(address)?;
    let block = BlockTag::Latest;
    let code = beerus.ethereum_lightclient.get_code(&addr, block).await?;

    Ok(QueryCodeResponse { code })
}

/// Query the Tx count of a given Block Number from the the Ethereum chain.
/// # Returns
/// `Ok(get_block_transaction_count_by_number)` - u64 (tx_count)
/// `Err(error)` - An error occurred.
/// # Errors
/// If the code query fails.
/// # Examples
pub async fn query_block_transaction_count_by_number_inner(
    _block: u64,
    beerus: &State<BeerusLightClient>,
) -> Result<QueryBlockTxCountByBlockNumberResponse> {
    debug!("Querying Block Tx count");
    // TODO: Change to BlockTag::Number(block) when previous blocks are available from Helios Client
    let block = BlockTag::Latest;
    let tx_count = beerus
        .ethereum_lightclient
        .get_block_transaction_count_by_number(block)
        .await?;

    Ok(QueryBlockTxCountByBlockNumberResponse { tx_count })
}

/// Query the Tx data of a Tx Hash from the the Ethereum chain.
/// # Returns
/// `Ok(get_transaction_by_hash)` - u64 (tx_count)
/// `Err(error)` - An error occurred.
/// # Errors
/// If the code query fails.
/// # Examples
pub async fn query_transaction_by_hash_inner(
    hash: &str,
    beerus: &State<BeerusLightClient>,
) -> Result<QueryTransactionByHashResponse> {
    debug!("Querying Tx data");
    let h256_hash = H256::from_str(hash).unwrap();

    let unformatted_tx_data = beerus
        .ethereum_lightclient
        .get_transaction_by_hash(&h256_hash)
        .await?;
    let tx_data = format!("{unformatted_tx_data:?}");

    Ok(QueryTransactionByHashResponse { tx_data })
}

/// Query gas price from the the Ethereum chain.
/// # Returns
/// `Ok(get_gas_price)` - U256 (gas_price)
/// `Err(error)` - An error occurred.
/// # Errors
/// If the code query fails.
/// # Examples
pub async fn query_gas_price_inner(
    beerus: &State<BeerusLightClient>,
) -> Result<QueryGasPriceResponse> {
    debug!("Querying Gas Price");
    let unformatted_tx_data = beerus.ethereum_lightclient.get_gas_price().await?;
    let gas_price = format!("{unformatted_tx_data:?}");
    Ok(QueryGasPriceResponse { gas_price })
}

/// Query the gas estimate of a transaction from the the Ethereum chain.
/// # Returns
/// `Ok(get_gas_estimate)` - u64 (quantity)
/// `Err(error)` - An error occurred.
/// # Errors
/// If the code query fails.
/// # Examples
pub async fn query_estimate_gas_inner(
    beerus: &State<BeerusLightClient>,
    transaction_object: Json<TransactionObject>,
) -> Result<QueryEstimateGasResponse> {
    debug!("Querying Gas Estimate");
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
            .and_then(|v| (ethers::utils::hex::decode(v)).ok()),
    };

    let quantity = beerus.ethereum_lightclient.estimate_gas(&call_opts).await?;
    Ok(QueryEstimateGasResponse { quantity })
}

/// Send raw transaction.
/// # Arguments
/// * `bytes` - Bytes of the transaction.
/// # Returns
/// `Ok(send_raw_transaction)` - Response from the Raw Transaction.
/// `Err(error)` - An error occurred.
/// # Errors
/// If the Ethereum address is invalid or the block tag is invalid.
/// # Examples
pub async fn send_raw_transaction_inner(
    beerus: &State<BeerusLightClient>,
    bytes: &str,
) -> Result<SendRawTransactionResponse> {
    debug!("Sending Raw Transaction: {}", bytes);
    let bytes: Vec<u8> = bytes[2..]
        .chars()
        .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
        .collect();
    let bytes_slice: &[u8] = bytes.as_ref();
    // Send Raw Transaction.
    let response = beerus
        .ethereum_lightclient
        .send_raw_transaction(bytes_slice)
        .await?;

    Ok(SendRawTransactionResponse {
        response: format!("{response:?}"),
    })
}

/// Query information about a block by block number.
/// # Arguments
/// * `block` - The block number or tag.
/// * `full_tx` - Whether to return full transaction objects or just the transaction hashes.
/// # Returns
/// `Ok(query_block_response)` - The query block response.
/// `Err(error)` - An error occurred.
/// # Errors
/// If the block tag is invalid or the full_tx boolean is invalid.
/// # Examples
pub async fn query_block_by_hash_inner(
    beerus: &State<BeerusLightClient>,
    hash: &str,
    full_tx: &str,
) -> Result<QueryBlockByHashResponse> {
    debug!(
        "Querying block by hash: {}, with full transactions: {}",
        hash, full_tx
    );

    let hash: Vec<u8> = hash[2..]
        .chars()
        .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
        .collect();

    let full_tx = bool::from_str(full_tx)?;
    let block_details = beerus
        .ethereum_lightclient
        .get_block_by_hash(&hash, full_tx)
        .await?;
    let block = match block_details {
        Some(block) => {
            let block_json_string: String = serde_json::to_string(&block).unwrap();
            let block_json_value: serde_json::Value =
                serde_json::from_str(block_json_string.as_str()).unwrap();
            Some(block_json_value)
        }
        None => None,
    };
    Ok(QueryBlockByHashResponse { block })
}