// use std::{path::PathBuf, str::FromStr};

// use env_logger::Env;
// use ethers::{types::Address, utils};
// use eyre::Result;
// use helios::{config::networks::Network, prelude::*};

use starknet::{
    core::types::FieldElement,
    providers::jsonrpc::models::{BlockId, FunctionCall},
};

use std::str::FromStr;

use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
        starknet::StarkNetLightClientImpl,
    },
};
// use beerus_rpc::BeerusRpc;
use env_logger::Env;
use log::{error, info};
// use std::process::exit;
use eyre::Result;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    // setting BEERUS_CONFIG env loads config from provided file
    env::set_var(
        "BEERUS_CONFIG",
        format!(
            "{}/examples/mainnet.toml",
            env::var("CARGO_MANIFEST_DIR").unwrap()
        ),
    );
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::from_env();
    let ethereum_lightclient = HeliosLightClient::new(config.clone()).await?;
    let starknet_lightclient = StarkNetLightClientImpl::new(&config)?;
    let mut beerus = BeerusLightClient::new(
        config.clone(),
        Box::new(ethereum_lightclient),
        Box::new(starknet_lightclient),
    );
    beerus.start().await?;

    let calldata = FunctionCall {
        contract_address: FieldElement::from_str(
            "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )
        .unwrap(),
        entry_point_selector: FieldElement::from_str(
            "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60",
        )
        .unwrap(),
        calldata: vec![],
    };
    let block_id = BlockId::Number(33482);

    beerus
        .starknet_lightclient
        .call(calldata, &block_id)
        .await?;
    Ok(())
}
