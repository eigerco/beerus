extern crate wasm_bindgen;
extern crate web_sys;

use beerus_core::config::Config;
use beerus_core::lightclient::beerus::{BeerusLightClient, SyncStatus};
use wasm_bindgen::prelude::*;

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
    #[wasm_bindgen]
    pub async fn new(network: String, consensus_rpc: String, execution_rpc: String, starknet_rpc: String) -> Self {
        console_error_panic_hook::set_once();

        let cfg = Config::from_args(network, consensus_rpc, execution_rpc, starknet_rpc);

        let mut beerus = BeerusLightClient::new(cfg);

        beerus.start().await.unwrap();

        Self { beerus }
    }

    #[wasm_bindgen]
    pub fn get_sync_status(&self) -> String {
        match self.beerus.sync_status() {
            &SyncStatus::NotSynced => "not synced".to_string(),
            &SyncStatus::Syncing => "syncing".to_string(),
            &SyncStatus::Synced => "sync successful".to_string(),
        }
    }

    #[wasm_bindgen]
    pub async fn get_block_number(&self) -> u32 {
        self.beerus.starknet_client.block_number().await.unwrap() as u32
    }

    #[wasm_bindgen]
    pub async fn get_starknet_state_root(&self) -> JsValue {
        let root = self.beerus.helios_client.read().await.starknet_state_root().await.unwrap();

        serde_wasm_bindgen::to_value(&root).unwrap()
    }
}
