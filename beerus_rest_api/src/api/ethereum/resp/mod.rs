use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionObject {
    pub from: Option<String>,
    pub to: String,
    pub gas: Option<String>,
    pub gas_price: Option<String>,
    pub value: Option<String>,
    pub data: Option<String>,
    pub nonce: Option<String>,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct SendRawTransactionResponse {
    pub response: String,
}
#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryBalanceResponse {
    pub address: String,
    pub balance: String,
    pub unit: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryNonceResponse {
    pub address: String,
    pub nonce: u64,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryBlockNumberResponse {
    pub block_number: u64,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryChainIdResponse {
    pub chain_id: u64,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryCodeResponse {
    pub code: Vec<u8>,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryTxCountResponse {
    pub tx_count: u64,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryBlockTxCountByBlockNumberResponse {
    pub tx_count: u64,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryBlockTxCountByBlockHashResponse {
    pub tx_count: u64,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryTransactionByHashResponse {
    pub tx_data: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryGasPriceResponse {
    pub gas_price: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryEstimateGasResponse {
    pub quantity: u64,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryBlockByHashResponse {
    pub block: Option<serde_json::Value>,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryPriorityFeeResponse {
    pub priority_fee: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryBlockByNumberResponse {
    pub block: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryLogsObject {
    pub address: Option<String>,
    pub block_hash: Option<String>,
    pub from_block: Option<String>,
    pub to_block: Option<String>,
    pub topics: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseLog {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub block_hash: Option<String>,
    pub block_number: Option<u64>,
    pub transaction_hash: Option<String>,
    pub transaction_index: Option<u64>,
    pub log_index: Option<String>,
    pub transaction_log_index: Option<String>,
    pub log_type: Option<String>,
    pub removed: Option<bool>,
}
#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryLogsResponse {
    pub logs: Vec<ResponseLog>,
}
