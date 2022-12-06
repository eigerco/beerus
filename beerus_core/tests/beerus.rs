#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use beerus_core::{
        config::Config,
        lightclient::{
            beerus::BeerusLightClient,
            ethereum::{helios_lightclient::HeliosLightClient, MockEthereumLightClient},
            starknet::{MockStarkNetLightClient, StarkNetLightClientImpl},
        },
    };
    use ethers::types::Address;
    use eyre::eyre;
    use primitive_types::U256;

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

    /// Test that starknet state root is returned when the Ethereum light client returns a value.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_state_root_then_should_work() {
        // Create a new Ethereum light client mock.
        let mut ethereum_lightclient_mock = MockEthereumLightClient::new();
        // Create a new StarkNet light client mock.
        let starknet_lightclient_mock = MockStarkNetLightClient::new();
        // Create a new Config mock.
        let config = mock_config();

        // Expected state root.
        let expected_starknet_state_root =
            U256::from_str("0x5bb9692622e817c39663e69dce50777daf4c167bdfa95f3e5cef99c6b8a344d")
                .unwrap();
        // Convert to bytes because that's what the mock returns.
        let mut expected_starknet_state_root_bytes: Vec<u8> = vec![0; 32];
        expected_starknet_state_root.to_big_endian(&mut expected_starknet_state_root_bytes);

        // Set the expected return value for the Ethereum light client mock.
        ethereum_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_call_opts, _block_tag| Ok(expected_starknet_state_root_bytes));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        )
        .unwrap();

        // Perform the test call.
        let starknet_state_root = beerus.starknet_state_root().await.unwrap();

        // Assert that the result is correct.
        assert_eq!(starknet_state_root, expected_starknet_state_root);
    }

    /// Test that starknet state root is returned when the Ethereum light client returns a value.
    #[tokio::test]
    async fn given_ethereum_light_client_returns_error_when_starknet_state_root_then_should_fail_with_same_error(
    ) {
        // Create a new Ethereum light client mock.
        let mut ethereum_lightclient_mock = MockEthereumLightClient::new();
        // Create a new StarkNet light client mock.
        let starknet_lightclient_mock = MockStarkNetLightClient::new();
        // Create a new Config mock.
        let config = mock_config();

        let expected_error = "Ethereum client out of sync";
        // Set the expected return value for the Ethereum light client mock.
        ethereum_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_call_opts, _block_tag| Err(eyre!(expected_error)));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        )
        .unwrap();

        // Perform the test call.
        let starknet_state_root_result = beerus.starknet_state_root().await;

        // Assert that the result is correct.
        assert!(starknet_state_root_result.is_err());
        assert_eq!(
            starknet_state_root_result.unwrap_err().to_string(),
            expected_error
        );
    }

    fn mock_config() -> Config {
        Config {
            ethereum_network: "mainnet".to_string(),
            ethereum_consensus_rpc: "http://localhost:8545".to_string(),
            ethereum_execution_rpc: "http://localhost:8545".to_string(),
            starknet_rpc: "http://localhost:8545".to_string(),
            starknet_core_contract_address: Address::from_str(
                "0x0000000000000000000000000000000000000000",
            )
            .unwrap(),
        }
    }
}
