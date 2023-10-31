use beerus_core::client::BeerusClient;
use beerus_core::config::Config;
use starknet::providers::Provider;
use eyre::Result;
use starknet::{
    core::types::FieldElement,
    core::types::{BlockId, FunctionCall},
};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use std::str::FromStr;
use std::env;

#[async_std::main]
async fn main() -> Result<()> {
    // logging
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Configuring beerus via env
    // Set the network to mainnet
    env::set_var("NETWORK", "MAINNET");
    // Set the ethereum execution rpc url. Put your key instead of <YOUR_API_KEY>
    env::set_var("ETH_EXECUTION_RPC", "https://eth-mainnet.g.alchemy.com/v2/<YOUR_API_KEY>");
    // Set the Starknet rpc url. Put your key instead of <YOUR_API_KEY>
    env::set_var("STARKNET_RPC", "https://starknet-mainnet.g.alchemy.com/v2/<YOUR_API_KEY>");

    // Initialize beerus
    let config = Config::from_env();
    let mut beerus = BeerusClient::new(config).await;
    beerus.start().await?;

    // Prepare contract's function call
    let block_id = BlockId::Number(33482);
    let starkgate_addr = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
    let name_selector = "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60";
    let calldata = FunctionCall {
        contract_address: FieldElement::from_str(starkgate_addr).unwrap(),
        entry_point_selector: FieldElement::from_str(name_selector).unwrap(),
        calldata: vec![],
    };

    let res = beerus
        .starknet_client
        .call(calldata, &block_id)
        .await?;

    println!("{:?}", res);
    Ok(())
}
