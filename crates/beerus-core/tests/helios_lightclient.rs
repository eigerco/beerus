#![cfg(not(target_arch = "wasm32"))]

pub mod common;
use common::mock_clients;

mod tests {
    use super::*;
    use beerus_core::lightclient::ethereum::{
        helios_lightclient::HeliosLightClient, EthereumLightClient,
    };
    use ethers::types::H256;

    use std::str::FromStr;

    /// Test that we can create a Helios light client.
    #[tokio::test]
    async fn given_normal_conditions_helios_lightclient_should_work() {
        // Mock config.
        let (config, _, _) = mock_clients();

        // Create a new Helios light client.
        let client = HeliosLightClient::new(config).await;
        assert!(client.is_ok()); // Assert that the client creation is successful.
    }

    /// Test that we can start a Helios light client.
    #[tokio::test]
    async fn given_normal_conditions_helios_lightclient_should_start() {
        let (config, mut ethereum_lightclient_mock, _) = mock_clients(); // Mock config.

        // Mock the `start` method of the Ethereum light client.
        ethereum_lightclient_mock
            .expect_start()
            .times(1)
            .return_once(move || Ok(()));

        let mut helios_light_client = HeliosLightClient::new(config).await.unwrap();

        // Start the Helios light client.
        let result = helios_light_client.helios_light_client.start().await;

        assert!(result.is_ok()); // Assert that the test passes.
    }

    #[tokio::test]
    async fn given_normal_conditions_helios_lightclient_should_send_raw_transaction() {
        let (config, mut ethereum_lightclient_mock, _) = mock_clients(); // Mock config.

        let expected_value =
            H256::from_str("0xc9bb964b3fe087354bc1c1904518acc2b9df7ebedcb89215e9f3b41f47b6c31d")
                .unwrap();

        ethereum_lightclient_mock
            .expect_send_raw_transaction()
            .return_once(move |_| Ok(expected_value));

        let bytes = &[10];

        let helios_light_client = HeliosLightClient::new(config).await.unwrap();
        let result = helios_light_client.send_raw_transaction(bytes).await;
        assert_eq!(expected_value, result.unwrap());
    }
}
