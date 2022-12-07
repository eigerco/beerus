#[cfg(test)]
mod tests {
    use beerus_core::{
        config::Config,
        lightclient::{
            beerus::{BeerusLightClient, SyncStatus},
            ethereum::MockEthereumLightClient,
            starknet::{MockStarkNetLightClient, StarkNetLightClientImpl},
        },
    };
    use ethers::types::Address;
    use eyre::eyre;
    use primitive_types::U256;
    use starknet::{
        core::types::FieldElement,
        macros::selector,
        providers::jsonrpc::{HttpTransport, JsonRpcClient},
    };
    use std::str::FromStr;
    use url::Url;

    #[test]
    fn when_call_new_then_should_return_beerus_lightclient() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Then
        assert!(beerus.config.eq(&config));
    }

    /// Test the `start` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `start` method of the external dependencies.
    /// It tests the `start` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_start_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `start` method of the Ethereum light client.
        ethereum_lightclient_mock
            .expect_start()
            .times(1)
            .return_once(move || Ok(()));

        // Mock the `start` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_start()
            .times(1)
            .return_once(move || Ok(()));

        // When
        let mut beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus.start().await;

        // Then
        // Assert that the `start` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the sync status of the Beerus light client is `SyncStatus::Synced`.
        assert_eq!(beerus.sync_status().clone(), SyncStatus::Synced);
    }

    /// Test the `start` method when the Ethereum light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `start` method of the external dependencies.
    /// It tests the `start` method of the Beerus light client.
    /// It tests the error handling of the `start` method of the Beerus light client.
    #[tokio::test]
    async fn given_ethereum_lightclient_error_when_call_start_then_should_return_error() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        let expected_error = "Ethereum light client error";

        // Mock the `start` method of the Ethereum light client.
        ethereum_lightclient_mock
            .expect_start()
            .times(1)
            .return_once(move || Err(eyre!(expected_error)));

        // When
        let mut beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus.start().await;

        // Then
        // Assert that the `start` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `start` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
        // Assert that the sync status of the Beerus light client is `SyncStatus::NotSynced`.
        assert_eq!(beerus.sync_status().clone(), SyncStatus::NotSynced);
    }

    /// Test the `start` method when the StarkNet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `start` method of the external dependencies.
    /// It tests the `start` method of the Beerus light client.
    /// It tests the error handling of the `start` method of the Beerus light client.
    #[tokio::test]
    async fn given_starknet_lightclient_error_when_call_start_then_should_return_error() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let expected_error = "StarkNet light client error";

        // Mock the `start` method of the Ethereum light client.
        // We need to mock the `start` method of the Ethereum light client because it is called before the `start` method of the StarkNet light client.
        ethereum_lightclient_mock
            .expect_start()
            .times(1)
            .return_once(move || Ok(()));

        // Mock the `start` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_start()
            .times(1)
            .return_once(move || Err(eyre!(expected_error)));

        // When
        let mut beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus.start().await;

        // Then
        // Assert that the `start` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `start` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
        // Assert that the sync status of the Beerus light client is `SyncStatus::NotSynced`.
        assert_eq!(beerus.sync_status().clone(), SyncStatus::NotSynced);
    }

    /// Test that starknet state root is returned when the Ethereum light client returns a value.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_state_root_then_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

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
        );

        // Perform the test call.
        let starknet_state_root = beerus.starknet_state_root().await.unwrap();

        // Assert that the result is correct.
        assert_eq!(starknet_state_root, expected_starknet_state_root);
    }

    /// Test that starknet state root return an error when the Ethereum Light client returns an error.
    #[tokio::test]
    async fn given_ethereum_light_client_returns_error_when_starknet_state_root_then_should_fail_with_same_error(
    ) {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Set the expected return value for the Ethereum light client mock.
        let expected_error = "Ethereum client out of sync";
        ethereum_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_call_opts, _block_tag| Err(eyre!(expected_error)));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Perform the test call.
        let starknet_state_root_result = beerus.starknet_state_root().await;

        // Assert that the result is correct.
        assert!(starknet_state_root_result.is_err());
        assert_eq!(
            starknet_state_root_result.unwrap_err().to_string(),
            expected_error
        );
    }

    /// Test that starknet view value is returned when the Starknet light client returns a value.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_call_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let expected_result = vec![
            FieldElement::from_hex_be("0x4e28f97185e801").unwrap(),
            FieldElement::from_hex_be("0x0").unwrap(),
        ];
        // Because FieldElement doesn't have the copy trait
        let expected_result2 = expected_result.clone();

        // Set the expected return value for the Ethereum light client mock.
        starknet_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_req| Ok(expected_result));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Perform the test call.
        let res = beerus
            .starknet_call_contract(
                FieldElement::from_hex_be(
                    "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
                )
                .unwrap(),
                selector!("balanceOf"),
                vec![FieldElement::from_hex_be(
                    "0x0000000000000000000000000000000000000000000000000000000000000001",
                )
                .unwrap()],
            )
            .await
            .unwrap();

        // Assert that the result is correct.
        assert!(!res.is_empty());
        assert!(res == expected_result2);
    }

    #[tokio::test]
    async fn given_starknet_light_client_returns_error_when_starknet_call_should_fail_with_same_error(
    ) {
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Set the expected return value for the Starknet light client mock.
        let expected_error = "Wrong url";
        starknet_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_req| Err(eyre!(expected_error)));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Perform the test call.
        let res = beerus
            .starknet_call_contract(
                FieldElement::from_hex_be(
                    "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
                )
                .unwrap(),
                selector!("balanceOf"),
                vec![FieldElement::from_hex_be(
                    "0x0000000000000000000000000000000000000000000000000000000000000001",
                )
                .unwrap()],
            )
            .await;

        // Assert that the result is correct.
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), expected_error);
    }

    #[test]
    fn test_create_sn_lightclient_with_url_should_work() {
        let (config, _, _) = mock_clients();
        let sn_light_client = StarkNetLightClientImpl::new(&config);
        assert!(sn_light_client.is_ok());
    }

    fn mock_clients() -> (Config, MockEthereumLightClient, MockStarkNetLightClient) {
        let config = Config {
            ethereum_network: "mainnet".to_string(),
            ethereum_consensus_rpc: "http://localhost:8545".to_string(),
            ethereum_execution_rpc: "http://localhost:8545".to_string(),
            starknet_rpc: "http://localhost:8545".to_string(),
            starknet_core_contract_address: Address::from_str(
                "0x0000000000000000000000000000000000000000",
            )
            .unwrap(),
        };
        (
            config,
            MockEthereumLightClient::new(),
            MockStarkNetLightClient::new(),
        )
    }
}
