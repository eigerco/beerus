use beerus_core::starknet_helper::block_id_string_to_block_id_type;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use starknet::providers::jsonrpc::models::BroadcastedTransaction;
use wiremock::{
    matchers::{body_json, method},
    Mock, MockServer, ResponseTemplate,
};

#[derive(Serialize, Debug)]
pub struct StarknetRpcBaseData<'a, StarknetParams> {
    id: usize,
    jsonrpc: &'a str,
    method: &'a str,
    params: StarknetParams,
}

#[derive(Deserialize, Debug)]
pub struct EthJsonRpcResponse<StarknetParams> {
    pub id: usize,
    pub jsonrpc: String,
    pub result: StarknetParams,
}

impl<'a, StarknetParams> StarknetRpcBaseData<'a, StarknetParams> {
    pub const fn stark_block_number(params: StarknetParams) -> Self {
        Self {
            id: 1,
            jsonrpc: "2.0",
            method: "starknet_blockNumber",
            params,
        }
    }

    pub const fn starknet_estimate_fee(params: StarknetParams) -> Self {
        Self {
            id: 1,
            jsonrpc: "2.0",
            method: "starknet_estimateFee",
            params,
        }
    }
}

pub async fn setup_wiremock() -> String {
    let mock_server = MockServer::start().await;
    mock_block_number().mount(&mock_server).await;
    mock_estimate_fee().mount(&mock_server).await;

    mock_server.uri()
}

fn mock_block_number() -> Mock {
    Mock::given(method(Method::POST))
        .and(body_json(StarknetRpcBaseData::stark_block_number(())))
        .respond_with(response_template_with_status(StatusCode::OK).set_body_raw(
            include_str!("data/blocks/starknet_blockNumber.json"),
            "application/json",
        ))
}

fn mock_estimate_fee() -> Mock {
    let block_type = "hash".to_string();
    let block_hash =
        "0x0147c4b0f702079384e26d9d34a15e7758881e32b219fc68c076b09d0be13f8c".to_string();
    let broadcasted_transaction = "{ \"type\": \"INVOKE\", \"nonce\": \"0x0\", \"max_fee\": \"0x12C72866EFA9B\", \"version\": \"0x0\", \"signature\": [ \"0x10E400D046147777C2AC5645024E1EE81C86D90B52D76AB8A8125E5F49612F9\", \"0x0ADB92739205B4626FEFB533B38D0071EB018E6FF096C98C17A6826B536817B\" ], \"contract_address\": \"0x0019fcae2482de8fb3afaf8d4b219449bec93a5928f02f58eef645cc071767f4\", \"calldata\": [ \"0x0000000000000000000000000000000000000000000000000000000000000001\", \"0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7\", \"0x0083afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e\", \"0x0000000000000000000000000000000000000000000000000000000000000000\", \"0x0000000000000000000000000000000000000000000000000000000000000003\", \"0x0000000000000000000000000000000000000000000000000000000000000003\", \"0x04681402a7ab16c41f7e5d091f32fe9b78de096e0bd5962ce5bd7aaa4a441f64\", \"0x000000000000000000000000000000000000000000000000001d41f6331e6800\", \"0x0000000000000000000000000000000000000000000000000000000000000000\", \"0x0000000000000000000000000000000000000000000000000000000000000001\" ], \"entry_point_selector\": \"0x015d40a3d6ca2ac30f4031e42be28da9b056fef9bb7357ac5e85627ee876e5ad\" }".to_string();

    let block_id = block_id_string_to_block_id_type(&block_type, &block_hash).unwrap();
    let broadcasted_transaction: BroadcastedTransaction =
        serde_json::from_str(&broadcasted_transaction).unwrap();

    Mock::given(method(Method::POST))
        .and(body_json(StarknetRpcBaseData::starknet_estimate_fee((
            broadcasted_transaction,
            &block_id,
        ))))
        .respond_with(response_template_with_status(StatusCode::OK).set_body_raw(
            include_str!("data/blocks/starknet_get_estimate_fee.json"),
            "application/json",
        ))
}

fn response_template_with_status(status_code: StatusCode) -> ResponseTemplate {
    ResponseTemplate::new(status_code)
        .append_header("vary", "Accept-Encoding")
        .append_header("vary", "Origin")
}
