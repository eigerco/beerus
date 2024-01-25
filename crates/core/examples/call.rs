use std::env;
use std::str::FromStr;

use beerus_core::client::BeerusClient;
use beerus_core::config::Config;
use eyre::{Context, Result};
use starknet::core::types::{BlockId, FieldElement, FunctionCall};
use starknet::providers::Provider;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let api_key = env::var("ALCHEMY_API_KEY").context("ALCHEMY_API_KEY is missing")?;

    let config = Config {
        network: helios::config::networks::Network::MAINNET,
        eth_execution_rpc: format!("https://eth-mainnet.g.alchemy.com/v2/{api_key}"),
        starknet_rpc: format!("https://starknet-mainnet.g.alchemy.com/v2/{api_key}"),
        ..Default::default()
    };
    let mut beerus = BeerusClient::new(config).await;
    beerus.start().await?;

    let block_id = BlockId::Number(33482);
    let starkgate_addr = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
    let name_selector = "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60";
    let calldata = FunctionCall {
        contract_address: FieldElement::from_str(starkgate_addr).unwrap(),
        entry_point_selector: FieldElement::from_str(name_selector).unwrap(),
        calldata: vec![],
    };

    let res = beerus.starknet_client.call(calldata, &block_id).await?;
    println!("{:?}", res);
    Ok(())
}
