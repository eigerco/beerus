use std::str::FromStr;

use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
        starknet::StarkNetLightClientImpl,
    },
};
use eyre::Result;
use log::debug;
use starknet::core::types::FieldElement;

/// Creates and starts beerus light client.
pub async fn load_beerus(config: Config) -> Result<BeerusLightClient> {
    let ethereum_lightclient = HeliosLightClient::new(config.clone())?;
    // Create a new StarkNet light client.
    let starknet_lightclient = StarkNetLightClientImpl::new(&config)?;
    // Create a new Beerus light client.
    let mut beerus = BeerusLightClient::new(
        config,
        Box::new(ethereum_lightclient),
        Box::new(starknet_lightclient),
    );
    // Start the Beerus light client.b46961 (feat(cli): add starknet call contract)
    debug!("Starting the Beerus light client...");
    beerus.start().await?;
    debug!("Beerus light client started and synced.");
    Ok(beerus)
}
/// Query the StarkNet state root.
pub async fn query_starknet_state_root(config: Config) -> Result<()> {
    debug!("Querying the StarkNet state root...");
    // Create a new Ethereum light client.
    let beerus = load_beerus(config).await?;
    // Start the Beerus light client.

    // Call the StarkNet contract to get the state root.
    let state_root = beerus.starknet_state_root().await?;
    println!("{}", state_root);
    Ok(())
}

/// Query a StarkNet contract view.
pub async fn query_starknet_contract_view(
    config: Config,
    address: String,
    selector: String,
    calldata: Vec<String>,
) -> Result<()> {
    debug!("Querying the StarkNet contract view...");
    // Create a new Ethereum light client.
    let beerus = load_beerus(config).await?;
    // Convert address to FieldElement.
    let address = FieldElement::from_str(&address)?;
    // Convert selector to FieldElement.
    let selector = FieldElement::from_str(&selector)?;
    // Convert calldata to FieldElements.
    let calldata = calldata
        .iter()
        .map(|x| FieldElement::from_str(x).unwrap())
        .collect();

    // Call the StarkNet contract to get the state root.
    let view_result = beerus
        .starknet_call_contract(address, selector, calldata)
        .await?;
    println!("{:?}", view_result);
    Ok(())
}
