#[cfg(test)]
mod tests {
    use beerus_core::{
        config::Config,
        lightclient::{
            beerus::{BeerusLightClient, SyncStatus},
            ethereum::{helios_lightclient::HeliosLightClient, MockEthereumLightClient},
            starknet::{MockStarkNetLightClient, StarkNetLightClient, StarkNetLightClientImpl},
        },
    };
    use ethers::types::Address;
    use eyre::eyre;
    use helios::types::BlockTag;
    use primitive_types::U256;
    use starknet::{core::types::FieldElement, macros::selector};
    use std::str::FromStr;

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

    /// Test the `get_nonce` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_nonce` method of the external dependencies.
    /// It tests the `get_nonce` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_nonce_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the `get_nonce` method of the Ethereum light client.
        ethereum_lightclient_mock
            .expect_get_nonce()
            .return_once(move |_, _| Ok(123));

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let address = "0xc24215226336d22238a20a72f8e489c005b44c4a".to_string();

        let addr: Address = Address::from_str(&address).unwrap();

        let block = BlockTag::Latest;

        // Query the balance of the Ethereum address.
        let result = beerus
            .ethereum_lightclient
            .get_nonce(&addr, block)
            .await
            .unwrap();

        // Assert that the `get_nonce` method of the Beerus light client returns `123`.
        assert_eq!("123", result.to_string());
    }

    /// Test the `get_nonce` method when the Ethereum light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_nonce` method of the external dependencies.
    /// It tests the `get_nonce` method of the Beerus light client.
    /// It tests the error handling of the `start` method of the Beerus light client.
    #[tokio::test]
    async fn given_ethereum_lightclient_error_when_call_get_nonce_then_should_return_error() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        let expected_error = "ethereum_lightclient_error";

        // Mock dependencies.
        ethereum_lightclient_mock
            .expect_get_nonce()
            .return_once(move |_, _| Err(eyre::eyre!("ethereum_lightclient_error")));

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let address = "0xc24215226336d22238a20a72f8e489c005b44c4a".to_string();

        let addr: Address = Address::from_str(&address).unwrap();

        let block = BlockTag::Latest;

        // Query the balance of the Ethereum address.
        let result = beerus.ethereum_lightclient.get_nonce(&addr, block).await;

        // Then
        // Assert that the `get_nonce` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_nonce` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
    }

    /// Test the `get_code` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_code` method of the external dependencies.
    /// It tests the `get_code` method of the Beerus light client.    
    #[tokio::test]
    async fn given_normal_conditions_when_get_code_then_should_return_ok() {
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the `get_code` method of the Ethereum light client.
        let expected_code = vec![0, 100, 87, 63];
        ethereum_lightclient_mock
            .expect_get_code()
            .return_once(move |_, _| Ok(expected_code));
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Prepare variables
        let address = "0xc24215226336d22238a20a72f8e489c005b44c4a".to_owned();
        let addr = Address::from_str(&address).unwrap();
        let block = BlockTag::Latest;

        // When
        let result = beerus.ethereum_lightclient.get_code(&addr, block).await;

        // Then
        // Assert that the `get_code` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the code returned byt `get_code` method of the Beerus light client is the expected code.
        assert_eq!(result.unwrap(), vec![0, 100, 87, 63]);
    }

    /// Test the `get_code` method when the Ethereum light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_code` method of the external dependencies.
    /// It tests the `get_code` method of the Beerus light client.
    /// It tests the error handling of the `start` method of the Beerus light client.
    #[tokio::test]
    async fn given_ethereum_lightclient_error_when_call_get_code_then_should_return_error() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        let expected_error = "ethereum_lightclient_error";

        // Mock dependencies.
        ethereum_lightclient_mock
            .expect_get_code()
            .return_once(move |_, _| Err(eyre::eyre!("ethereum_lightclient_error")));

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let address = "0xc24215226336d22238a20a72f8e489c005b44c4a".to_owned();

        let addr: Address = Address::from_str(&address).unwrap();

        let block = BlockTag::Latest;

        // Query the balance of the Ethereum address.
        let result = beerus.ethereum_lightclient.get_code(&addr, block).await;

        // Then
        // Assert that the `get_code` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_code` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
    }

    /// Test the `get_block_number` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_block_number` method of the external dependencies.
    /// It tests the `get_block_number` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_block_number_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the `get_block_number` method of the Ethereum light client.
        let expected_block_number = 1;
        ethereum_lightclient_mock
            .expect_get_block_number()
            .return_once(move || Ok(expected_block_number));

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus.ethereum_lightclient.get_block_number().await;

        // Then
        // Assert that the `get_block_number` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the block number returned by the `get_block_number` method of the Beerus light client is the expected block number.
        assert_eq!(result.unwrap(), expected_block_number);
    }

    /// Test the `chain_id` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `chain_id` method of the external dependencies.
    /// It tests the `chain_id` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_chain_id_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the `chain_id` method of the Ethereum light client.
        let expected_chain_id = 1;
        ethereum_lightclient_mock
            .expect_chain_id()
            .return_once(move || expected_chain_id);

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus.ethereum_lightclient.chain_id().await;

        // Then
        // Assert that the chain id returned by the `chain_id` method of the Beerus light client is the expected chain id.
        assert_eq!(result, expected_chain_id);
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

    /// Test that starknet state root is returned when the Ethereum light client returns a value.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_last_proven_block_then_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Expected block number.
        let expected_starknet_block_number = U256::from(10);
        // Convert to bytes because that's what the mock returns.
        let mut expected_starknet_block_number_bytes: Vec<u8> = vec![0; 32];
        expected_starknet_block_number.to_big_endian(&mut expected_starknet_block_number_bytes);

        // Set the expected return value for the Ethereum light client mock.
        ethereum_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_call_opts, _block_tag| Ok(expected_starknet_block_number_bytes));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Perform the test call.
        let starknet_block_number = beerus.starknet_last_proven_block().await.unwrap();

        // Assert that the result is correct.
        assert_eq!(starknet_block_number, expected_starknet_block_number);
    }

    /// Test that starknet state root return an error when the Ethereum Light client returns an error.
    #[tokio::test]
    async fn given_ethereum_light_client_returns_error_when_starknet_last_proven_block_then_should_fail_with_same_error(
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
        let (config, mut ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

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
            .return_once(move |_req, _block_nb| Ok(expected_result));
        ethereum_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Ok(vec![2]));
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
        assert_eq!(res, expected_result2);
    }

    /// Test that starknet call return an error when the StarkNet Light client returns an error.
    #[tokio::test]
    async fn given_starknet_light_client_returns_error_when_starknet_call_should_fail_with_same_error(
    ) {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Set the expected return value for the Starknet light client mock.
        let expected_error = "Wrong url";
        starknet_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Err(eyre!(expected_error)));
        ethereum_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Ok(vec![2]));
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

    /// Test that starknet storage value is returned when the Starknet light client returns a value.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_get_storage_at_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();
        let expected_result = FieldElement::from_hex_be("298305742194").unwrap();
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient_mock
            .expect_get_storage_at()
            .times(1)
            .return_once(move |_address, _key, _block_nb| Ok(expected_result));
        ethereum_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Ok(vec![2]));
        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let address = FieldElement::from_hex_be(
            "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )
        .unwrap();
        let key = selector!("ERC20_name");
        // Perform the test call.
        let res = beerus.starknet_get_storage_at(address, key).await.unwrap();

        assert_eq!(res, expected_result);
    }

    /// Test that starknet get_storage_at return an error when the StarkNet Light client returns an error.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_get_storage_at_should_fail_with_same_error(
    ) {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Set the expected return value for the Starknet light client mock.
        let expected_error = "Wrong url";
        starknet_lightclient_mock
            .expect_get_storage_at()
            .times(1)
            .return_once(move |_address, _key, _block_nb| Err(eyre!(expected_error)));
        ethereum_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Ok(vec![2]));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let address = FieldElement::from_hex_be(
            "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )
        .unwrap();
        let key = selector!("ERC20_name");

        // Perform the test call.
        let res = beerus.starknet_get_storage_at(address, key).await;

        // Assert that the result is correct.
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), expected_error);
    }

    /// Test that starknet get_nonce.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_get_nonce_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();
        let expected_result = FieldElement::from_hex_be("298305742194").unwrap();
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient_mock
            .expect_get_nonce()
            .return_once(move |_block_nb, _address| Ok(expected_result));
        ethereum_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Ok(vec![2]));
        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let address = FieldElement::from_hex_be(
            "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )
        .unwrap();
        // Get nonce
        let res = beerus.starknet_get_nonce(address).await.unwrap();

        assert_eq!(res, expected_result);
    }

    /// Test that starknet get_nonce.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_get_nonce_should_fail_with_same_error(
    ) {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Set the expected return value for the Starknet light client mock.
        let expected_error = "Wrong url";
        starknet_lightclient_mock
            .expect_get_nonce()
            .return_once(move |_block_nb, _address| Err(eyre!(expected_error)));
        ethereum_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Ok(vec![2]));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let address = FieldElement::from_hex_be(
            "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )
        .unwrap();

        // Get Nonce.
        let res = beerus.starknet_get_nonce(address).await;

        // Assert that the result is correct.
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), expected_error);
    }

    /// Test that with a correct url we can create StarkNet light client.
    #[test]
    fn given_normal_conditions_when_create_sn_lightclient_should_work() {
        // Mock config.
        let (config, _, _) = mock_clients();
        // Create a new StarkNet light client.
        let sn_light_client = StarkNetLightClientImpl::new(&config);
        assert!(sn_light_client.is_ok());
    }

    /// Test that starknet light client starts.
    #[tokio::test]
    async fn given_normal_conditions_when_start_sn_lightclient_should_work() {
        // Mock config.
        let (config, _, _) = mock_clients();
        // Create a new StarkNet light client.
        let sn_light_client = StarkNetLightClientImpl::new(&config).unwrap();
        assert!(sn_light_client.start().await.is_ok());
    }

    /// Test that with a wrong url we can't create StarkNet light client.
    #[test]
    fn given_wrong_url_when_create_sn_lightclient_should_fail() {
        // Mock config.
        let config = Config {
            ethereum_network: "mainnet".to_string(),
            ethereum_consensus_rpc: "http://localhost:8545".to_string(),
            ethereum_execution_rpc: "http://localhost:8545".to_string(),
            starknet_rpc: "mainnet".to_string(),
            starknet_core_contract_address: Address::from_str(
                "0x0000000000000000000000000000000000000000",
            )
            .unwrap(),
        };
        // Create a new StarkNet light client.
        let sn_light_client = StarkNetLightClientImpl::new(&config);
        assert!(sn_light_client.is_err());
        assert!(sn_light_client
            .err()
            .unwrap()
            .to_string()
            .contains("relative URL without a base"));
    }

    /// Test that we can create a Helios light client.
    #[test]
    fn given_normal_conditions_when_create_helios_lightclient_should_work() {
        // Mock config.
        let (config, _, _) = mock_clients();
        // Create a new Helios light client.
        let helios_light_client = HeliosLightClient::new(config);
        assert!(helios_light_client.is_ok());
    }

    /// Test that cancellation timestamp is returned when the Ethereum light client returns a value.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_l1_to_l2_message_cancellations_then_should_work()
    {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Expected block number.
        let expected_timestamp = U256::from(1234);
        // Convert to bytes because that's what the mock returns.
        let mut expected_timestamp_bytes: Vec<u8> = vec![0; 32];
        expected_timestamp.to_big_endian(&mut expected_timestamp_bytes);

        // Set the expected return value for the Ethereum light client mock.
        ethereum_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_call_opts, _block_tag| Ok(expected_timestamp_bytes));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Perform the test call.
        let cancellation_timestamp = beerus
            .starknet_l1_to_l2_message_cancellations(U256::from(0))
            .await
            .unwrap();

        // Assert that the result is correct.
        assert_eq!(cancellation_timestamp, expected_timestamp);
    }

    /// Test that starknet_l1_to_l2_message_cancellations return an error when the Ethereum Light client returns an error.
    #[tokio::test]
    async fn given_ethereum_light_client_returns_error_when_starknet_l1_to_l2_message_cancellations_then_should_fail_with_same_error(
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
        let result = beerus
            .starknet_l1_to_l2_message_cancellations(U256::from(0))
            .await;

        // Assert that the result is correct.
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), expected_error);
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
