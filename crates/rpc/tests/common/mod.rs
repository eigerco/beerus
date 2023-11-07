use std::net::SocketAddr;
use std::str::FromStr;

use beerus_core::client::BeerusClient;
use beerus_core::config::Config;
use beerus_rpc::BeerusRpc;
use reqwest::{Method, StatusCode};
use rstest::fixture;
use serde::{Deserialize, Serialize};
use serde_json::json;
use starknet::core::types::{BlockId, BlockTag, FunctionCall};
use starknet::macros::felt;
use wiremock::matchers::{body_json, method};
use wiremock::{Mock, MockServer, ResponseTemplate};
pub mod helper;
use helper::create_mock_broadcasted_transaction;
use starknet::core::types::FieldElement;

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
    pub const fn chain_id(params: StarknetParams) -> Self {
        Self { id: 1, jsonrpc: "2.0", method: "starknet_chainId", params }
    }

    pub const fn block_number(params: StarknetParams) -> Self {
        Self { id: 1, jsonrpc: "2.0", method: "starknet_blockNumber", params }
    }

    pub const fn starknet_get_block_transaction_count(params: StarknetParams) -> Self {
        Self { id: 1, jsonrpc: "2.0", method: "starknet_getBlockTransactionCount", params }
    }

    pub const fn starknet_syncing(params: StarknetParams) -> Self {
        Self { id: 1, jsonrpc: "2.0", method: "starknet_syncing", params }
    }

    pub const fn starknet_block_hash_and_number(params: StarknetParams) -> Self {
        Self { id: 1, jsonrpc: "2.0", method: "starknet_blockHashAndNumber", params }
    }
    pub const fn starknet_estimate_fee(params: StarknetParams) -> Self {
        Self { id: 1, jsonrpc: "2.0", method: "starknet_estimateFee", params }
    }

    pub const fn starknet_get_transaction_by_block_id_and_index(params: StarknetParams) -> Self {
        Self { id: 1, jsonrpc: "2.0", method: "starknet_getTransactionByBlockIdAndIndex", params }
    }

    pub const fn starknet_get_block_with_tx_hashes(params: StarknetParams) -> Self {
        Self { id: 1, jsonrpc: "2.0", method: "starknet_getBlockWithTxHashes", params }
    }

    pub const fn starknet_call(params: StarknetParams) -> Self {
        Self { id: 1, jsonrpc: "2.0", method: "starknet_call", params }
    }
}
pub const MOCK_BLOCK_NUMBER: u64 = 19640; // current block number

#[fixture]
pub async fn setup_beerus_rpc() -> BeerusRpc {
    let mut config = Config::from_file("../../examples/conf/beerus.toml");
    config.starknet_rpc = setup_wiremock().await;
    // config.eth_execution_rpc = "http://localhost:4000".to_string();
    let mut beerus = BeerusClient::new(config.clone()).await;
    {
        let mut node_lock = beerus.node.write().await;
        // node_lock.l1_state_root =
        // felt!("0x074af3e986fa1fe87edb5ef2ae58bcc8d86b2fe7f63c7fae8583a47993a0f32a");
        node_lock.l1_state_root = felt!("0x1");
        node_lock.l1_block_num = MOCK_BLOCK_NUMBER;
    }
    beerus.start().await.unwrap();

    BeerusRpc::new(beerus)
}

#[fixture]
pub async fn setup_wiremock() -> String {
    let mock_server = MockServer::start().await;
    mock_chain_id().mount(&mock_server).await;
    mock_block_number().mount(&mock_server).await;
    mock_get_block_transaction_count(BlockId::Tag(BlockTag::Latest), 100).mount(&mock_server).await;
    mock_get_block_transaction_count(BlockId::Number(MOCK_BLOCK_NUMBER), 90).mount(&mock_server).await;
    mock_starknet_syncing().mount(&mock_server).await;
    mock_estimate_fee().mount(&mock_server).await;
    mock_starknet_block_hash_and_number().mount(&mock_server).await;
    mock_starknet_get_transaction_by_block_id_and_index().mount(&mock_server).await;
    mock_starknet_get_block_with_tx_hashes().mount(&mock_server).await;
    mock_starknet_call().mount(&mock_server).await;

    mock_server.uri()
}

fn rpc_json_body(result: serde_json::Value) -> String {
    json!({"jsonrpc": "2.0", "id": 1, "result": result }).to_string()
}

fn mock_chain_id() -> Mock {
    Mock::given(method("POST")).and(body_json(StarknetRpcBaseData::chain_id(Vec::<u8>::new()))).respond_with(
        response_template_with_status(StatusCode::OK)
            .set_body_raw(rpc_json_body(json!("0x534e5f4d41494e")), "application/json"),
    )
}

fn mock_block_number() -> Mock {
    Mock::given(method("POST")).and(body_json(StarknetRpcBaseData::block_number(Vec::<u8>::new()))).respond_with(
        response_template_with_status(StatusCode::OK)
            .set_body_raw(rpc_json_body(json!(MOCK_BLOCK_NUMBER)), "application/json"),
    )
}

