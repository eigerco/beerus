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
    let config: dto::Config = serde_json::from_str(config_json)
        .map_err(|e| JsValue::from_str(&format!("failed to parse config: {e:?}")))?;
    web_sys::console::log_1(&"beerus: config parsed".into());

    let config = Config {
        network: Network::from_str(&config.network)
            .map_err(|e| JsValue::from_str(&format!("unrecognized network: {e:?}")))?,
        eth_execution_rpc: config.ethereum_url,
        starknet_rpc: config.starknet_url,
        data_dir: Default::default(),
        poll_secs: Default::default(),
        rpc_addr: ([0, 0, 0, 0], 0).into(),
    };

    let beerus = crate::client::Client::new(&config)
        .await
        .map_err(|e| JsValue::from_str(&format!("client failed: {e:?}")))?;
    web_sys::console::log_1(&"beerus: client created".into());

    beerus.start()
        .await
        .map_err(|e| JsValue::from_str(&format!("start failed: {e:?}")))?;
    web_sys::console::log_1(&"beerus: client started".into());

    let state = beerus.get_state()
        .await
        .map_err(|e| JsValue::from_str(&format!("get_state failed: {e:?}")))?;
    web_sys::console::log_1(&"beerus: state ready".into());

    let state = dto::State {
        block_number: state.block_number as i64,
        block_hash: state.block_hash.as_ref().to_owned(),
        root: state.root.as_ref().to_owned(),
    };
    let ret = serde_json::to_string(&state)
        .map_err(|e| JsValue::from_str(&format!("failed to return response: {e:?}")))?;
    web_sys::console::log_1(&"beerus: state done".into());
    Ok(ret)
}
