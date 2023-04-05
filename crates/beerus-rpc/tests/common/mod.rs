use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::MockEthereumLightClient,
        starknet::StarkNetLightClientImpl,
    },
    starknet_helper::block_id_string_to_block_id_type,
};
use beerus_rpc::BeerusRpc;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use starknet::providers::jsonrpc::models::{
    BlockId, BlockTag, BroadcastedTransaction, EventFilter,
};
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
    let broadcasted_transaction = "{ \"type\": \"INVOKE\", \"nonce\": \"0x0\", \"max_fee\": \"0x12C72866EFA9B\", \"version\": \"0x0\", \"signature\": [ \"0x10E400D046147777C2AC5645024E1EE81C86D90B52D76AB8A8125E5F49612F9\", \"0x0ADB92739205B4626FEFB533B38D0071EB018E6FF096C98C17A6826B536817B\" ], \"contract_address\": \"0x0019fcae2482de8fb3afaf8d4b219449bec93a5928f02f58eef645cc071767f4\", \"calldata\": [ \"0x0000000000000000000000000000000000000000000000000000000000000001\", \"0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7\", \"0x0083afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e\", \"0x0000000000000000000000000000000000000000000000000000000000000000\", \"0x0000000000000000000000000000000000000000000000000000000000000003\", \"0x0000000000000000000000000000000000000000000000000000000000000003\", \"0x04681402a7ab16c41f7e5d091f32fe9b78de096e0bd5962ce5bd7aaa4a441f64\", \"0x000000000000000000000000000000000000000000000000001d41f6331e6800\", \"0x0000000000000000000000000000000000000000000000000000000000000000\", \"0x0000000000000000000000000000000000000000000000000000000000000001\" ], \"entry_point_selector\": \"0x015d40a3d6ca2ac30f4031e42be28da9b056fef9bb7357ac5e85627ee876e5ad\" }".to_string();

    let block_id = block_id_string_to_block_id_type(&block_type, &block_hash).unwrap();
    let broadcasted_transaction: BroadcastedTransaction =
        serde_json::from_str(&broadcasted_transaction).unwrap();

    Mock::given(method("POST"))
        .and(body_json(StarknetRpcBaseData::starknet_estimate_fee((
            broadcasted_transaction,
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

fn response_template_with_status(status_code: StatusCode) -> ResponseTemplate {
    ResponseTemplate::new(status_code)
        .append_header("vary", "Accept-Encoding")
        .append_header("vary", "Origin")
}
