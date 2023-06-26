use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::MockEthereumLightClient,
        starknet::StarkNetLightClientImpl,
    },
    starknet_helper::{block_id_string_to_block_id_type, create_mock_broadcasted_transaction},
};
use beerus_rpc::BeerusRpc;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use starknet::core::types::FieldElement;
use starknet::providers::jsonrpc::models::{BlockId, BlockTag, EventFilter, FunctionCall};
use std::path::PathBuf;
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
    pub const fn block_number(params: StarknetParams) -> Self {
        Self {
            id: 1,
            jsonrpc: "2.0",
            method: "starknet_blockNumber",
            params,
        }
    }

    pub const fn starknet_get_block_transaction_count(params: StarknetParams) -> Self {
        Self {
            id: 1,
            jsonrpc: "2.0",
            method: "starknet_getBlockTransactionCount",
            params,
        }
    }

    pub const fn get_events(params: StarknetParams) -> Self {
        Self {
            id: 1,
            jsonrpc: "2.0",
            method: "starknet_getEvents",
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

    pub const fn starknet_syncing(params: StarknetParams) -> Self {
        Self {
            id: 1,
            jsonrpc: "2.0",
            method: "starknet_syncing",
            params,
        }
    }

    pub const fn starknet_block_hash_and_number(params: StarknetParams) -> Self {
        Self {
            id: 1,
            jsonrpc: "2.0",
            method: "starknet_blockHashAndNumber",
            params,
        }
    }

    pub const fn starknet_get_transaction_by_block_id_and_index(params: StarknetParams) -> Self {
        Self {
            id: 1,
            jsonrpc: "2.0",
            method: "starknet_getTransactionByBlockIdAndIndex",
            params,
        }
    }

    pub const fn starknet_get_block_with_tx_hashes(params: StarknetParams) -> Self {
        Self {
            id: 1,
            jsonrpc: "2.0",
            method: "starknet_getBlockWithTxHashes",
            params,
        }
    }

    pub const fn starknet_call(params: StarknetParams) -> Self {
        Self {
            id: 1,
            jsonrpc: "2.0",
            method: "starknet_call",
            params,
        }
    }
}

pub async fn setup_wiremock() -> String {
    let mock_server = MockServer::start().await;
    mock_block_number().mount(&mock_server).await;
    mock_get_block_transaction_count().mount(&mock_server).await;
    mock_get_events().mount(&mock_server).await;
    mock_estimate_fee().mount(&mock_server).await;
    mock_starknet_syncing().mount(&mock_server).await;
    mock_starknet_block_hash_and_number()
        .mount(&mock_server)
        .await;
    mock_starknet_get_transaction_by_block_id_and_index()
        .mount(&mock_server)
        .await;
    mock_starknet_get_block_with_tx_hashes()
        .mount(&mock_server)
        .await;
    mock_starknet_call().mount(&mock_server).await;

    mock_server.uri()
}

pub async fn setup_beerus_rpc() -> BeerusRpc {
    let mut config = Config::from_file(&PathBuf::from("tests/common/data/test.toml"));
    config.starknet_rpc = setup_wiremock().await;

    let ethereum_lightclient = MockEthereumLightClient::new();
    let starknet_lightclient = StarkNetLightClientImpl::new(&config).unwrap();

    let beerus_client = BeerusLightClient::new(
        config,
        Box::new(ethereum_lightclient),
        Box::new(starknet_lightclient),
    );
    BeerusRpc::new(beerus_client)
}

fn mock_block_number() -> Mock {
    Mock::given(method("POST"))
        .and(body_json(StarknetRpcBaseData::block_number(())))
        .respond_with(response_template_with_status(StatusCode::OK).set_body_raw(
            include_str!("data/starknet_blockNumber.json"),
            "application/json",
        ))
}

fn mock_get_block_transaction_count() -> Mock {
    let latest_block = BlockId::Tag(BlockTag::Latest);
    Mock::given(method("POST"))
        .and(body_json(
            StarknetRpcBaseData::starknet_get_block_transaction_count([&latest_block]),
        ))
        .respond_with(response_template_with_status(StatusCode::OK).set_body_raw(
            include_str!("data/starknet_getBlockTransactionCount.json"),
            "application/json",
        ))
}

fn mock_estimate_fee() -> Mock {
    let block_type = "hash".to_string();
    let block_hash =
        "0x0147c4b0f702079384e26d9d34a15e7758881e32b219fc68c076b09d0be13f8c".to_string();
    let broadcasted_transaction = create_mock_broadcasted_transaction();
    let block_id = block_id_string_to_block_id_type(&block_type, &block_hash).unwrap();

    Mock::given(method("POST"))
        .and(body_json(StarknetRpcBaseData::starknet_estimate_fee((
            broadcasted_transaction.0,
            &block_id,
        ))))
        .respond_with(response_template_with_status(StatusCode::OK).set_body_raw(
            include_str!("data/starknet_getEstimateFee.json"),
            "application/json",
        ))
}

fn mock_get_events() -> Mock {
    // TODO: avoid duplicating the input values in rpc.rs
    let filter = EventFilter {
        from_block: Some(BlockId::Number(800)),
        to_block: Some(BlockId::Number(1701)),
        address: None,
        keys: None,
    };
    let continuation_token = Some("1000".to_string());
    let chunk_size = 1000;

    let param = json!({
        "from_block": filter.from_block,
        "to_block": filter.to_block,
        "continuation_token": continuation_token,
        "chunk_size": chunk_size
    });

    Mock::given(method("POST"))
        .and(body_json(StarknetRpcBaseData::get_events([&param])))
        .respond_with(response_template_with_status(StatusCode::OK).set_body_raw(
            include_str!("data/starknet_getEvents.json"),
            "application/json",
        ))
}

fn mock_starknet_syncing() -> Mock {
    Mock::given(method("POST"))
        .and(body_json(StarknetRpcBaseData::starknet_syncing(())))
        .respond_with(response_template_with_status(StatusCode::OK).set_body_raw(
            include_str!("data/starknet_syncing.json"),
            "application/json",
        ))
}

fn mock_starknet_block_hash_and_number() -> Mock {
    Mock::given(method("POST"))
        .and(body_json(
            StarknetRpcBaseData::starknet_block_hash_and_number(()),
        ))
        .respond_with(response_template_with_status(StatusCode::OK).set_body_raw(
            include_str!("data/starknet_blockHashAndNumber.json"),
            "application/json",
        ))
}

fn mock_starknet_get_transaction_by_block_id_and_index() -> Mock {
    let latest_block = BlockId::Tag(BlockTag::Latest);
    let index: u64 = 5;
    Mock::given(method("POST"))
        .and(body_json(
            StarknetRpcBaseData::starknet_get_transaction_by_block_id_and_index([
                serde_json::to_value(&latest_block).unwrap(),
                serde_json::to_value(index).unwrap(),
            ]),
        ))
        .respond_with(response_template_with_status(StatusCode::OK).set_body_raw(
            include_str!("data/starknet_getTransactionByBlockIdAndIndex.json"),
            "application/json",
        ))
}

fn mock_starknet_get_block_with_tx_hashes() -> Mock {
    let latest_block = BlockId::Tag(BlockTag::Latest);
    Mock::given(method("POST"))
        .and(body_json(
            StarknetRpcBaseData::starknet_get_block_with_tx_hashes([&latest_block]),
        ))
        .respond_with(response_template_with_status(StatusCode::OK).set_body_raw(
            include_str!("data/starknet_getBlockWithTxHashes.json"),
            "application/json",
        ))
}

fn mock_starknet_call() -> Mock {
    let request = FunctionCall {
        contract_address: FieldElement::from_hex_be(
            "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )
        .unwrap(),
        entry_point_selector: FieldElement::from_hex_be(
            "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60",
        )
        .unwrap(),
        calldata: Vec::new(),
    };
    let latest_block = BlockId::Tag(BlockTag::Latest);

    Mock::given(method(Method::POST))
        .and(body_json(StarknetRpcBaseData::starknet_call((
            &request,
            &latest_block,
        ))))
        .respond_with(
            response_template_with_status(StatusCode::OK)
                .set_body_raw(include_str!("data/starknet_call.json"), "application/json"),
        )
}

fn response_template_with_status(status_code: StatusCode) -> ResponseTemplate {
    ResponseTemplate::new(status_code)
        .append_header("vary", "Accept-Encoding")
        .append_header("vary", "Origin")
}
