#[cfg(test)]
mod tests {
    use beerus_core::{
        config::Config,
        lightclient::beerus::{Beerus, BeerusLightClient},
    };

    // TODO: Disabled because of Helios instability.
    // TODO: We need to think how we want to handle integrations tests
    #[ignore]
    #[tokio::test]
    async fn starknet_state_root_works() {
        let config = Config::default();
        let mut beerus = BeerusLightClient::new(config).unwrap();
        beerus.start().await.unwrap();
        let starknet_state_root = beerus.starknet_state_root().await.unwrap();
        assert!(!starknet_state_root.is_zero());
    }
}
