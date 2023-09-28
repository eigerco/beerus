#[cfg(not(target_arch = "wasm32"))]
use beerus_core::{
    config::{Config, DEFAULT_BEERUS_RPC_ADDR, DEFAULT_HELIOS_RPC_ADDR},
    lightclient::{
        ethereum::MockEthereumLightClient,
        starknet::{storage_proof::GetProofOutput, MockStarkNetLightClient},
    },
};
use ethers::types::Address;
use httpmock::{prelude::*, Mock};
use serde::{Deserialize, Serialize};
use serde_json::json;
use starknet::core::types::{
    BlockStatus, BlockWithTxs, BroadcastedInvokeTransaction, BroadcastedInvokeTransactionV1,
    BroadcastedTransaction, FieldElement, InvokeTransactionV1, Transaction,
};
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

pub fn mock_clients() -> (Config, MockEthereumLightClient, MockStarkNetLightClient) {
    (
        Config::default(),
        MockEthereumLightClient::new(),
        MockStarkNetLightClient::new(),
    )
}

pub fn mock_invoke_tx_v1(tx_hash: String) -> InvokeTransactionV1 {
    InvokeTransactionV1 {
        transaction_hash: FieldElement::from_hex_be(&tx_hash).unwrap(),
        max_fee: FieldElement::from_hex_be("0").unwrap(),
        signature: Vec::<FieldElement>::new(),
        nonce: FieldElement::from_hex_be("0").unwrap(),
        sender_address: FieldElement::from_hex_be("0x").unwrap(),
        calldata: Vec::<FieldElement>::new(),
    }
}

pub fn mock_broadcasted_transaction() -> BroadcastedTransaction {
    BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(
        BroadcastedInvokeTransactionV1 {
            max_fee: FieldElement::from_hex_be("0").unwrap(),
            signature: Vec::<FieldElement>::new(),
            nonce: FieldElement::from_hex_be("0").unwrap(),
            sender_address: FieldElement::from_hex_be("0").unwrap(),
            calldata: Vec::<FieldElement>::new(),
            is_query: true,
        },
    ))
}

pub fn mock_block_with_txs(
    transactions: Vec<Transaction>,
    block_number: u64,
    status: BlockStatus,
    block_hash: FieldElement,
) -> BlockWithTxs {
    BlockWithTxs {
        status,
        block_hash,
        parent_hash: FieldElement::from_hex_be("0").unwrap(),
        block_number,
        new_root: FieldElement::from_hex_be("0").unwrap(),
        timestamp: 10,
        sequencer_address: FieldElement::from_hex_be("0").unwrap(),
        transactions,
    }
}

pub fn mock_get_contract_storage_proof(server: &MockServer) -> (Mock, GetProofOutput) {
    let path = "tests/common/data/data.json";
    let s = fs::read_to_string(path).unwrap();

    #[derive(Debug, Serialize, Deserialize)]
    struct JsonOutput {
        result: GetProofOutput,
    }
    let output: JsonOutput = serde_json::from_str(&s).unwrap();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "id":1,
            "jsonrpc":"2.0",
            "method":"pathfinder_getProof",
            "params":[
                {
                    "block_number":1
                },
                "0x4d4e07157aeb54abeb64f5792145f2e8db1c83bda01a8f06e050be18cfb8153",
                ["0x1"]
            ]
        }));
        then.status(200)
            .header("content-type", "application/json")
            .body_from_file(path);
    });
    (mock, output.result)
}

pub fn mock_get_storage_at(server: &MockServer) -> Mock {
    server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "id":1,
            "jsonrpc":"2.0",
            "method":"starknet_getStorageAt",
            "params":[
                "0x0",
                "0x0",
                {
                    "block_number": 1
                }
            ]
        }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0x0000000000000000000000000000000000000000000000000000000000000001"
            }));
    })
}

pub fn mock_call(server: &MockServer) -> Mock {
    server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "id":1,
            "jsonrpc":"2.0",
            "method":"starknet_call",
            "params":[
                {
                    "calldata":[

                    ],
                    "contract_address":"0x0",
                    "entry_point_selector":"0x0"
                },
                {
                    "block_number":1
                }
            ]
        }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": ["0x0000000000000000000000000000000000000000000000000000000000000001"]
            }));
    })
}

pub fn mock_server_config(server: &MockServer) -> Config {
    Config {
        ethereum_network: "mainnet".to_string(),
        ethereum_consensus_rpc: server.base_url(),
        ethereum_execution_rpc: server.base_url(),
        starknet_rpc: server.base_url(),
        starknet_core_contract_address: Address::from_str(
            "0x0000000000000000000000000000000000000000",
        )
        .unwrap(),
        data_dir: PathBuf::from("/tmp"),
        poll_interval_secs: Some(5),
        beerus_rpc_address: Some(SocketAddr::from_str(DEFAULT_BEERUS_RPC_ADDR).unwrap()),
        helios_rpc_address: Some(DEFAULT_HELIOS_RPC_ADDR),
        ethereum_checkpoint: None,
    }
}
