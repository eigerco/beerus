use std::str::FromStr;
use wasm_bindgen::prelude::*;

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
    let config: dto::Config =
        serde_json::from_str(config_json).map_err(|e| {
            JsValue::from_str(&format!("failed to parse config: {e:?}"))
        })?;
    web_sys::console::log_1(&"beerus: config parsed".into());

    let config = beerus::config::Config {
        network: helios::prelude::networks::Network::from_str(&config.network)
            .map_err(|e| {
                JsValue::from_str(&format!("unrecognized network: {e:?}"))
            })?,
        eth_execution_rpc: config.ethereum_url,
        starknet_rpc: config.starknet_url,
        data_dir: Default::default(),
        poll_secs: Default::default(),
        rpc_addr: ([0, 0, 0, 0], 0).into(),
    };

    let beerus = beerus::client::Client::new(&config)
        .await
        .map_err(|e| JsValue::from_str(&format!("client failed: {e:?}")))?;
    web_sys::console::log_1(&"beerus: client created".into());

    beerus
        .start()
        .await
        .map_err(|e| JsValue::from_str(&format!("start failed: {e:?}")))?;
    web_sys::console::log_1(&"beerus: client started".into());

    let state = beerus
        .get_state()
        .await
        .map_err(|e| JsValue::from_str(&format!("get_state failed: {e:?}")))?;
    web_sys::console::log_1(&format!("beerus: state {state:?}").into());

    let client = beerus::gen::client::blocking::Client::new(&config.starknet_rpc, beerus::client::Http::new());
    web_sys::console::log_1(&"beerus: rpc client ready".into());
    let json = serde_json::json!({
        "calldata": [],
        "contract_address": "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        "entry_point_selector": "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60"
      });
    let function_call: beerus::gen::FunctionCall = serde_json::from_value(json)
        .map_err(|_| JsValue::from_str("invalid function call"))?;
    web_sys::console::log_1(&format!("beerus: rpc function call: {function_call:?}").into());
    let result = beerus::exe::call(&client, function_call, state.clone())
        .map_err(|e| JsValue::from_str(&format!("function call failed: {e:?}")))?;
    web_sys::console::log_1(&format!("beerus: rpc call result: {result:?}").into());

    let state = dto::State {
        block_number: state.block_number as i64,
        block_hash: state.block_hash.as_ref().to_owned(),
        root: state.root.as_ref().to_owned(),
    };
    let ret = serde_json::to_string(&state).map_err(|e| {
        JsValue::from_str(&format!("failed to return response: {e:?}"))
    })?;
    web_sys::console::log_1(&"beerus: call done".into());
    Ok(ret)
}
