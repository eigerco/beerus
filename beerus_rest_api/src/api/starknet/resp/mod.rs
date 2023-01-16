use rocket::serde::Serialize;
use schemars::JsonSchema;
use serde_json::Value;
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
pub struct QueryL2ToL1MessagesResponse {
    pub result: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryL1ToL2MessageNonceResponse {
    pub result: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryChainIdResponse {
    pub chain_id: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryBlockNumberResponse {
    pub block_number: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryBlockHashAndNumberResponse {
    pub block_hash: String,
    pub block_number: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryGetClassResponse {
    pub abi: Value,
    pub entry_points_by_type: Value,
    pub program: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryGetClassAtResponse {
    pub abi: Value,
    pub entry_points_by_type: Value,
    pub program: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryGetBlockTransactionCountResponse {
    pub block_transaction_count: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QuerySyncing {
    pub data: Option<Value>,
    pub status: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct NonceResponse {
    pub contract_address: String,
    pub nonce: String,
}
#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct DeployedContractResponse {
    pub address: String,
    pub class_hash: String,
}
#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct StorageEntryResponse {
    pub key: String,
    pub value: String,
}
#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct StorageDiffResponse {
    pub address: String,
    pub storage_entries: Vec<StorageEntryResponse>,
}

#[derive(Serialize, JsonSchema)]
pub struct StateDiffResponse {
    pub storage_diffs: Vec<StorageDiffResponse>,
    pub declared_contract_hash: Vec<String>,
    pub deployed_contracts: Vec<DeployedContractResponse>,
    pub nonces: Vec<NonceResponse>,
}
#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryStateUpdateResponse {
    pub block_hash: String,
    pub new_root: String,
    pub old_root: String,
    pub state_diff: StateDiffResponse,
}
