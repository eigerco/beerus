#[cfg(test)]
mod tests {
    use beerus_core::{
        config::Config,
        lightclient::{
            beerus::BeerusLightClient, ethereum::helios_lightclient::HeliosLightClient,
            starknet::StarkNetLightClientImpl,
        },
    };

    // TODO: Disabled because of Helios instability.
    // TODO: We need to think how we want to handle integrations tests
    #[ignore]
    #[tokio::test]
    async fn starknet_state_root_works() {
        // Create config.
        let config = Config::default();
        // Create a new Ethereum light client.
        let ethereum_lightclient = HeliosLightClient::new(config.clone()).unwrap();
        // Create a new StarkNet light client.
        let starknet_lightclient = StarkNetLightClientImpl::new(config.clone()).unwrap();
        // Create a new Beerus light client.
        let mut beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        )
        .unwrap();
        // Start the Beerus light client.
        beerus.start().await.unwrap();
        let starknet_state_root = beerus.starknet_state_root().await.unwrap();
        assert!(!starknet_state_root.is_zero());
    }
}
