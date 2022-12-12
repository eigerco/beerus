use rocket::serde::Serialize;
use schemars::JsonSchema;

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
pub struct QueryCodeResponse {
    pub code: Vec<u8>,
}
