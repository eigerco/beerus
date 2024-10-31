use std::rc::Rc;
use wasm_bindgen::prelude::*;

const MAINNET_ETHEREUM_CHAINID: &str = "0x1";
const SEPOLIA_ETHEREUM_CHAINID: &str = "0xaa36a7";

const MAINNET_STARKNET_CHAINID: &str = "0x534e5f4d41494e";
const SEPOLIA_STARKNET_CHAINID: &str = "0x534e5f5345504f4c4941";

pub mod dto {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct State {
        pub len: i64,
        pub hash: String,
        pub root: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Config {
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
        let response = post(&client, url, &request).await?;
        Ok(response)
    }
}

async fn post<Q: serde::Serialize, R: serde::de::DeserializeOwned>(
    client: &reqwest::Client,
    url: &str,
    request: Q,
) -> std::result::Result<R, iamgroot::jsonrpc::Error> {
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

async fn call(client: &reqwest::Client, url: &str, method: &str) -> Result<String, JsValue> {
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": [],
        "id": 0
    });
    let response: serde_json::Value = post(&client, url, &request).await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    response["result"]
        .as_str()
        .map(|result| result.to_owned())
        .ok_or_else(|| JsValue::from_str(&format!("Result missing for '{method}'.")))
}

async fn check(config: &dto::Config) -> Result<(), JsValue> {
    let client = reqwest::Client::new();
    let ethereum_chain = call(&client, &config.ethereum_url, "eth_chainId").await?;
    let starknet_chain = call(&client, &config.starknet_url, "starknet_chainId").await?;
    match (ethereum_chain.as_str(), starknet_chain.as_str()) {
        (MAINNET_ETHEREUM_CHAINID, MAINNET_STARKNET_CHAINID) => Ok(()),
        (SEPOLIA_ETHEREUM_CHAINID, SEPOLIA_STARKNET_CHAINID) => Ok(()),
        _ => {
            Err(JsValue::from_str(&format!("Chain ID mismatch ethereum={ethereum_chain} starknet={starknet_chain}")))
        }
    }
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub struct Beerus {
    beerus: beerus::client::Client<Http>,
    state: Option<beerus::client::State>,
}

#[wasm_bindgen]
impl Beerus {

    #[wasm_bindgen(constructor)]
    pub async fn new(config_json: &str, f: js_sys::Function) -> Result<Beerus, JsValue> {
        let config: dto::Config = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&format!("beerus: invalid config JSON: {e:?}")))?;
        check(&config).await
            .map_err(|e| JsValue::from_str(&format!("beerus: invalid RPC config: {e:?}")))?;
        let config = beerus::config::Config {
            ethereum_rpc: config.ethereum_url,
            starknet_rpc: config.starknet_url,
        };
        let beerus = beerus::client::Client::new(&config, Http(Rc::new(f)))
            .await
            .map_err(|e| JsValue::from_str(&format!("failed to create client: {e:?}")))?;
        web_sys::console::log_1(&"beerus: ready".into());
        Ok(Self { beerus, state: None })
    }

    #[wasm_bindgen]
    pub async fn get_state(&mut self) -> Result<JsValue, JsValue> {
        let state = self.beerus.get_state().await
            .map_err(|e| JsValue::from_str(&format!("failed to get state: {e:?}")))?;

        let ret = serde_json::to_string(&dto::State {
            len: state.block_number as i64,
            hash: state.block_hash.as_ref().to_owned(),
            root: state.root.as_ref().to_owned(),
        }).map_err(|e| JsValue::from_str(&format!("failed to serialize state: {e:?}")))?;
        let ret = JsValue::from_str(&ret);

        self.state = Some(state);
        Ok(ret)
    }

    #[wasm_bindgen]
    pub async fn execute(&mut self, request: &str) -> Result<JsValue, JsValue> {
        if self.state.is_none() {
            let _ = self.get_state().await?;
        }
        let state = self.state.clone().unwrap();

        let request: beerus::gen::FunctionCall = serde_json::from_str(request)
            .map_err(|e| JsValue::from_str(&format!("failed to parse request: {e:?}")))?;

        let result = self.beerus.execute(request, state)
            .map_err(|e| JsValue::from_str(&format!("failed to execute call: {e:?}")))?;

        let result = serde_json::to_string(&result)
            .map_err(|e| JsValue::from_str(&format!("failed to serialize call result: {e:?}")))?;
        Ok(JsValue::from_str(&result))
    }
}
