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
pub struct QueryBlockTxCountByBlockNumberResponse {
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
