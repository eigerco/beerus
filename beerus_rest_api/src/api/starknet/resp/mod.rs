use rocket::serde::Serialize;
use schemars::JsonSchema;

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryStateRootResponse {
    pub state_root: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryContractViewResponse {
    pub result: Vec<String>,
}
#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryGetStorageAtResponse {
    pub result: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryNonceResponse {
    pub result: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryL1ToL2MessageCancellationsResponse {
    pub result: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryL1ToL2MessagesResponse {
    pub result: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryChainIdResponse {
    pub chain_id: String,
}
