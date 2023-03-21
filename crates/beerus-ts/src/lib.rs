extern crate console_error_panic_hook;
extern crate web_sys;

// TODO: reduce size of wasm module
use wasm_bindgen::prelude::*;

use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
        starknet::StarkNetLightClientImpl,
    },
};

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct BeerusClient {
    beerus: BeerusLightClient,
}

#[wasm_bindgen]
impl BeerusClient {
    #[wasm_bindgen(constructor)]
    pub async fn new(
        network: String,
        consensus_rpc: String,
        execution_rpc: String,
        starknet_rpc: String,
    ) -> Self {
        console_error_panic_hook::set_once();

        let cfg = Config::from_args(network, consensus_rpc, execution_rpc, starknet_rpc);

        let eth_lc = match HeliosLightClient::new(cfg.clone()).await {
            Ok(eth_lc) => eth_lc,
            Err(err) => {
                panic!("{err}");
            }
        };

        let starknet_lc = match StarkNetLightClientImpl::new(&cfg) {
            Ok(starknet_lc) => starknet_lc,
            Err(err) => {
                panic!("{err}");
            }
        };

        let mut beerus = BeerusLightClient::new(cfg, Box::new(eth_lc), Box::new(starknet_lc));

        beerus.start().await.unwrap();

        Self { beerus }
    }

    #[wasm_bindgen]
    pub fn sync_status(&self) -> String {
        self.beerus.sync_status().to_string()
    }
}
