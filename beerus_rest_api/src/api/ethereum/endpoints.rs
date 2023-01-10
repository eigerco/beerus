use crate::api::ethereum::resp::{
    QueryBalanceResponse, QueryBlockNumberResponse, QueryBlockTxCountByBlockNumberResponse,
    QueryChainIdResponse, QueryCodeResponse, QueryEstimateGasResponse, QueryGasPriceResponse,
    QueryNonceResponse, QueryTransactionByHashResponse, ResponseLog, TransactionObject,
};
use crate::api::ApiResponse;

use beerus_core::lightclient::beerus::BeerusLightClient;

use ethers::{
    types::{Address, H256, U256},
    utils,
};
use eyre::Result;
use helios::types::{BlockTag, CallOpts};
use log::debug;
use rocket::serde::json::Json;
use rocket::{get, State};
use rocket_okapi::openapi;
use std::str::FromStr;

use super::resp::{QueryLogsObject, QueryLogsResponse};

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
#[post("/ethereum/logs", data = "<query_logs_object>")]
pub async fn query_logs(
    beerus: &State<BeerusLightClient>,
    query_logs_object: Json<QueryLogsObject>,
) -> ApiResponse<QueryLogsResponse> {
    ApiResponse::from_result(query_logs_inner(beerus, query_logs_object).await)
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

/// Query logs.
/// # Returns
/// `Ok(logs_query)` - Vec<ResponseLog> (fetched lgos)
/// `Err(error)` - An error occurred.
/// # Errors
/// If the query fails, or if there are more than 5 logs.
/// # Examples
pub async fn query_logs_inner(
    beerus: &State<BeerusLightClient>,
    filter: Json<QueryLogsObject>,
) -> Result<QueryLogsResponse> {
    debug!("Querying logs");
    let Json(QueryLogsObject {
        address,
        block_hash,
        from_block,
        to_block,
        topics,
    }) = filter;
    let logs = beerus
        .ethereum_lightclient
        .get_logs(&from_block, &to_block, &address, &topics, &block_hash)
        .await?
        .into_iter()
        .map(|log| ResponseLog {
            address: format!("{:?}", log.address),
            topics: log
                .topics
                .into_iter()
                .map(|topic| format!("{:?}", topic))
                .collect::<Vec<String>>(),
            data: log.data.to_string(),
            block_hash: log.block_hash.map(|hash| format!("{:?}", hash)),
            block_number: log.block_number.map(|num| num.as_u64()),
            transaction_hash: log.transaction_hash.map(|hash| format!("{:?}", hash)),
            transaction_index: log.transaction_index.map(|num| num.as_u64()),
            log_index: log.log_index.map(|index| index.to_string()),
            transaction_log_index: log
                .transaction_log_index
                .map(|index| format!("{:?}", index)),
            log_type: log.log_type,
            removed: log.removed,
        })
        .collect::<Vec<_>>();
    Ok(QueryLogsResponse { logs })
}
