#[cfg(test)]
mod tests {
    use beerus_core::{
        config::Config,
        lightclient::{
            beerus::BeerusLightClient, ethereum::helios::HeliosLightClient,
            starknet::StarkNetLightClient,
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
        let mut ethereum_lightclient = HeliosLightClient::new(&config).unwrap();
        // Create a new StarkNet light client.
        let starknet_lightclient = StarkNetLightClient::new(&config).unwrap();
        // Create a new Beerus light client.
        let mut beerus =
            BeerusLightClient::new(&config, &mut ethereum_lightclient, starknet_lightclient)
                .unwrap();
        beerus.start().await.unwrap();
        let starknet_state_root = beerus.starknet_state_root().await.unwrap();
        assert!(!starknet_state_root.is_zero());
    }
}
