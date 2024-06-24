use std::str::FromStr;

use helios::config::networks::Network;

use crate::config::Config;

use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod dto {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct State {
        pub block_number: i64,
        pub block_hash: String,
        pub root: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Config {
        pub network: String,
        pub ethereum_url: String,
        pub starknet_url: String,
    }
}

#[wasm_bindgen]
pub async fn get_state(config_json: &str) -> Result<String, JsValue> {
    let config: dto::Config = serde_json::from_str(config_json).expect("config");
    let config = Config {
        network: Network::from_str(&config.network).expect("network"),
        eth_execution_rpc: config.ethereum_url,
        starknet_rpc: config.starknet_url,
        data_dir: Default::default(),
        poll_secs: Default::default(),
        rpc_addr: ([0, 0, 0, 0], 0).into(),
    };

    let beerus = crate::client::Client::new(&config)
        .await
        .expect("client failed");
    beerus.start().await.expect("start failed");

    let state = beerus.get_state().await.expect("get_state failed");

    let state = dto::State {
        block_number: state.block_number as i64,
        block_hash: state.block_hash.as_ref().to_owned(),
        root: state.root.as_ref().to_owned(),
    };
    let ret = serde_json::to_string(&state).expect("json");
    Ok(ret)
}
