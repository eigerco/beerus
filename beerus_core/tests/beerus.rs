#[cfg(test)]
mod tests {
    use beerus_core::{
        config::Config,
        lightclient::beerus::{Beerus, BeerusLightClient},
    };

    #[tokio::test]
    async fn starknet_state_root_works() {
        let config = Config::default();
        let mut beerus = BeerusLightClient::new(config).unwrap();
        beerus.start().await.unwrap();

        let starknet_state_root = beerus.starknet_state_root().await.unwrap();
        println!("StarkNet state root: {:?}", starknet_state_root);
    }
}
