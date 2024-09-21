use std::rc::Rc;
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

#[derive(Clone)]
pub struct Http(Rc<js_sys::Function>);

impl beerus::gen::client::blocking::HttpClient for Http {
    fn post(
        &self,
        url: &str,
        request: &iamgroot::jsonrpc::Request,
    ) -> std::result::Result<
        iamgroot::jsonrpc::Response,
        iamgroot::jsonrpc::Error,
    > {
        let json = serde_json::to_string(&request)
            .map_err(|e| {
                iamgroot::jsonrpc::Error::new(
                    32101,
                    format!("request failed: {e:?}"),
                )
            })?;
        let result = self.0.as_ref()
            .call2(
                &JsValue::null(), 
                &JsValue::from_str(url), 
                &JsValue::from_str(&json), 
            )
            .map_err(|e| {
                iamgroot::jsonrpc::Error::new(
                    32101,
                    format!("request failed: {e:?}"),
                )
            })?;
        let result = result.as_string()
            .ok_or_else(|| {
                iamgroot::jsonrpc::Error::new(
                    32101,
                    format!("request failed: ¯\\_(ツ)_/¯"),
                )
            })?;
        let response = serde_json::from_str(&result)
            .map_err(|e| {
                iamgroot::jsonrpc::Error::new(
                    32101,
                    format!("request failed: {e:?}"),
                )
            })?;
        Ok(response)
    }
}

#[async_trait::async_trait(?Send)]
impl beerus::gen::client::HttpClient for Http {
    async fn post(
        &self,
        url: &str,
        request: &iamgroot::jsonrpc::Request,
    ) -> std::result::Result<
        iamgroot::jsonrpc::Response,
        iamgroot::jsonrpc::Error,
    > {
        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                iamgroot::jsonrpc::Error::new(
                    32101,
                    format!("request failed: {e:?}"),
                )
            })?
            .json()
            .await
            .map_err(|e| {
                iamgroot::jsonrpc::Error::new(
                    32102,
                    format!("invalid response: {e:?}"),
                )
            })?;
        Ok(response)
    }
}

#[wasm_bindgen]
pub async fn get_state(config_json: &str, f: js_sys::Function) -> Result<String, JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    {
        use num_traits::cast::ToPrimitive;
        web_sys::console::log_1(&format!("size_of::<u64> = {}", std::mem::size_of::<u64>()).into());
        web_sys::console::log_1(&format!("size_of::<usize> = {}", std::mem::size_of::<usize>()).into());

        let x = num_bigint::BigUint::from(42u32);
        let y = x.to_u64().unwrap_or_default();
        web_sys::console::log_1(&format!("bigint: {x:?}, u64: {y:?}").into());

        let x = num_bigint::BigUint::from(18446744073709551615u64);
        let y = x.to_u64().unwrap_or_default();
        let z = x.to_usize().unwrap_or_default();
        web_sys::console::log_1(&format!("bigint: {x:?}, u64: {y:?}, usize: {z:?}").into());
    }

    (&f).call2(&JsValue::null(), &JsValue::from_str("http://localhost:3000/example"), &JsValue::from_str("{}"))?;
    let post = Rc::new(f);

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

    let http = Http(post.clone());
    let beerus = beerus::client::Client::new(&config, http)
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

    let http = Http(post.clone());
    let client = beerus::gen::client::blocking::Client::new(&config.starknet_rpc, http);
    web_sys::console::log_1(&"beerus: rpc client ready".into());
    let json = serde_json::json!({
        "calldata": [],
        "contract_address": "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        "entry_point_selector": "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60"
      });
    let function_call: beerus::gen::FunctionCall = serde_json::from_value(json)
        .map_err(|_| JsValue::from_str("invalid function call"))?;
    web_sys::console::log_1(&format!("beerus: rpc function call: {function_call:?}").into());
    let result = beerus::exe::call(client, function_call, state.clone())
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
