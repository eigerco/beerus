use beerus_core::{config::Config, lightclient::beerus::BeerusLightClient};
use env_logger::Env;
use eyre::Result;
use starknet::{
    core::types::FieldElement,
    core::types::{BlockId, FunctionCall},
};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::from_env();
    let mut beerus = BeerusLightClient::new(config.clone()).await?;
    beerus.start().await?;

    let starkgate_addr = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
    let name_selector = "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60";

    let calldata = FunctionCall {
        contract_address: FieldElement::from_str(starkgate_addr).unwrap(),
        entry_point_selector: FieldElement::from_str(name_selector).unwrap(),
        calldata: vec![],
    };
    let block_id = BlockId::Number(33482);

    let res = beerus
        .starknet_lightclient
        .call(calldata, &block_id)
        .await?;
    println!("{:?}", res);
    Ok(())
}
