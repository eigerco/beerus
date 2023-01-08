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
    use ethers::types::U256;
    use ethers::types::{Address, Transaction, H256};
    use eyre::eyre;
    use helios::types::{BlockTag, CallOpts, ExecutionBlock, Transactions};
    use starknet::providers::jsonrpc::models::{BlockHashAndNumber, BlockId};
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

    /// Test the `send_raw_transaction` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `send_raw_transaction` method of the external dependencies.
    /// It tests the `send_raw_transaction` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_send_raw_transaction_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();
        let expected_value =
            H256::from_str("0xc9bb964b3fe087354bc1c1904518acc2b9df7ebedcb89215e9f3b41f47b6c31d")
                .unwrap();

        // H256::new();
        // Mock the `get_balance` method of the Ethereum light client.
        ethereum_lightclient_mock
            .expect_send_raw_transaction()
            .return_once(move |_| Ok(expected_value));

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let bytes = &[10];

        // Query the balance of the Ethereum address.
        let result = beerus
            .ethereum_lightclient
            .send_raw_transaction(bytes)
            .await
            .unwrap();

        // Assert that the `send_raw_transaction` method of the Beerus light client returns `123`.
        assert_eq!(expected_value, result);
    }

    /// Test the `get_send_raw_transaction` method when the Ethereum light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `send_raw_transaction` method of the external dependencies.
    /// It tests the `send_raw_transaction` method of the Beerus light client.
    /// It tests the error handling of the `send_raw_transaction` method of the Beerus light client.
    #[tokio::test]
    async fn given_ethereum_lightclient_error_when_call_send_raw_transaction_then_should_return_error(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        let expected_error = "ethereum_lightclient_error";

        // Mock dependencies.
        ethereum_lightclient_mock
            .expect_send_raw_transaction()
            .return_once(move |_| Err(eyre::eyre!("ethereum_lightclient_error")));

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let bytes = &vec![60, 80, 60];

        // Send raw transaction.
        let result = beerus
            .ethereum_lightclient
            .send_raw_transaction(bytes)
            .await;

        // Then
        // Assert that the `send_raw_transaction` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `send_raw_transaction` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
    }

    /// Test the `get_balance` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_balance` method of the external dependencies.
    /// It tests the `get_balance` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_balance_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the `get_balance` method of the Ethereum light client.
        ethereum_lightclient_mock
            .expect_get_balance()
            .return_once(move |_, _| Ok((123).into()));

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
            .get_balance(&addr, block)
            .await
            .unwrap();

        // Assert that the `get_balance` method of the Beerus light client returns `123`.
        assert_eq!("123", result.to_string());
    }

    /// Test the `get_balance` method when the Ethereum light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_balance` method of the external dependencies.
    /// It tests the `get_balance` method of the Beerus light client.
    /// It tests the error handling of the `get_balance` method of the Beerus light client.
    #[tokio::test]
    async fn given_ethereum_lightclient_error_when_call_get_balance_then_should_return_error() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        let expected_error = "ethereum_lightclient_error";

        // Mock dependencies.
        ethereum_lightclient_mock
            .expect_get_balance()
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
        let result = beerus.ethereum_lightclient.get_balance(&addr, block).await;

        // Then
        // Assert that the `get_balance` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_balance` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
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
    /// Test the `get_block_transaction_count_by_number` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_block_transaction_count_by_number` method of the external dependencies.
    /// It tests the `get_block_transaction_count_by_number` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_query_tx_count_by_block_number_then_ok() {
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the `get_block_transaction_count_by_number` method of the Ethereum light client.
        let expected_code: u64 = 120;
        ethereum_lightclient_mock
            .expect_get_block_transaction_count_by_number()
            .return_once(move |_| Ok(expected_code));
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Prepare variables
        let block = BlockTag::Latest;

        // When
        let result = beerus
            .ethereum_lightclient
            .get_block_transaction_count_by_number(block)
            .await;

        // Then
        // Assert that the `get_block_transaction_count_by_number` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the code returned byt `get_block_transaction_count_by_number` method of the Beerus light client is the expected code.
        assert_eq!(result.unwrap(), 120);
    }

    /// Test the `get_block_transaction_count_by_number` method when the Ethereum light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_block_transaction_count_by_number` method of the external dependencies.
    /// It tests the `get_block_transaction_count_by_number` method of the Beerus light client.
    /// It tests the error handling of the `start` method of the Beerus light client.
    #[tokio::test]
    async fn giver_ethereum_lightclient_returns_error_when_query_tx_count_by_block_number_then_error_is_propagated(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        let expected_error = "ethereum_lightclient_error";

        // Mock dependencies.
        ethereum_lightclient_mock
            .expect_get_block_transaction_count_by_number()
            .return_once(move |_| Err(eyre::eyre!("ethereum_lightclient_error")));

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block = BlockTag::Latest;

        // Query the balance of the Ethereum address.
        let result = beerus
            .ethereum_lightclient
            .get_block_transaction_count_by_number(block)
            .await;

        // Then
        // Assert that the `get_block_transaction_count_by_number` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_block_transaction_count_by_number` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
    }

    /// Test the `get_block_by_number` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_block_by_number` method of the external dependencies.
    /// It tests the `get_block_by_number` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_block_by_number_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the `get_block_by_number` method of the Ethereum light client.
        let expected_block_number = 1;
        let expected_block = Some(ExecutionBlock {
            number: 1,
            base_fee_per_gas: U256::from(1),
            difficulty: U256::from(1),
            extra_data: vec![],
            gas_limit: 1,
            gas_used: 1,
            hash: H256::from_low_u64_be(1),
            logs_bloom: vec![],
            miner: Address::from_low_u64_be(1),
            mix_hash: H256::from_low_u64_be(1),
            nonce: String::from("1"),
            parent_hash: H256::from_low_u64_be(1),
            receipts_root: H256::from_low_u64_be(1),
            sha3_uncles: H256::from_low_u64_be(1),
            size: 1,
            state_root: H256::from_low_u64_be(1),
            timestamp: 1,
            total_difficulty: 1,
            transactions: Transactions::Full(vec![]),
            transactions_root: H256::from_low_u64_be(1),
            uncles: vec![],
        });
        let _expected_block = expected_block.clone();
        ethereum_lightclient_mock
            .expect_get_block_by_number()
            .return_once(move |_, _| Ok(_expected_block));

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus
            .ethereum_lightclient
            .get_block_by_number(BlockTag::Number(expected_block_number), false)
            .await;

        // Then
        // Assert that the `get_block_by_number` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the block returned by the `get_block_by_number` method of the Beerus light client is the expected block.
        let result_json = serde_json::to_string(&result.unwrap()).unwrap();
        let expected_block_json = serde_json::to_string(&expected_block).unwrap();
        assert_eq!(result_json, expected_block_json);
    }

    /// Test the `get_block_by_number` method when the Ethereum light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_block_by_number` method of the external dependencies.
    /// It tests the `get_block_by_number` method of the Beerus light client.
    #[tokio::test]
    async fn given_error_when_call_get_block_by_number_then_should_return_err() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the `get_block_by_number` method of the Ethereum light client.
        let expected_error = "ethereum_lightclient_error".to_string();
        let _expected_error = expected_error.clone();
        ethereum_lightclient_mock
            .expect_get_block_by_number()
            .return_once(move |_, _| Err(eyre!(_expected_error)));

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus
            .ethereum_lightclient
            .get_block_by_number(BlockTag::Latest, false)
            .await;

        // Then
        // Assert that the `get_block_by_number` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_block_by_number` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
    }

    /// Test the `get_block_transaction_count_by_hash` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_block_transaction_count_by_hash` method of the external dependencies.
    /// It tests the `get_block_transaction_count_by_hash` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_query_tx_count_by_block_hash_then_ok() {
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the `get_block_transaction_count_by_number` method of the Ethereum light client.
        let expected_code: u64 = 120;
        ethereum_lightclient_mock
            .expect_get_block_transaction_count_by_hash()
            .return_once(move |_| Ok(expected_code));
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Prepare variables
        let hash = vec![0, 13, 15];

        // When
        let result = beerus
            .ethereum_lightclient
            .get_block_transaction_count_by_hash(&hash)
            .await;

        // Then
        // Assert that the `get_block_transaction_count_by_hash` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the code returned byt `get_block_transaction_count_by_hash` method of the Beerus light client is the expected code.
        assert_eq!(result.unwrap(), 120);
    }

    /// Test the `get_block_transaction_count_by_hash` method when the Ethereum light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_block_transaction_count_by_hash` method of the external dependencies.
    /// It tests the `get_block_transaction_count_by_hash` method of the Beerus light client.
    /// It tests the error handling of the `start` method of the Beerus light client.
    #[tokio::test]
    async fn giver_ethereum_lightclient_returns_error_when_query_tx_count_by_block_hash_then_error_is_propagated(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        let expected_error = "ethereum_lightclient_error";

        // Mock dependencies.
        ethereum_lightclient_mock
            .expect_get_block_transaction_count_by_hash()
            .return_once(move |_| Err(eyre::eyre!("ethereum_lightclient_error")));

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let hash = vec![0, 13, 15];

        // Query the balance of the Ethereum address.
        let result = beerus
            .ethereum_lightclient
            .get_block_transaction_count_by_hash(&hash)
            .await;

        // Then
        // Assert that the `get_block_transaction_count_by_hash` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_block_transaction_count_by_hash` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
    }

    /// Test the `get_transaction_by_hash` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_transaction_by_hash` method of the external dependencies.
    /// It tests the `get_transaction_by_hash` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_query_transaction_by_hash_then_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the `get_transaction_by_hash` method of the Ethereum light client.
        let transaction = Transaction::default();
        let _transaction = transaction.clone();

        // Given
        // Mock dependencies
        ethereum_lightclient_mock
            .expect_get_transaction_by_hash()
            .return_once(move |_| Ok(Some(_transaction)));

        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let tx_hash =
            H256::from_str("0xc9bb964b3fe087354bc1c1904518acc2b9df7ebedcb89215e9f3b41f47b6c31d")
                .unwrap();
        // When
        // Query the transaction data given a hash on Ethereum.
        let result = beerus
            .ethereum_lightclient
            .get_transaction_by_hash(&tx_hash)
            .await;

        // Then
        // Assert that the `query_transaction_by_hash` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the code returned byt `query_transaction_by_hash` method of the Beerus light client is the expected code.
        assert_eq!(result.unwrap(), Some(transaction));
    }

    /// Test the `query_transaction_by_hash` method when the Ethereum light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `query_transaction_by_hash` method of the external dependencies.
    /// It tests the `query_transaction_by_hash` method of the Beerus light client.
    /// It tests the error handling of the `start` method of the Beerus light client.
    #[tokio::test]
    async fn giver_ethereum_lightclient_returns_error_when_query_transaction_by_hash_then_error_is_propagated(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        let expected_error = "ethereum_lightclient_error";

        // Mock dependencies.
        ethereum_lightclient_mock
            .expect_get_transaction_by_hash()
            .return_once(move |_| Err(eyre::eyre!("ethereum_lightclient_error")));

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let tx_hash =
            H256::from_str("0xc9bb964b3fe087354bc1c1904518acc2b9df7ebedcb89215e9f3b41f47b6c31d")
                .unwrap();
        // When
        // Query the transaction data given a hash on Ethereum.
        let result = beerus
            .ethereum_lightclient
            .get_transaction_by_hash(&tx_hash)
            .await;

        // Then
        // Assert that the `query_transaction_by_hash` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `query_transaction_by_hash` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
    }

    /// Test the `gas_price method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `gas_price` method of the external dependencies.
    /// It tests the `gas_price` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_query_gas_price_then_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the `gas_price` method of the Ethereum light client.
        let gas_price = U256::default();

        // Given
        // Mock dependencies
        ethereum_lightclient_mock
            .expect_get_gas_price()
            .return_once(move || Ok(gas_price));

        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // When
        // Query the transaction data given a hash on Ethereum.
        let result = beerus.ethereum_lightclient.get_gas_price().await;

        // Then
        // Assert that the `gas_price` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the code returned byt `gas_price` method of the Beerus light client is the expected code.
        assert_eq!(result.unwrap(), gas_price);
    }

    /// Test the `gas_price` method when the Ethereum light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `gas_price` method of the external dependencies.
    /// It tests the `gas_price` method of the Beerus light client.
    /// It tests the error handling of the `start` method of the Beerus light client.
    #[tokio::test]
    async fn giver_ethereum_lightclient_returns_error_when_query_gas_price_then_error_is_propagated(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        let expected_error = "ethereum_lightclient_error";

        // Mock dependencies.
        ethereum_lightclient_mock
            .expect_get_gas_price()
            .return_once(move || Err(eyre::eyre!("ethereum_lightclient_error")));

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // When
        // Query the transaction data given a hash on Ethereum.
        let result = beerus.ethereum_lightclient.get_gas_price().await;

        // Then
        // Assert that the `gas_price` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `gas_price` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
    }

    /// Test the `estimate_gas` method when everything is fine.
    #[tokio::test]
    async fn given_normal_conditions_when_query_estimate_gas_then_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the `estimate_gas` method of the Ethereum light client.
        let gas = 10_u64;

        let call_opts = CallOpts {
            from: Some(Address::from_low_u64_be(0)),
            to: Address::from_low_u64_be(1),
            gas: Some(U256::from(10_u64)),
            gas_price: Some(U256::from(10_u64)),
            value: Some(U256::from(10_u64)),
            data: Some(vec![0_u8, 1_u8]),
        };

        // Given
        // Mock dependencies
        ethereum_lightclient_mock
            .expect_estimate_gas()
            .return_once(move |_| Ok(gas));

        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // When
        // Query the transaction data given a hash on Ethereum.
        let result = beerus.ethereum_lightclient.estimate_gas(&call_opts).await;

        // Then
        // Assert that the `estimate_gas` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the code returned byt `estimate_gas` method of the Beerus light client is the expected code.
        assert_eq!(result.unwrap(), gas);
    }

    /// Test the `estimate_gas` method when the Ethereum light client returns an error.
    #[tokio::test]
    async fn giver_ethereum_lightclient_returns_error_when_query_estimate_gas_then_error_is_propagated(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        let expected_error = "ethereum_lightclient_error";
        let call_opts = CallOpts {
            from: Some(Address::from_low_u64_be(0)),
            to: Address::from_low_u64_be(1),
            gas: Some(U256::from(10_u64)),
            gas_price: Some(U256::from(10_u64)),
            value: Some(U256::from(10_u64)),
            data: Some(vec![0_u8, 1_u8]),
        };

        // Mock dependencies.
        ethereum_lightclient_mock
            .expect_estimate_gas()
            .return_once(move |_| Err(eyre::eyre!("ethereum_lightclient_error")));

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // When
        // Query the transaction data given a hash on Ethereum.
        let result = beerus.ethereum_lightclient.estimate_gas(&call_opts).await;

        // Then
        // Assert that the `estimate_gas` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `estimate_gas` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
    }

    /// Test the `get_block_by_hash` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_block_by_hash` method of the external dependencies.
    /// It tests the `get_block_by_hash` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_block_by_hash_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the `get_block_by_hash` method of the Ethereum light client.
        let expected_block = Some(ExecutionBlock {
            number: 1,
            base_fee_per_gas: U256::from(1),
            difficulty: U256::from(1),
            extra_data: vec![],
            gas_limit: 1,
            gas_used: 1,
            hash: H256::from_low_u64_be(1),
            logs_bloom: vec![],
            miner: Address::from_low_u64_be(1),
            mix_hash: H256::from_low_u64_be(1),
            nonce: String::from("1"),
            parent_hash: H256::from_low_u64_be(1),
            receipts_root: H256::from_low_u64_be(1),
            sha3_uncles: H256::from_low_u64_be(1),
            size: 1,
            state_root: H256::from_low_u64_be(1),
            timestamp: 1,
            total_difficulty: 1,
            transactions: Transactions::Full(vec![]),
            transactions_root: H256::from_low_u64_be(1),
            uncles: vec![],
        });
        let _expected_block = expected_block.clone();
        ethereum_lightclient_mock
            .expect_get_block_by_hash()
            .return_once(move |_, _| Ok(_expected_block));

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let hash = vec![125, 242, 156];

        let result = beerus
            .ethereum_lightclient
            .get_block_by_hash(hash.as_ref(), false)
            .await;

        // Then
        // Assert that the `get_block_by_hash` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the block returned by the `get_block_by_hash` method of the Beerus light client is the expected block.
        let result_json = serde_json::to_string(&result.unwrap()).unwrap();
        let expected_block_json = serde_json::to_string(&expected_block).unwrap();
        assert_eq!(result_json, expected_block_json);
    }

    /// Test the `get_block_by_hash` method when the Ethereum light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_block_by_hash` method of the external dependencies.
    /// It tests the `get_block_by_hash` method of the Beerus light client.
    #[tokio::test]
    async fn given_error_when_call_get_block_by_hash_then_should_return_err() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the `get_block_by_hash` method of the Ethereum light client.
        let expected_error = "ethereum_lightclient_error".to_string();
        let _expected_error = expected_error.clone();
        ethereum_lightclient_mock
            .expect_get_block_by_hash()
            .return_once(move |_, _| Err(eyre!(_expected_error)));

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let hash = vec![125, 242, 156];

        let result = beerus
            .ethereum_lightclient
            .get_block_by_hash(hash.as_ref(), false)
            .await;

        // Then
        // Assert that the `get_block_by_hash` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_block_by_hash` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
    }

    /// Test the `priority_fee method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `priority_fee` method of the external dependencies.
    /// It tests the `priority_fee` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_query_priority_fee_then_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the `priority_fee` method of the Ethereum light client.
        let priority_fee = U256::default();

        // Given
        // Mock dependencies
        ethereum_lightclient_mock
            .expect_get_priority_fee()
            .return_once(move || Ok(priority_fee));

        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // When
        // Query the transaction data given a hash on Ethereum.
        let result = beerus.ethereum_lightclient.get_priority_fee().await;

        // Then
        // Assert that the `priority_fee` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the code returned byt `priority_fee` method of the Beerus light client is the expected code.
        assert_eq!(result.unwrap(), priority_fee);
    }

    /// Test the `priority_fee` method when the Ethereum light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `priority_fee` method of the external dependencies.
    /// It tests the `priority_fee` method of the Beerus light client.
    /// It tests the error handling of the `start` method of the Beerus light client.
    #[tokio::test]
    async fn giver_ethereum_lightclient_returns_error_when_query_priority_fee_then_error_is_propagated(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        let expected_error = "ethereum_lightclient_error";

        // Mock dependencies.
        ethereum_lightclient_mock
            .expect_get_priority_fee()
            .return_once(move || Err(eyre::eyre!("ethereum_lightclient_error")));

        // When
        let beerus = BeerusLightClient::new(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // When
        // Query the transaction data given a hash on Ethereum.
        let result = beerus.ethereum_lightclient.get_priority_fee().await;

        // Then
        // Assert that the `priority_fee` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `priority_fee` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
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