fn mock_get_block_transaction_count(id: BlockId, response: u64) -> Mock {
    Mock::given(method("POST"))
        .and(body_json(StarknetRpcBaseData::starknet_get_block_transaction_count([&id])))
        .respond_with(
            response_template_with_status(StatusCode::OK)
                .set_body_raw(rpc_json_body(json!(response)), "application/json"),
        )
}

fn mock_estimate_fee() -> Mock {
    let broadcasted_transaction = create_mock_broadcasted_transaction();
    // let block_id = BlockId::Hash(
    //     FieldElement::from_str(
    //         "0x0147c4b0f702079384e26d9d34a15e7758881e32b219fc68c076b09d0be13f8c",
    //     )
    //         .unwrap(),
    // );
    let block_id = BlockId::Number(MOCK_BLOCK_NUMBER);
    let result = json!([{
      "gas_consumed": "0x1de6",
      "gas_price": "0x5df32828e",
      "overall_fee": "0xaf8f402b6194"
    }]);
    Mock::given(method("POST"))
        .and(body_json(StarknetRpcBaseData::starknet_estimate_fee((vec![broadcasted_transaction.0], &block_id))))
        .respond_with(
            response_template_with_status(StatusCode::OK).set_body_raw(rpc_json_body(result), "application/json"),
        )
}

fn mock_starknet_syncing() -> Mock {
    let result = json!({
      "current_block_hash": "0x7f65231188b64236c1142ae6a894e826583725bef6b9172f46b6ad5f9d87469",
      "current_block_num": 27468,
      "highest_block_hash": "0x7f65231188b64236c1142ae6a894e826583725bef6b9172f46b6ad5f9d87469",
      "highest_block_num": 27468,
      "starting_block_hash": "0x54cfb11a0c61c26b2e84c6d085a8317e5a1a437fa092d59a97564936afe2438",
      "starting_block_num": 24317
    });

    Mock::given(method("POST")).and(body_json(StarknetRpcBaseData::starknet_syncing(Vec::<u8>::new()))).respond_with(
        response_template_with_status(StatusCode::OK).set_body_raw(rpc_json_body(result), "application/json"),
    )
}

fn mock_starknet_block_hash_and_number() -> Mock {
    let result = json!({
        "block_hash": "0x63813d0cd71bf351dfe3217f9d2dcd8871cf4d56c0ffe3563980b3d02b6898d",
        "block_number": 27461
    });

    Mock::given(method("POST"))
        .and(body_json(StarknetRpcBaseData::starknet_block_hash_and_number(Vec::<u8>::new())))
        .respond_with(
            response_template_with_status(StatusCode::OK).set_body_raw(rpc_json_body(result), "application/json"),
        )
}

fn mock_starknet_get_transaction_by_block_id_and_index() -> Mock {
    let result = json!({
      "calldata": [
        "0x1"
      ],
      "max_fee": "0x1",
      "nonce": "0x1",
      "sender_address": "0x1",
      "signature": [
        "0x1"
      ],
      "transaction_hash": "0x1",
      "type": "INVOKE",
      "version": "0x1"
    });
    Mock::given(method("POST"))
        .and(body_json(StarknetRpcBaseData::starknet_get_transaction_by_block_id_and_index([
            serde_json::to_value(&BlockId::Tag(BlockTag::Latest)).unwrap(),
            serde_json::to_value(5_u64).unwrap(),
        ])))
        .respond_with(
            response_template_with_status(StatusCode::OK).set_body_raw(rpc_json_body(result), "application/json"),
        )
}

fn mock_starknet_get_block_with_tx_hashes() -> Mock {
    let result = json!({
      "block_hash": "0x1",
      "block_number": 1,
      "new_root": "0x1",
      "parent_hash": "0x1",
      "sequencer_address": "0x1",
      "status": "ACCEPTED_ON_L2",
      "timestamp": 10,
      "transactions": [
        "0x1"
      ]
    });
    Mock::given(method("POST"))
        .and(body_json(StarknetRpcBaseData::starknet_get_block_with_tx_hashes([&BlockId::Tag(BlockTag::Latest)])))
        .respond_with(
            response_template_with_status(StatusCode::OK).set_body_raw(rpc_json_body(result), "application/json"),
        )
}

fn mock_starknet_call() -> Mock {
    let request = FunctionCall {
        contract_address: felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7"),
        entry_point_selector: felt!("0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60"),
        calldata: Vec::new(),
    };

    let result = json!(["298305742194"]);

    Mock::given(method(Method::POST))
        .and(body_json(StarknetRpcBaseData::starknet_call((&request, &BlockId::Tag(BlockTag::Latest)))))
        .respond_with(
            response_template_with_status(StatusCode::OK).set_body_raw(rpc_json_body(result), "application/json"),
        )
}

fn response_template_with_status(status_code: StatusCode) -> ResponseTemplate {
    ResponseTemplate::new(status_code).append_header("vary", "Accept-Encoding").append_header("vary", "Origin")
}
