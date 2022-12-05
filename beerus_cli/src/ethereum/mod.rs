use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios::HeliosLightClient,
        starknet::StarkNetLightClient,
    },
};
use ethers::{types::Address, utils};
use eyre::Result;
use helios::types::BlockTag;
use std::str::FromStr;

/// Query the balance of an Ethereum address.
pub async fn query_balance(config: Config, address: String) -> Result<()> {
    // Create a new Ethereum light client.
    let mut ethereum_lightclient = HeliosLightClient::new(&config)?;
    // Create a new StarkNet light client.
    let starknet_lightclient = StarkNetLightClient::new(&config)?;
    // Create a new Beerus light client.
    let mut beerus =
        BeerusLightClient::new(&config, &mut ethereum_lightclient, starknet_lightclient)?;
    // Start the Beerus light client.
    beerus.start().await?;
    // Parse the Ethereum address.
    let addr = Address::from_str(&address)?;

    // TODO: Make the block tag configurable.
    let block = BlockTag::Latest;
    // Query the balance of the Ethereum address.
    let balance = beerus
        .ethereum_lightclient
        .get_balance(&addr, block)
        .await?;
    // Format the balance in Ether.
    let balance_in_eth = utils::format_units(balance, "ether")?;
    println!("{} ETH", balance_in_eth);
    Ok(())
}
