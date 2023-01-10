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
    use ethers::types::U256;
    use ethers::types::{Address, Transaction, H256};
    use eyre::eyre;
    use helios::types::{BlockTag, CallOpts, ExecutionBlock, Transactions};
    use starknet::{
        core::types::FieldElement,
        macros::selector,
        providers::jsonrpc::models::{BlockHashAndNumber, BlockId},
    };
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

    /// Test the `get_class_hash` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_class_hash` method of the external dependencies.
    /// It tests the `get_class_hash` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_lightclient_call_get_class_hash_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (_, _, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `get_class_hash` method of the Starknet light client.
        let expected_result = FieldElement::from_str("0x0123").unwrap();

        starknet_lightclient_mock
            .expect_get_class_hash_at()
            .return_once(move |_, _| Ok(expected_result));

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let contract_address = FieldElement::from_str("0x0123").unwrap();
        let result = starknet_lightclient_mock
            .get_class_hash_at(&block_id, contract_address)
            .await
            .unwrap();

        // Then
        // Assert that the contract class returned by the `get_class_hash` method of the Beerus light client
        // is the expected contract class.
        assert_eq!(result, expected_result)
    }

    /// Test the `get_class_hash` method when the StarkNet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_class_hash` method of the external dependencies.
    /// It tests the `get_class_hash` method of the Beerus light client.
    /// It tests the error handling of the `get_class_hash` method of the Beerus light client.
    #[tokio::test]
    async fn given_starknet_lightclient_error_when_call_get_class_hash_then_should_return_error() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (_, _, mut starknet_lightclient_mock) = mock_clients();

        let expected_error = "StarkNet light client error";

        // Mock the `get_class_hash` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_get_class_hash_at()
            .return_once(move |_, _| Err(eyre!(expected_error)));

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let contract_address = FieldElement::from_str("0x0123").unwrap();
        let result = starknet_lightclient_mock
            .get_class_hash_at(&block_id, contract_address)
            .await;

        // Assert that the `get_class_hash` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_class_hash` method of the Beerus light client is the expected error.
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

    /// Test the `get_class_at` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_class_at` method of the external dependencies.
    /// It tests the `get_class_at` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_class_at_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (_, _, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `get_class_at` method of the Starknet light client.
        let (expected_result, expected_result_value) =
            beerus_core::starknet_helper::create_mock_contract_class();

        starknet_lightclient_mock
            .expect_get_class_at()
            .return_once(move |_block_id, _contract_address| Ok(expected_result));

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let contract_address = FieldElement::from_str("0x0123").unwrap();
        let result = starknet_lightclient_mock
            .get_class_at(&block_id, contract_address)
            .await
            .unwrap();

        // Then
        // Assert that the contract class returned by the `get_class_at` method of the Beerus light client
        // is the expected contract class.
        assert_eq!(
            serde_json::value::to_value(result).unwrap(),
            expected_result_value
        )
    }

    /// Test the `get_class_at` method when the StarkNet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_class_at` method of the external dependencies.
    /// It tests the `get_class_at` method of the Beerus light client.
    /// It tests the error handling of the `get_class_at` method of the Beerus light client.
    #[tokio::test]
    async fn given_starknet_lightclient_error_when_call_get_class_at_then_should_return_error() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, _, mut starknet_lightclient_mock) = mock_clients();

        let expected_error = "StarkNet light client error";

        // Mock the `get_class_at` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_get_class_at()
            .times(1)
            .return_once(move |_block_id, _contract_address| Err(eyre!(expected_error)));

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let contract_address = FieldElement::from_str("0x0123").unwrap();
        let result = starknet_lightclient_mock
            .get_class_at(&block_id, contract_address)
            .await;

        // Then
        // Assert that the `get_class_at` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_class_at` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
        // Assert that the sync status of the Beerus light client is `SyncStatus::NotSynced`.
    }
}
