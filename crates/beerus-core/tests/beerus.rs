#![cfg(not(target_arch = "wasm32"))]

pub mod common;
use common::mock_clients;

#[cfg(test)]
mod tests {
    use super::*;
    use beerus_core::{
        config::Config,
        lightclient::{
            beerus::{BeerusLightClient, NodeData, SyncStatus},
            ethereum::helios_lightclient::HeliosLightClient,
            starknet::{StarkNetLightClient, StarkNetLightClientImpl},
        },
        starknet_helper::{block_id_string_to_block_id_type, create_mock_broadcasted_transaction},
    };
    use ethabi::Uint as U256;
    use ethers::types::{Address, Transaction, H256};

    use eyre::eyre;
    use helios::types::{BlockTag, CallOpts, ExecutionBlock, Transactions};
    use starknet::providers::jsonrpc::{models::PendingBlockWithTxs, JsonRpcError};
    use starknet::{
        core::types::FieldElement,
        macros::selector,
        providers::jsonrpc::models::{
            BlockHashAndNumber, BlockId, BlockStatus, BlockTag as StarknetBlockTag,
            BlockWithTxHashes, BlockWithTxs, BroadcastedDeclareTransaction,
            BroadcastedDeclareTransactionV1, BroadcastedDeployTransaction,
            BroadcastedInvokeTransaction, BroadcastedInvokeTransactionV0,
            BroadcastedInvokeTransactionV1, BroadcastedTransaction, ContractClass,
            DeclareTransactionResult, DeployTransactionResult, EventFilter, FeeEstimate,
            InvokeTransaction, InvokeTransactionReceipt, InvokeTransactionResult,
            InvokeTransactionV0, InvokeTransactionV1, LegacyContractClass,
            LegacyContractEntryPoint, LegacyEntryPointsByType, MaybePendingBlockWithTxHashes,
            MaybePendingBlockWithTxs, MaybePendingTransactionReceipt, StateDiff, StateUpdate,
            SyncStatusType, Transaction as StarknetTransaction, TransactionReceipt,
            TransactionStatus,
        },
    };
    use std::{collections::BTreeMap, str::FromStr, sync::Arc};
    use tokio::sync::RwLock;

    const UNKNOWN_ERROR_CODE: i64 = 520;
    const TRANSACTION_HASH_NOT_FOUND_CODE: i64 = 25;

    const STARKNET_LIGHT_CLIENT_ERROR: &str = "StarkNet light client error";
    const WRONG_URL: &str = "Wrong Url";
    const NETWORK_FAILURE: &str = "Network Failure";
    const TRANSACTION_HASH_NOT_FOUND: &str = "Transaction hash not found";

    #[tokio::test]
    async fn when_call_new_then_should_return_beerus_lightclient() {
        use std::env;
        // Given
        // Mock config from env vars
        env::set_var(
            "ETHEREUM_CONSENSUS_RPC_URL",
            "https://www.lightclientdata.org",
        );
        env::set_var(
            "ETHEREUM_EXECUTION_RPC_URL",
            "https://eth-mainnet.g.alchemy.com/v2/<YOUR_API_KEY>",
        );
        env::set_var(
            "STARKNET_RPC_URL",
            "https://starknet-mainnet.infura.io/v3/<YOUR_API_KEY>",
        );
        let config = Config::from_env();

        // When
        let beerus = BeerusLightClient::new(config.clone()).await.unwrap();

        // Then
        assert!(beerus.config.eq(&config));
    }

    #[test]
    fn when_call_new_from_clients_then_should_return_beerus_lightclient() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // When
        let beerus = BeerusLightClient::new_from_clients(
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
        let mut beerus = BeerusLightClient::new_from_clients(
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
        let mut beerus = BeerusLightClient::new_from_clients(
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
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let bytes = &[10];

        // Query the balance of the Ethereum address.
        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
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
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let bytes = &vec![60, 80, 60];

        // Send raw transaction.
        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
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
        let beerus = BeerusLightClient::new_from_clients(
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
            .lock()
            .await
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
        let beerus = BeerusLightClient::new_from_clients(
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
            .lock()
            .await
            .get_balance(&addr, block)
            .await;

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
        let beerus = BeerusLightClient::new_from_clients(
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
            .lock()
            .await
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
        let beerus = BeerusLightClient::new_from_clients(
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
            .lock()
            .await
            .get_nonce(&addr, block)
            .await;

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
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_block_number()
            .await;

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
        let expected_get_chain_id = 1;
        ethereum_lightclient_mock
            .expect_get_chain_id()
            .return_once(move || Ok(expected_get_chain_id));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_chain_id()
            .await
            .unwrap();

        // Then
        // Assert that the chain id returned by the `chain_id` method of the Beerus light client is the expected chain id.
        assert_eq!(result, expected_get_chain_id);
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
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Prepare variables
        let address = "0xc24215226336d22238a20a72f8e489c005b44c4a".to_owned();
        let addr = Address::from_str(&address).unwrap();
        let block = BlockTag::Latest;

        // When
        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_code(&addr, block)
            .await;

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
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let address = "0xc24215226336d22238a20a72f8e489c005b44c4a".to_owned();

        let addr: Address = Address::from_str(&address).unwrap();

        let block = BlockTag::Latest;

        // Query the balance of the Ethereum address.
        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_code(&addr, block)
            .await;

        // Then
        // Assert that the `get_code` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_code` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
    }

    /// Test the `get_transaction_count` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_transaction_count` method of the external dependencies.
    /// It tests the `get_transaction_count` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_query_tx_count_then_ok() {
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the `get_transaction_count` method of the Ethereum light client.
        let expected_result: u64 = 120;
        ethereum_lightclient_mock
            .expect_get_transaction_count()
            .return_once(move |_, _| Ok(expected_result));
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Prepare variables
        let address = Address::from_str("0xc24215226336d22238a20a72f8e489c005b44c4a").unwrap();
        let block = BlockTag::Latest;

        // When
        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_transaction_count(&address, block)
            .await;

        // Then
        // Assert that the `get_transaction_count` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the code returned byt `get_transaction_count` method of the Beerus light client is the expected code.
        assert_eq!(result.unwrap(), 120);
    }

    /// Test the `get_transaction_count` method when the Ethereum light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_transaction_count` method of the external dependencies.
    /// It tests the `get_transaction_count` method of the Beerus light client.
    /// It tests the error handling of the `start` method of the Beerus light client.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_tx_count_then_error_is_propagated()
    {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        let expected_error = "ethereum_lightclient_error";

        // Mock dependencies.
        ethereum_lightclient_mock
            .expect_get_transaction_count()
            .return_once(move |_, _| Err(eyre::eyre!("ethereum_lightclient_error")));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let address = Address::from_str("0xc24215226336d22238a20a72f8e489c005b44c4a").unwrap();
        let block = BlockTag::Latest;

        // Query the transaction of the Ethereum address from a given block.
        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_transaction_count(&address, block)
            .await;

        // Then
        // Assert that the `get_transaction_count` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_transaction_count` method of the Beerus light client is the expected error.
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
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Prepare variables
        let block = BlockTag::Latest;

        // When
        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
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
    async fn given_ethereum_lightclient_returns_error_when_query_tx_count_by_block_number_then_error_is_propagated(
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
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block = BlockTag::Latest;

        // Query the balance of the Ethereum address.
        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
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
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
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
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
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
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Prepare variables
        let hash = vec![0, 13, 15];

        // When
        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
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
    async fn given_ethereum_lightclient_returns_error_when_query_tx_count_by_block_hash_then_error_is_propagated(
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
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let hash = vec![0, 13, 15];

        // Query the balance of the Ethereum address.
        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
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

        let beerus = BeerusLightClient::new_from_clients(
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
            .lock()
            .await
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
    async fn given_ethereum_lightclient_returns_error_when_query_transaction_by_hash_then_error_is_propagated(
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
        let beerus = BeerusLightClient::new_from_clients(
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
            .lock()
            .await
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

        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // When
        // Query the transaction data given a hash on Ethereum.
        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_gas_price()
            .await;

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
    async fn given_ethereum_lightclient_returns_error_when_query_gas_price_then_error_is_propagated(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock dependencies.
        ethereum_lightclient_mock
            .expect_get_gas_price()
            .return_once(move || {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: "Ethereum lightclient error".to_string(),
                }
                .into())
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // When
        // Query the transaction data given a hash on Ethereum.
        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_gas_price()
            .await;

        // Then
        // Assert that the `gas_price` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `gas_price` method of the Beerus light client is the expected error.
        assert_eq!(
            result.unwrap_err().to_string(),
            "JSON-RPC error: code=520, message=\"Ethereum lightclient error\"".to_string()
        );
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
            to: Some(Address::from_low_u64_be(1)),
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

        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // When
        // Query the transaction data given a hash on Ethereum.
        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
            .estimate_gas(&call_opts)
            .await;

        // Then
        // Assert that the `estimate_gas` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the code returned byt `estimate_gas` method of the Beerus light client is the expected code.
        assert_eq!(result.unwrap(), gas);
    }

    /// Test the `estimate_gas` method when the Ethereum light client returns an error.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_estimate_gas_then_error_is_propagated(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        let expected_error = "ethereum_lightclient_error";
        let call_opts = CallOpts {
            from: Some(Address::from_low_u64_be(0)),
            to: Some(Address::from_low_u64_be(1)),
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
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // When
        // Query the transaction data given a hash on Ethereum.
        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
            .estimate_gas(&call_opts)
            .await;

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
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let hash = vec![125, 242, 156];

        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
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
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let hash = vec![125, 242, 156];

        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
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

        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // When
        // Query the transaction data given a hash on Ethereum.
        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_priority_fee()
            .await;

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
    async fn given_ethereum_lightclient_returns_error_when_query_priority_fee_then_error_is_propagated(
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
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // When
        // Query the transaction data given a hash on Ethereum.
        let result = beerus
            .ethereum_lightclient
            .lock()
            .await
            .get_priority_fee()
            .await;

        // Then
        // Assert that the `priority_fee` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `priority_fee` method of the Beerus light client is the expected error.
        assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
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
        let mut beerus = BeerusLightClient::new_from_clients(
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
        let expected_starknet_state_root_bytes: Vec<u8> = vec![0; 32];
        expected_starknet_state_root.to_big_endian(&mut expected_starknet_state_root_bytes.clone());

        // Set the expected return value for the Ethereum light client mock.
        ethereum_lightclient_mock
            .expect_starknet_state_root()
            .return_once(move || Ok(expected_starknet_state_root));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Perform the test call.
        let starknet_state_root = beerus
            .ethereum_lightclient
            .lock()
            .await
            .starknet_state_root()
            .await
            .unwrap();

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
            .expect_starknet_state_root()
            .return_once(move || Err(eyre!(expected_error)));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Perform the test call.
        let starknet_state_root_result = beerus
            .ethereum_lightclient
            .lock()
            .await
            .starknet_state_root()
            .await;

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
            .expect_starknet_last_proven_block()
            .times(1)
            .return_once(move || Ok(expected_starknet_block_number));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Perform the test call.
        let starknet_block_number = beerus
            .ethereum_lightclient
            .lock()
            .await
            .starknet_last_proven_block()
            .await
            .unwrap();

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
            .expect_starknet_state_root()
            .return_once(move || Err(eyre!(expected_error)));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Perform the test call.
        let starknet_state_root_result = beerus
            .ethereum_lightclient
            .lock()
            .await
            .starknet_state_root()
            .await;

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
            .expect_starknet_last_proven_block()
            .return_once(move || Ok(U256::from(10000)));
        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
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

        starknet_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: WRONG_URL.to_string(),
                })
            });

        ethereum_lightclient_mock
            .expect_starknet_last_proven_block()
            .return_once(move || Ok(U256::from(10)));
        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Perform the test call.
        let result = beerus
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
        assert!(result.is_err());
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, WRONG_URL.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
    }

    /// Test that starknet estimated fee is returned when the Starknet light client returns a value.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_estimate_fee_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let expected_result = FeeEstimate {
            gas_consumed: 0,
            gas_price: 0,
            overall_fee: 0,
        };

        // Set the expected return value for the Ethereum light client mock.
        starknet_lightclient_mock
            .expect_estimate_fee()
            .times(1)
            .return_once(|_request, _block_id| Ok(expected_result));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let request = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(
            BroadcastedInvokeTransactionV1 {
                max_fee: FieldElement::from_hex_be("0").unwrap(),
                signature: Vec::<FieldElement>::new(),
                nonce: FieldElement::from_hex_be("0").unwrap(),
                sender_address: FieldElement::from_hex_be("0").unwrap(),
                calldata: Vec::<FieldElement>::new(),
            },
        ));
        let block_id = BlockId::Number(10);

        // Perform the test estimate.
        let res = beerus.starknet_estimate_fee(request, &block_id).await;

        // // Assert that the result is correct.
        assert!(res.is_ok());
    }

    /// Test that starknet estimate_fee return an error when the StarkNet Light client returns an error
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_estimate_fee_should_fail_with_same_error(
    ) {
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Set the expected return value for the Starknet light client mock.
        let expected_error = JsonRpcError {
            code: 0,
            message: "Wrong Url".to_string(),
        };

        starknet_lightclient_mock
            .expect_estimate_fee()
            .return_once(move |_block_nb, _address| Err(expected_error));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let request = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(
            BroadcastedInvokeTransactionV1 {
                max_fee: FieldElement::from_hex_be("0").unwrap(),
                signature: Vec::<FieldElement>::new(),
                nonce: FieldElement::from_hex_be("0").unwrap(),
                sender_address: FieldElement::from_hex_be("0").unwrap(),
                calldata: Vec::<FieldElement>::new(),
            },
        ));
        let block_id = BlockId::Number(10);

        // Perform the test estimate.
        let res = beerus.starknet_estimate_fee(request, &block_id).await;

        // Assert that the result is correct.
        assert!(res.is_err());
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
            .expect_starknet_last_proven_block()
            .return_once(move || Ok(U256::from(10)));
        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let address = FieldElement::from_hex_be(
            "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )
        .unwrap();
        let key = selector!("ERC20_name");
        let block_id = BlockId::Number(10);
        // Perform the test call.
        let res = beerus
            .starknet_get_storage_at(address, key, &block_id)
            .await
            .unwrap();

        assert_eq!(res, expected_result);
    }

    /// Test that starknet storage value is returned when the Starknet light client returns a value(second scenario)
    #[tokio::test]
    async fn given_normal_conditions_with_second_scenario_when_starknet_get_storage_at_should_work()
    {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let expected_result = FieldElement::from_hex_be("298305742194").unwrap();

        let test_block_with_tx_hashes = BlockWithTxHashes {
            status: BlockStatus::AcceptedOnL2,
            block_hash: FieldElement::from_hex_be("0").unwrap(),
            parent_hash: FieldElement::from_hex_be("0").unwrap(),
            block_number: 10,
            new_root: FieldElement::from_hex_be("0").unwrap(),
            timestamp: 10,
            sequencer_address: FieldElement::from_hex_be("0").unwrap(),
            transactions: Vec::new(),
        };

        let test_block = MaybePendingBlockWithTxHashes::Block(test_block_with_tx_hashes);

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient_mock
            .expect_get_storage_at()
            .times(1)
            .return_once(move |_address, _key, _block_nb| Ok(expected_result));

        starknet_lightclient_mock
            .expect_get_block_with_tx_hashes()
            .times(1)
            .return_once(move |_block_id| Ok(test_block));

        ethereum_lightclient_mock
            .expect_starknet_last_proven_block()
            .return_once(move || Ok(U256::from(10)));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let address = FieldElement::from_hex_be(
            "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )
        .unwrap();

        let key = selector!("ERC20_name");
        let block_id = BlockId::Hash(FieldElement::from_hex_be("0").unwrap());

        // Perform the test call.
        let res = beerus
            .starknet_get_storage_at(address, key, &block_id)
            .await
            .unwrap();

        assert_eq!(res, expected_result);
    }

    /// Test that an error is return when getting storage at an unproven block
    #[tokio::test]
    async fn given_unproven_blockid_when_starknet_get_storage_at_should_fail_with_blockid_not_proven_err(
    ) {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        ethereum_lightclient_mock
            .expect_starknet_last_proven_block()
            .return_once(move || Ok(U256::from(10)));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let address = FieldElement::from_hex_be(
            "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )
        .unwrap();

        let key = selector!("ERC20_name");
        let block_id = BlockId::Number(11);

        // Perform the test call.
        let res = beerus
            .starknet_get_storage_at(address, key, &block_id)
            .await;

        let expected_result = JsonRpcError {
            code: 520,
            message: "BlockId is not proven yet".to_string(),
        };
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), expected_result.to_string());
    }

    /// Test that starknet get_storage_at return an error when the StarkNet Light client returns an error.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_get_storage_at_should_fail_with_same_error(
    ) {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        starknet_lightclient_mock
            .expect_get_storage_at()
            .times(1)
            .return_once(move |_address, _key, _block_nb| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: WRONG_URL.to_string(),
                })
            });
        ethereum_lightclient_mock
            .expect_starknet_last_proven_block()
            .return_once(move || Ok(U256::from(10)));
        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let address = FieldElement::from_hex_be(
            "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )
        .unwrap();
        let key = selector!("ERC20_name");
        let block_id = BlockId::Number(10);
        // Perform the test call.
        let result = beerus
            .starknet_get_storage_at(address, key, &block_id)
            .await;

        // Assert that the result is correct.
        assert!(result.is_err());

        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, WRONG_URL.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
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
            .return_once(move |_req, _block_nb| Ok(vec![2]));
        ethereum_lightclient_mock
            .expect_starknet_last_proven_block()
            .return_once(move || Ok(U256::from(0)));
        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let address = FieldElement::from_hex_be(
            "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )
        .unwrap();

        let block_id = BlockId::Tag(StarknetBlockTag::Latest);

        // Get nonce
        let res = beerus.starknet_get_nonce(address, &block_id).await.unwrap();

        assert_eq!(res, expected_result);
    }

    /// Test that starknet get_nonce return an error when the StarkNet Light client returns an error
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_get_nonce_should_fail_with_same_error(
    ) {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        starknet_lightclient_mock
            .expect_get_nonce()
            .return_once(move |_block_nb, _address| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: WRONG_URL.to_string(),
                })
            });
        ethereum_lightclient_mock
            .expect_starknet_last_proven_block()
            .return_once(move || Ok(U256::from(10)));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let address = FieldElement::from_hex_be(
            "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )
        .unwrap();

        let block_id = BlockId::Tag(StarknetBlockTag::Latest);

        // Get Nonce.
        let result = beerus.starknet_get_nonce(address, &block_id).await;

        // Assert that the result is correct.
        assert!(result.is_err());
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, WRONG_URL.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
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
        let mut config = Config::default();
        config.starknet_rpc = "".to_string();

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
    #[tokio::test]
    async fn given_normal_conditions_when_create_helios_lightclient_should_work() {
        // Mock config.
        let (config, _, _) = mock_clients();
        // Create a new Helios light client.
        let helios_light_client = HeliosLightClient::new(config).await;
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
        let beerus = BeerusLightClient::new_from_clients(
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
        ethereum_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_call_opts, _block_tag| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: "Ethereum client out of sync".to_string(),
                }
                .into())
            });

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
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
        assert_eq!(
            result.unwrap_err().message,
            "JSON-RPC error: code=520, message=\"Ethereum client out of sync\"".to_string()
        );
    }

    /// Test that msg_fee + 1 for the message with the given 'msgHash is returned when the Ethereum light client returns a value.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_l1_to_l2_messages_then_should_work() {
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
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Perform the test call.
        let message_timestamp = beerus
            .starknet_l1_to_l2_messages(U256::from(0))
            .await
            .unwrap();

        // Assert that the result is correct.
        assert_eq!(message_timestamp, expected_timestamp);
    }

    /// Test that starknet_l1_to_l2_messages return an error when the Ethereum Light client returns an error.
    #[tokio::test]
    async fn given_ethereum_light_client_returns_error_when_starknet_l1_to_l2_messages_then_should_fail_with_same_error(
    ) {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Set the expected return value for the Ethereum light client mock.
        ethereum_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_call_opts, _block_tag| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: "Ethereum lightclient error".to_string(),
                }
                .into())
            });

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Perform the test call.
        let result = beerus.starknet_l1_to_l2_messages(U256::from(0)).await;

        // Assert that the result is correct.
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message,
            "JSON-RPC error: code=520, message=\"Ethereum lightclient error\"".to_string()
        );
    }

    /// Test that starknet block is returned when a block number is given and the Starknet light client returns a value.
    #[tokio::test]
    async fn given_block_number_when_starknet_get_block_with_txs_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let block_with_tx_hashes = BlockWithTxs {
            status: BlockStatus::AcceptedOnL2,
            block_hash: FieldElement::from_hex_be("0").unwrap(),
            parent_hash: FieldElement::from_hex_be("0").unwrap(),
            block_number: 10,
            new_root: FieldElement::from_hex_be("0").unwrap(),
            timestamp: 10,
            sequencer_address: FieldElement::from_hex_be("0").unwrap(),
            transactions: Vec::new(),
        };

        let expected_result = MaybePendingBlockWithTxs::Block(block_with_tx_hashes);

        // // Set the expected return value for the Starknet light client mock.
        starknet_lightclient_mock
            .expect_get_block_with_txs()
            .times(1)
            .return_once(|_block_id| Ok(expected_result));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Number(10);

        let res = beerus.get_block_with_txs(&block_id).await;

        // Assert that the result is correct.
        assert!(res.is_ok());
    }

    /// Test that starknet block is returned when a block number is given and the Starknet light client returns a value(second scenerio).
    #[tokio::test]
    async fn given_block_number_second_scenerio_when_starknet_get_block_with_txs_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        let block_number = 10;

        let block_with_tx_hashes = BlockWithTxs {
            status: BlockStatus::AcceptedOnL2,
            block_hash: FieldElement::from_hex_be("0").unwrap(),
            parent_hash: FieldElement::from_hex_be("0").unwrap(),
            block_number,
            new_root: FieldElement::from_hex_be("0").unwrap(),
            timestamp: 10,
            sequencer_address: FieldElement::from_hex_be("0").unwrap(),
            transactions: Vec::new(),
        };

        let mut btree_map: BTreeMap<u64, BlockWithTxs> = BTreeMap::new();
        btree_map.insert(block_number, block_with_tx_hashes);

        let node_data = NodeData {
            block_number,
            state_root: String::from("0"),
            payload: btree_map,
        };

        // Create a new Beerus light client.
        let mut beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );
        beerus.node = Arc::new(RwLock::new(node_data));

        let block_id = BlockId::Number(10);

        let res = beerus.get_block_with_txs(&block_id).await;
        // Assert that the result is correct.
        assert!(res.is_ok());
    }

    /// Test that starknet block is returned when a block tag is given and the Starknet light client returns a value.
    #[tokio::test]
    async fn given_block_tag_when_starknet_get_block_with_txs_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let block_with_tx_hashes = BlockWithTxs {
            status: BlockStatus::AcceptedOnL2,
            block_hash: FieldElement::from_hex_be("0").unwrap(),
            parent_hash: FieldElement::from_hex_be("0").unwrap(),
            block_number: 10,
            new_root: FieldElement::from_hex_be("0").unwrap(),
            timestamp: 10,
            sequencer_address: FieldElement::from_hex_be("0").unwrap(),
            transactions: Vec::new(),
        };

        let expected_result = MaybePendingBlockWithTxs::Block(block_with_tx_hashes);

        // // Set the expected return value for the Starknet light client mock.
        starknet_lightclient_mock
            .expect_get_block_with_txs()
            .times(1)
            .return_once(|_block_id| Ok(expected_result));

        starknet_lightclient_mock
            .expect_block_number()
            .times(1)
            .return_once(|| Ok(10));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Tag(StarknetBlockTag::Latest);

        let res = beerus.get_block_with_txs(&block_id).await;

        // Assert that the result is correct.
        assert!(res.is_ok());
    }

    /// Test that starknet block is returned when a block hash is given and the Starknet light client returns a value.
    #[tokio::test]
    async fn given_block_hash_when_starknet_get_block_with_txs_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let block_with_tx_hashes = BlockWithTxs {
            status: BlockStatus::AcceptedOnL2,
            block_hash: FieldElement::from_hex_be("0").unwrap(),
            parent_hash: FieldElement::from_hex_be("0").unwrap(),
            block_number: 10,
            new_root: FieldElement::from_hex_be("0").unwrap(),
            timestamp: 10,
            sequencer_address: FieldElement::from_hex_be("0").unwrap(),
            transactions: Vec::new(),
        };

        let expected_result = MaybePendingBlockWithTxs::Block(block_with_tx_hashes);

        // // Set the expected return value for the Starknet light client mock.
        starknet_lightclient_mock
            .expect_get_block_with_txs()
            .times(1)
            .return_once(|_block_id| Ok(expected_result));

        starknet_lightclient_mock
            .expect_block_number()
            .times(1)
            .return_once(|| Ok(10));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Hash(FieldElement::from_hex_be("0").unwrap());

        let res = beerus.get_block_with_txs(&block_id).await;

        // Assert that the result is correct.
        assert!(res.is_ok());
    }

    // Test that starknet block return an error when the StarkNet Light client returns an error
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_get_block_with_txs_should_fail_with_same_error(
    ) {
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Set the expected return value for the Starknet light client mock.
        let expected_error = JsonRpcError {
            code: 0,
            message: "Wrong Url".to_string(),
        };

        starknet_lightclient_mock
            .expect_get_block_with_txs()
            .return_once(move |_block_id| Err(expected_error));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Number(10);

        // // Perform the get_block_with_txs
        let res = beerus.get_block_with_txs(&block_id).await;

        // Assert that the result is correct.
        assert!(res.is_err());
    }

    /// Test that starknet block hash and number is returned when the Starknet light client returns a value.
    #[tokio::test]
    async fn given_normal_condition_get_block_hash_and_number_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        let block_number = 10;

        let block_with_tx_hashes = BlockWithTxs {
            status: BlockStatus::AcceptedOnL2,
            block_hash: FieldElement::from_hex_be("0").unwrap(),
            parent_hash: FieldElement::from_hex_be("0").unwrap(),
            block_number,
            new_root: FieldElement::from_hex_be("0").unwrap(),
            timestamp: 10,
            sequencer_address: FieldElement::from_hex_be("0").unwrap(),
            transactions: Vec::new(),
        };

        let mut btree_map: BTreeMap<u64, BlockWithTxs> = BTreeMap::new();
        btree_map.insert(block_number, block_with_tx_hashes);

        let node_data = NodeData {
            block_number,
            state_root: String::from("0"),
            payload: btree_map,
        };

        // Create a new Beerus light client.
        let mut beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        beerus.node = Arc::new(RwLock::new(node_data));

        let res = beerus.get_block_hash_and_number().await;

        // Assert that the result is correct.
        assert!(res.is_ok());
    }

    /// Test that starknet block hash and number returns error when block number is not found in payload but Starknet light client returns a value.
    #[tokio::test]
    async fn given_error_condition_when_get_block_hash_and_number_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let res = beerus.get_block_hash_and_number().await;

        let expected_error = JsonRpcError {
            code: 24,
            message: "Block not found".to_string(),
        };
        // Assert that the result is correct.
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), expected_error.to_string());
    }

    /// Test that starknet gets transaction receipt when Starknet light client and Ethereum light mock returns a value.
    #[tokio::test]
    async fn given_normal_condition_starknet_get_transaction_receipt_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let tx_hash = String::from("0x1234");

        let tx_receipt = TransactionReceipt::Invoke(InvokeTransactionReceipt {
            transaction_hash: FieldElement::from_hex_be(&tx_hash).unwrap(),
            actual_fee: FieldElement::from_hex_be("10").unwrap(),
            status: TransactionStatus::AcceptedOnL2,
            block_hash: FieldElement::from_hex_be("0x5678").unwrap(),
            block_number: 10,
            messages_sent: Vec::<_>::new(),
            events: Vec::<_>::new(),
        });

        let expected_result = MaybePendingTransactionReceipt::Receipt(tx_receipt);

        ethereum_lightclient_mock
            .expect_starknet_state_root()
            .times(1)
            .return_once(|| Ok(U256::from("1")));

        starknet_lightclient_mock
            .expect_get_transaction_receipt()
            .times(1)
            .return_once(|_tx_hash| Ok(expected_result));

        let block_number = 10;

        let block_with_tx_hashes = BlockWithTxs {
            status: BlockStatus::AcceptedOnL2,
            block_hash: FieldElement::from_hex_be("0").unwrap(),
            parent_hash: FieldElement::from_hex_be("0").unwrap(),
            block_number,
            new_root: FieldElement::from_hex_be("0").unwrap(),
            timestamp: 10,
            sequencer_address: FieldElement::from_hex_be("0").unwrap(),
            transactions: Vec::new(),
        };

        let mut btree_map: BTreeMap<u64, BlockWithTxs> = BTreeMap::new();
        btree_map.insert(block_number, block_with_tx_hashes);

        let node_data = NodeData {
            block_number,
            state_root: String::from("1"),
            payload: btree_map,
        };

        // Create a new Beerus light client.
        let mut beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        beerus.node = Arc::new(RwLock::new(node_data));

        let res = beerus.starknet_get_transaction_receipt(tx_hash).await;

        // Assert that the result is correct.
        assert!(res.is_ok());
    }

    /// Test that starknet gets transaction receipt returns error when state root on node data is not equal to ethereum light client state root.
    #[tokio::test]
    async fn given_error_condition_when_starknet_get_transaction_receipt_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        ethereum_lightclient_mock
            .expect_starknet_state_root()
            .times(1)
            .return_once(|| Ok(U256::from("0x1234")));

        let block_number = 10;

        let block_with_tx_hashes = BlockWithTxs {
            status: BlockStatus::AcceptedOnL2,
            block_hash: FieldElement::from_hex_be("0").unwrap(),
            parent_hash: FieldElement::from_hex_be("0").unwrap(),
            block_number,
            new_root: FieldElement::from_hex_be("0").unwrap(),
            timestamp: 10,
            sequencer_address: FieldElement::from_hex_be("0").unwrap(),
            transactions: Vec::new(),
        };

        let mut btree_map: BTreeMap<u64, BlockWithTxs> = BTreeMap::new();
        btree_map.insert(block_number, block_with_tx_hashes);

        let node_data = NodeData {
            block_number,
            state_root: String::from("0x5678"),
            payload: btree_map,
        };

        // Create a new Beerus light client.
        let mut beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        beerus.node = Arc::new(RwLock::new(node_data));

        let tx_hash = String::from("0x1234");

        let res = beerus.starknet_get_transaction_receipt(tx_hash).await;

        let expected_error = JsonRpcError {
            code: 520,
            message: "State root mismatch".to_string(),
        };
        // Assert that the result is correct.
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), expected_error.to_string());
    }

    /// Test that starknet gets transaction hash when Starknet light client returns a value.
    #[tokio::test]
    async fn given_normal_condition_get_transaction_by_hash_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let tx_hash = String::from("0x1234");

        let invoke_tx_v1 = InvokeTransactionV1 {
            transaction_hash: FieldElement::from_hex_be(&tx_hash).unwrap(),
            max_fee: FieldElement::from_hex_be("0").unwrap(),
            signature: Vec::<FieldElement>::new(),
            nonce: FieldElement::from_hex_be("0").unwrap(),
            sender_address: FieldElement::from_hex_be("0x").unwrap(),
            calldata: Vec::<FieldElement>::new(),
        };

        let expected_transaction = StarknetTransaction::Invoke(InvokeTransaction::V1(invoke_tx_v1));

        starknet_lightclient_mock
            .expect_get_transaction_by_hash()
            .times(1)
            .return_once(|_tx_hash| Ok(expected_transaction));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let res = beerus.get_transaction_by_hash(tx_hash).await;

        // Assert that the result is correct.
        assert!(res.is_ok());
    }

    /// Test that starknet gets transaction by block and index when Starknet light client returns a value and when `MaybePendingBlockWithTxs::Block` is returned
    #[tokio::test]
    async fn given_normal_condition_and_block_get_transaction_by_block_and_index_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let status = BlockStatus::Pending;
        let block_hash = FieldElement::from_dec_str("01").unwrap();
        let parent_hash = FieldElement::from_dec_str("01").unwrap();
        let block_number = 0;
        let new_root = FieldElement::from_dec_str("01").unwrap();
        let timestamp: u64 = 0;
        let sequencer_address = FieldElement::from_dec_str("01").unwrap();

        let tx_hash = String::from("0x1234");
        let invoke_tx_v1 = InvokeTransactionV1 {
            transaction_hash: FieldElement::from_hex_be(&tx_hash).unwrap(),
            max_fee: FieldElement::from_hex_be("0").unwrap(),
            signature: Vec::<FieldElement>::new(),
            nonce: FieldElement::from_hex_be("0").unwrap(),
            sender_address: FieldElement::from_hex_be("0x").unwrap(),
            calldata: Vec::<FieldElement>::new(),
        };
        let transaction = StarknetTransaction::Invoke(InvokeTransaction::V1(invoke_tx_v1));
        let transactions = vec![transaction];

        let block = BlockWithTxs {
            status,
            block_hash,
            parent_hash,
            block_number,
            new_root,
            timestamp,
            sequencer_address,
            transactions,
        };

        let expected_block_with_txs = MaybePendingBlockWithTxs::Block(block);

        starknet_lightclient_mock
            .expect_get_block_with_txs()
            .times(1)
            .return_once(|_block_id| Ok(expected_block_with_txs));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Number(1);
        let index = 0;
        let res = beerus
            .get_transaction_by_block_and_index(&block_id, index)
            .await;

        // Assert that the result is correct.
        assert!(res.is_ok());
    }

    /// Test that starknet gets transaction by block and index when Starknet light client returns a value and when `MaybePendingBlockWithTxs::PendingBlock` is returned
    #[tokio::test]
    async fn given_normal_condition_and_pending_block_get_transaction_by_block_and_index_should_work(
    ) {
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let parent_hash = FieldElement::from_dec_str("01").unwrap();
        let timestamp: u64 = 0;
        let sequencer_address = FieldElement::from_dec_str("01").unwrap();

        let tx_hash = String::from("0x1234");
        let invoke_tx_v1 = InvokeTransactionV1 {
            transaction_hash: FieldElement::from_hex_be(&tx_hash).unwrap(),
            max_fee: FieldElement::from_hex_be("0").unwrap(),
            signature: Vec::<FieldElement>::new(),
            nonce: FieldElement::from_hex_be("0").unwrap(),
            sender_address: FieldElement::from_hex_be("0x").unwrap(),
            calldata: Vec::<FieldElement>::new(),
        };
        let transaction = StarknetTransaction::Invoke(InvokeTransaction::V1(invoke_tx_v1));
        let transactions = vec![transaction];

        let pending_block = PendingBlockWithTxs {
            timestamp,
            sequencer_address,
            transactions,
            parent_hash,
        };

        let expected_block_with_txs = MaybePendingBlockWithTxs::PendingBlock(pending_block);

        starknet_lightclient_mock
            .expect_get_block_with_txs()
            .times(1)
            .return_once(|_block_id| Ok(expected_block_with_txs));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Number(1);
        let index = 0;
        let res = beerus
            .get_transaction_by_block_and_index(&block_id, index)
            .await;

        // Assert that the result is correct.
        assert!(res.is_ok());
    }

    /// Test that starknet gets transaction count when Starknet light client returns a value and when `MaybePendingBlockWithTxs::Block` is returned
    #[tokio::test]
    async fn given_normal_condition_and_block_get_transaction_count_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let status = BlockStatus::Pending;
        let block_hash = FieldElement::from_dec_str("01").unwrap();
        let parent_hash = FieldElement::from_dec_str("01").unwrap();
        let block_number = 0;
        let new_root = FieldElement::from_dec_str("01").unwrap();
        let timestamp: u64 = 0;
        let sequencer_address = FieldElement::from_dec_str("01").unwrap();

        let tx_hash = String::from("0x1234");
        let invoke_tx_v1 = InvokeTransactionV1 {
            transaction_hash: FieldElement::from_hex_be(&tx_hash).unwrap(),
            max_fee: FieldElement::from_hex_be("0").unwrap(),
            signature: Vec::<FieldElement>::new(),
            nonce: FieldElement::from_hex_be("0").unwrap(),
            sender_address: FieldElement::from_hex_be("0x").unwrap(),
            calldata: Vec::<FieldElement>::new(),
        };
        let transaction = StarknetTransaction::Invoke(InvokeTransaction::V1(invoke_tx_v1));
        let transactions = vec![transaction];

        let block = BlockWithTxs {
            status,
            block_hash,
            parent_hash,
            block_number,
            new_root,
            timestamp,
            sequencer_address,
            transactions,
        };

        let expected_block_with_txs = MaybePendingBlockWithTxs::Block(block);

        starknet_lightclient_mock
            .expect_get_block_with_txs()
            .times(1)
            .return_once(|_block_id| Ok(expected_block_with_txs));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Number(1);
        let res = beerus.get_block_transaction_count(&block_id).await;

        // Assert that the result is correct.
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1);
    }

    /// Test that starknet gets transaction by block and index when Starknet light client returns a value and when `MaybePendingBlockWithTxs::PendingBlock` is returned
    #[tokio::test]
    async fn given_normal_condition_and_pending_block_get_transaction_count_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let parent_hash = FieldElement::from_dec_str("01").unwrap();
        let timestamp: u64 = 0;
        let sequencer_address = FieldElement::from_dec_str("01").unwrap();

        let tx_hash = String::from("0x1234");
        let invoke_tx_v1 = InvokeTransactionV1 {
            transaction_hash: FieldElement::from_hex_be(&tx_hash).unwrap(),
            max_fee: FieldElement::from_hex_be("0").unwrap(),
            signature: Vec::<FieldElement>::new(),
            nonce: FieldElement::from_hex_be("0").unwrap(),
            sender_address: FieldElement::from_hex_be("0x").unwrap(),
            calldata: Vec::<FieldElement>::new(),
        };
        let transaction = StarknetTransaction::Invoke(InvokeTransaction::V1(invoke_tx_v1));
        let transactions = vec![transaction];

        let pending_block = PendingBlockWithTxs {
            timestamp,
            sequencer_address,
            transactions,
            parent_hash,
        };

        let expected_block_with_txs = MaybePendingBlockWithTxs::PendingBlock(pending_block);

        starknet_lightclient_mock
            .expect_get_block_with_txs()
            .times(1)
            .return_once(|_block_id| Ok(expected_block_with_txs));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Number(1);
        let res = beerus.get_block_transaction_count(&block_id).await;

        // Assert that the result is correct.
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1);
    }

    /// Test the `block_number` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `block_number` method of the external dependencies.
    /// It tests the `block_number` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_block_number_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `block_number` method of the Starknet light client.
        let expected_block_number: u64 = 123456;
        starknet_lightclient_mock
            .expect_block_number()
            .return_once(move || Ok(expected_block_number));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus.starknet_lightclient.block_number().await.unwrap();

        // Then
        // Assert that the block number returned by the `block_number` method of the Beerus light client is the expected block number.
        assert_eq!(result, expected_block_number);
    }

    /// Test the `block_number` method when the StarkNet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `block_number` method of the external dependencies.
    /// It tests the `block_number` method of the Beerus light client.
    /// It tests the error handling of the `block_number` method of the Beerus light client.
    #[tokio::test]
    async fn given_starknet_lightclient_error_when_call_block_number_then_should_return_error() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `block_number` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_block_number()
            .times(1)
            .return_once(move || {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: STARKNET_LIGHT_CLIENT_ERROR.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus.starknet_lightclient.block_number().await;

        // Then
        // Assert that the `block_number` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `block_number` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, STARKNET_LIGHT_CLIENT_ERROR.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
        // Assert that the sync status of the Beerus light client is `SyncStatus::NotSynced`.
        assert_eq!(beerus.sync_status().clone(), SyncStatus::NotSynced);
    }

    // Test the `starknet_l1_to_l2_message_nonce` method when everything is fine.
    // This test mocks external dependencies.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_l1_to_l2_message_nonce_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Expected block number.
        let expected_nonce = U256::from(1234);
        // Convert to bytes because that's what the mock returns.
        let mut expected_nonce_bytes: Vec<u8> = vec![0; 32];
        expected_nonce.to_big_endian(&mut expected_nonce_bytes);

        // Mock the next call to the Ethereum light client (starknet_core.l1ToL2MessageNonce)
        ethereum_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_call_opts, _block_tag| Ok(expected_nonce_bytes));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );
        let result = beerus.starknet_l1_to_l2_message_nonce().await.unwrap();

        // Then
        assert_eq!(expected_nonce, result);
    }

    /// Test the `starknet_l1_to_l2_message_nonce` method when everything is fine.
    /// This test mocks external dependencies.
    #[tokio::test]
    async fn given_ethereum_client_error_when_call_get_l1_to_l2_message_nonce_then_should_return_error(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Mock the next call to the Ethereum light client (starknet_core.l1ToL2MessageNonce)
        ethereum_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_call_opts, _block_tag| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: "Ethereum lightclient error".to_string(),
                }
                .into())
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );
        let result = beerus.starknet_l1_to_l2_message_nonce().await;

        // Then
        // Assert that the `block_number` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `block_number` method of the Beerus light client is the expected error.
        assert_eq!(
            result.unwrap_err().message,
            "JSON-RPC error: code=520, message=\"Ethereum lightclient error\"".to_string()
        );
        // Assert that the sync status of the Beerus light client is `SyncStatus::NotSynced`.
        assert_eq!(beerus.sync_status().clone(), SyncStatus::NotSynced);
    }

    /// Test the `block_hash_and_number` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `block_hash_and_number` method of the external dependencies.
    /// It tests the `block_hash_and_number` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_block_hash_and_number_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `block_hash_and_number` method of the Starknet light client.
        let expected_result = BlockHashAndNumber {
            block_hash: FieldElement::from_dec_str("123456").unwrap(),
            block_number: 123456,
        };
        let expected_block_hash_and_number = expected_result.clone();

        starknet_lightclient_mock
            .expect_block_hash_and_number()
            .return_once(move || Ok(expected_block_hash_and_number));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus
            .starknet_lightclient
            .block_hash_and_number()
            .await
            .unwrap();

        // Then
        // Assert that the block hash and number returned by the `block_hash_and_number` method of the Beerus light client
        // is the expected block hash and number.
        assert_eq!(result.block_hash, expected_result.block_hash);
        assert_eq!(result.block_number, expected_result.block_number);
    }

    /// Test the `block_hash_and_number` method when the StarkNet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `block_hash_and_number` method of the external dependencies.
    /// It tests the `block_hash_and_number` method of the Beerus light client.
    /// It tests the error handling of the `block_hash_and_number` method of the Beerus light client.
    #[tokio::test]
    async fn given_starknet_lightclient_error_when_call_block_hash_and_number_then_should_return_error(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `block_number` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_block_hash_and_number()
            .times(1)
            .return_once(move || {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: STARKNET_LIGHT_CLIENT_ERROR.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus.starknet_lightclient.block_hash_and_number().await;

        // Then
        // Assert that the `block_hash_and_number` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `block_number` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, STARKNET_LIGHT_CLIENT_ERROR.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
        // Assert that the sync status of the Beerus light client is `SyncStatus::NotSynced`.
        assert_eq!(beerus.sync_status().clone(), SyncStatus::NotSynced);
    }

    /// Test the `get_class` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_class` method of the external dependencies.
    /// It tests the `get_class` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_class_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `get_class` method of the Starknet light client.
        let (expected_result, expected_result_value) =
            beerus_core::starknet_helper::create_mock_contract_class();

        starknet_lightclient_mock
            .expect_get_class()
            .return_once(move |_block_id, _class_hash| Ok(expected_result));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let class_hash = FieldElement::from_str("0x0123").unwrap();
        let result = beerus
            .starknet_lightclient
            .get_class(&block_id, class_hash)
            .await
            .unwrap();

        // Then
        // Assert that the contract class returned by the `get_class` method of the Beerus light client
        // is the expected contract class.
        assert_eq!(
            serde_json::value::to_value(result).unwrap(),
            expected_result_value
        )
    }

    /// Test the `get_class` method when the StarkNet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_class` method of the external dependencies.
    /// It tests the `get_class` method of the Beerus light client.
    /// It tests the error handling of the `get_class` method of the Beerus light client.
    #[tokio::test]
    async fn given_starknet_lightclient_error_when_call_get_class_then_should_return_error() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `get_class` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_get_class()
            .times(1)
            .return_once(move |_block_id, _class_hash| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: STARKNET_LIGHT_CLIENT_ERROR.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let class_hash = FieldElement::from_str("0x0123").unwrap();
        let result = beerus
            .starknet_lightclient
            .get_class(&block_id, class_hash)
            .await;

        // Then
        // Assert that the `get_class` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_class` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, STARKNET_LIGHT_CLIENT_ERROR.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
        // Assert that the sync status of the Beerus light client is `SyncStatus::NotSynced`.
        assert_eq!(beerus.sync_status().clone(), SyncStatus::NotSynced);
    }

    /// Test that msg_fee + 1 for the message with the given 'msgHash is returned when the Ethereum light client returns a value.
    #[tokio::test]
    async fn given_normal_conditions_when_query_l2_to_l1_messages_then_should_work() {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Expected fee
        let expected_fee = U256::from(1234);
        // Convert to bytes because that's what the mock returns.
        let mut expected_fee_bytes: Vec<u8> = vec![0; 32];
        expected_fee.to_big_endian(&mut expected_fee_bytes);

        // Set the expected return value for the Ethereum light client mock.
        ethereum_lightclient_mock
            .expect_call()
            .times(1)
            .return_once(move |_call_opts, _block_tag| Ok(expected_fee_bytes));

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Perform the test call.
        let message_fee = beerus
            .starknet_l2_to_l1_messages(U256::from(0))
            .await
            .unwrap();

        // Assert that the result is correct.
        assert_eq!(message_fee, expected_fee);
    }

    /// Test that starknet_l2_to_l1_messages return an error when the Ethereum Light client returns an error.
    #[tokio::test]
    async fn given_ethereum_light_client_returns_error_when_query_l2_to_l1_messages_then_should_fail_with_same_error(
    ) {
        // Mock config, ethereum light client and starknet light client.
        let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();

        // Set the expected return value for the Ethereum light client mock.
        ethereum_lightclient_mock
            .expect_call()
            .return_once(move |_call_opts, _block_tag| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: "Ethereum lightclient error".to_string(),
                }
                .into())
            });

        // Create a new Beerus light client.
        let beerus = BeerusLightClient::new_from_clients(
            config,
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // Perform the test call.
        let result = beerus.starknet_l2_to_l1_messages(U256::from(0)).await;

        // Assert that the result is correct.
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message,
            "JSON-RPC error: code=520, message=\"Ethereum lightclient error\"".to_string()
        );
    }

    /// Test the `get_class_hash` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_class_hash` method of the external dependencies.
    /// It tests the `get_class_hash` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_class_hash_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `get_class_hash` method of the Starknet light client.
        let expected_result = FieldElement::from_str("0x0123").unwrap();

        starknet_lightclient_mock
            .expect_get_class_hash_at()
            .return_once(move |_, _| Ok(expected_result));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let contract_address = FieldElement::from_str("0x0123").unwrap();
        let result = beerus
            .starknet_lightclient
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
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `get_class_hash` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_get_class_hash_at()
            .return_once(move |_, _| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: STARKNET_LIGHT_CLIENT_ERROR.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let contract_address = FieldElement::from_str("0x0123").unwrap();
        let result = beerus
            .starknet_lightclient
            .get_class_hash_at(&block_id, contract_address)
            .await;

        // Assert that the `get_class_hash` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_class_hash` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, STARKNET_LIGHT_CLIENT_ERROR.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
    }

    /// Test the `get_class_at` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_class_at` method of the external dependencies.
    /// It tests the `get_class_at` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_class_at_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `get_class_at` method of the Starknet light client.
        let (expected_result, expected_result_value) =
            beerus_core::starknet_helper::create_mock_contract_class();

        starknet_lightclient_mock
            .expect_get_class_at()
            .return_once(move |_block_id, _contract_address| Ok(expected_result));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let contract_address = FieldElement::from_str("0x0123").unwrap();
        let result = beerus
            .starknet_lightclient
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
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `get_class_at` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_get_class_at()
            .times(1)
            .return_once(move |_block_id, _contract_address| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: STARKNET_LIGHT_CLIENT_ERROR.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let contract_address = FieldElement::from_str("0x0123").unwrap();
        let result = beerus
            .starknet_lightclient
            .get_class_at(&block_id, contract_address)
            .await;

        // Then
        // Assert that the `get_class_at` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_class_at` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, STARKNET_LIGHT_CLIENT_ERROR.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
        // Assert that the sync status of the Beerus light client is `SyncStatus::NotSynced`.
        assert_eq!(beerus.sync_status().clone(), SyncStatus::NotSynced);
    }

    /// Test the `get_block_transaction_count` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_block_transaction_count` method of the external dependencies.
    /// It tests the `get_block_transaction_count` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_block_transaction_count_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `get_block_transaction_count` method of the Starknet light client.
        let expected_result: u64 = 34;
        starknet_lightclient_mock
            .expect_get_block_transaction_count()
            .return_once(move |_block_id| Ok(expected_result));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );
        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let result = beerus
            .starknet_lightclient
            .get_block_transaction_count(&block_id)
            .await
            .unwrap();

        // Then
        // Assert that the number of transactions in a block returned by the `get_block_transaction_count` method of the Beerus light client is the expected number of transactions in a block.
        assert_eq!(result, expected_result);
    }

    /// Test the `get_block_transaction_count` method when the StarkNet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_block_transaction_count` method of the external dependencies.
    /// It tests the `get_block_transaction_count` method of the Beerus light client.
    /// It tests the error handling of the `get_block_transaction_count` method of the Beerus light client.
    #[tokio::test]
    async fn given_starknet_lightclient_error_when_call_get_block_transaction_count_then_should_return_error(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `get_block_transaction_count` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_get_block_transaction_count()
            .times(1)
            .return_once(move |_block_id| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: STARKNET_LIGHT_CLIENT_ERROR.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let result = beerus
            .starknet_lightclient
            .get_block_transaction_count(&block_id)
            .await;

        // Then
        // Assert that the `get_block_transaction_count` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_block_transaction_count` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, STARKNET_LIGHT_CLIENT_ERROR.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
        // Assert that the sync status of the Beerus light client is `SyncStatus::NotSynced`.
        assert_eq!(beerus.sync_status().clone(), SyncStatus::NotSynced);
    }

    // /// Test the `get_logs` when everything is fine.
    // /// This test mocks external dependencies.
    // /// It does not test the `get_logs` method of the external dependencies.
    // /// It tests the `get_logs` method of the Beerus light client.
    // #[tokio::test]
    // async fn given_normal_conditions_when_query_get_logs_then_ok() {
    //     // Given
    //     // Mock config, ethereum light client and starknet light client.
    //     let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();
    //     // Mock the `get_logs` method of the Ethereum light client.
    //     // Given
    //     // Mock dependencies
    //     ethereum_lightclient_mock
    //         .expect_get_logs()
    //         .return_once(move |_, _, _, _, _| Ok(vec![Log::default()]));
    //     // When
    //     let beerus = BeerusLightClient::new_from_clients(
    //         config.clone(),
    //         Box::new(ethereum_lightclient_mock),
    //         Box::new(starknet_lightclient_mock),
    //     );
    //     // Query the transaction data given a hash on Ethereum.
    //     let result = beerus
    //         .ethereum_lightclient
    //         .lock()
    //         .await
    //         .get_logs(
    //             &Some("finalized".to_string()),
    //             &Some("pending".to_string()),
    //             &None,
    //             &None,
    //             &None,
    //         )
    //         .await;
    //     // Then
    //     // Assert that the `get_logs` method of the Beerus light client returns `Ok`.
    //     assert!(result.is_ok());
    //     // Assert that the code returned by the `get_logs` method of the Beerus light client is the expected code.
    //     assert_eq!(result.unwrap(), vec![Log::default()]);
    // }

    // /// Test the `get_logs` method when the Ethereum light client returns an error.
    // /// This test mocks external dependencies.
    // /// It does not test the `get_logs` method of the external dependencies.
    // /// It tests the `get_logs` method of the Beerus light client.
    // #[tokio::test]
    // async fn given_ethereum_lightclient_returns_error_when_query_get_logs_then_error_is_propagated()
    // {
    //     // Given
    //     // Mock config, ethereum light client and starknet light client.
    //     let (config, mut ethereum_lightclient_mock, starknet_lightclient_mock) = mock_clients();
    //     let expected_error = concat!(
    //         "Non valid combination of from_block, to_block and blockhash. ",
    //         "If you want to filter blocks, then ",
    //         "you can only use either from_block and to_block or blockhash, not both",
    //     );
    //     // Mock dependencies.
    //     ethereum_lightclient_mock
    //         .expect_get_logs()
    //         .return_once(move |_, _, _, _, _| Err(eyre::eyre!(expected_error.clone())));
    //     // When
    //     let beerus = BeerusLightClient::new_from_clients(
    //         config.clone(),
    //         Box::new(ethereum_lightclient_mock),
    //         Box::new(starknet_lightclient_mock),
    //     );
    //     // Query the transaction data given a hash on Ethereum.
    //     let result = beerus
    //         .ethereum_lightclient
    //         .lock()
    //         .await
    //         .get_logs(&None, &None, &None, &None, &None)
    //         .await;
    //     // Then
    //     // Assert that the `get_logs` method of the Beerus light client returns `Err`.
    //     assert!(result.is_err());
    //     // Assert that the error returned by the `get_logs` method of the Beerus light client is the expected error.
    //     assert_eq!(result.unwrap_err().to_string(), expected_error.to_string());
    // }

    /// Test the `get_events` when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_events` method of the external dependencies.
    /// It tests the `get_events` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_events_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `get_events` method of the Starknet light client.
        let (expected_result, expected_result_value) =
            beerus_core::starknet_helper::create_mock_get_events();

        starknet_lightclient_mock
            .expect_get_events()
            .return_once(move |_, _, _| Ok(expected_result));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let filter = EventFilter {
            from_block: None,
            to_block: None,
            address: None,
            keys: None,
        };
        let continuation_token = Some("5".to_string());
        let chunk_size = 1;
        let result = beerus
            .starknet_lightclient
            .get_events(filter, continuation_token, chunk_size)
            .await
            .unwrap();

        // Then
        // Assert that the code returned by the `get_events` method of the Beerus light client is the expected code.
        assert_eq!(
            serde_json::value::to_value(result).unwrap(),
            expected_result_value
        )
    }

    /// Test the `get_events` when starknet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_events` method of the external dependencies.
    /// It tests the `get_events` method of the Beerus light client.
    #[tokio::test]
    async fn given_starknet_lightclient_error_when_call_get_events_then_should_return_error() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `get_events` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_get_events()
            .times(1)
            .return_once(move |_, _, _| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: STARKNET_LIGHT_CLIENT_ERROR.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let filter = EventFilter {
            from_block: None,
            to_block: None,
            address: None,
            keys: None,
        };
        let continuation_token = Some("5".to_string());
        let chunk_size = 1;
        let result = beerus
            .starknet_lightclient
            .get_events(filter, continuation_token, chunk_size)
            .await;

        // Then
        // Assert that the `get_events` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_events` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, STARKNET_LIGHT_CLIENT_ERROR.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
        // Assert that the sync status of the Beerus light client is `SyncStatus::NotSynced`.
        assert_eq!(beerus.sync_status().clone(), SyncStatus::NotSynced);
    }

    /// Test the `syncing` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `syncing` method of the external dependencies.
    /// It tests the `syncing` method of the Beerus light client.
    /// Case: node starknet is syncing.
    #[tokio::test]
    async fn given_normal_conditions_when_call_syncing_case_status_syncing_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `syncing` method of the Starknet light client.
        let (expected_result, _, expected_result_value) =
            beerus_core::starknet_helper::create_mock_syncing_case_syncing();

        starknet_lightclient_mock
            .expect_syncing()
            .return_once(move || Ok(expected_result));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus.starknet_lightclient.syncing().await.unwrap();

        // Then
        // Assert that the node starknet syncing returned by the `syncing` method of the Beerus light client
        // is the expected sync status type.
        assert_eq!(
            serde_json::value::to_value(result).unwrap(),
            expected_result_value
        )
    }

    /// Test the `syncing` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `syncing` method of the external dependencies.
    /// It tests the `syncing` method of the Beerus light client.
    /// Case: node starknet is not syncing.
    #[tokio::test]
    async fn given_normal_conditions_when_call_syncing_case_status_not_syncing_then_should_return_ok(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `syncing` method of the Starknet light client.
        let (expected_result, _) =
            beerus_core::starknet_helper::create_mock_syncing_case_not_syncing();
        let expected_result_value = SyncStatusType::NotSyncing;

        starknet_lightclient_mock
            .expect_syncing()
            .return_once(move || Ok(expected_result));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus.starknet_lightclient.syncing().await.unwrap();

        // Then
        // Assert that the node starknet syncing returned by the `syncing` method of the Beerus light client
        // is the expected sync status type.
        assert_eq!(
            serde_json::value::to_value(result).unwrap(),
            serde_json::value::to_value(expected_result_value).unwrap()
        )
    }

    /// Test the `syncing` method when the StarkNet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `syncing` method of the external dependencies.
    /// It tests the `syncing` method of the Beerus light client.
    /// It tests the error handling of the `syncing` method of the Beerus light client.
    #[tokio::test]
    async fn given_starknet_lightclient_error_when_call_syncing_then_should_return_error() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `syncing` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_syncing()
            .times(1)
            .return_once(move || {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: STARKNET_LIGHT_CLIENT_ERROR.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus.starknet_lightclient.syncing().await;

        // Then
        // Assert that the `get_class_at` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `syncing` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, STARKNET_LIGHT_CLIENT_ERROR.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
        // Assert that the sync status of the Beerus light client is `SyncStatus::NotSynced`.
        assert_eq!(beerus.sync_status().clone(), SyncStatus::NotSynced);
    }

    /// Test the `estimate_fee` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `estimate_fee` method of the external dependencies.
    /// It tests the `estimate_fee` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_estimate_fee_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `estimate_fee` method of the Starknet light client.
        let expected_result = FeeEstimate {
            gas_consumed: 5194,
            gas_price: 25886605195,
            overall_fee: 134455027382830,
        };
        let expected_result_string = serde_json::to_string(&expected_result).unwrap();

        starknet_lightclient_mock
            .expect_estimate_fee()
            .return_once(move |_, _| Ok(expected_result));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = block_id_string_to_block_id_type("tag", "latest").unwrap();
        let (tx, _) = create_mock_broadcasted_transaction();

        let result = beerus
            .starknet_lightclient
            .estimate_fee(tx, &block_id)
            .await
            .unwrap();

        // Then
        // Assert that the estimated fee returned by the `estimate_fee` method of the Beerus light client
        // is the expected estimated gas fee
        assert_eq!(
            serde_json::to_string(&result).unwrap(),
            expected_result_string,
        )
    }

    /// Test the `estimate_fee` method when the StarkNet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `estimate_fee` method of the external dependencies.
    /// It tests the `estimate_fee` method of the Beerus light client.
    /// It tests the error handling of the `estimate_fee` method of the Beerus light client.
    #[tokio::test]
    async fn given_starknet_lightclient_error_when_call_estimate_fee_then_should_return_error() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `estimate_fee` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_estimate_fee()
            .times(1)
            .return_once(move |_, _| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: STARKNET_LIGHT_CLIENT_ERROR.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );
        let block_id = block_id_string_to_block_id_type("tag", "latest").unwrap();
        let (tx, _) = create_mock_broadcasted_transaction();

        let result = beerus
            .starknet_lightclient
            .estimate_fee(tx, &block_id)
            .await;

        // Then
        // Assert that the `estimate_fee` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `estimate_fee` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
        assert_eq!(result_err.message, STARKNET_LIGHT_CLIENT_ERROR.to_string());
    }

    /// Test the `get_state_update` when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_state_update` method of the external dependencies.
    /// It tests the `get_state_update` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_query_get_state_update_then_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();
        let felt = FieldElement::from_hex_be("0x1").unwrap();
        let expected_result = StateUpdate {
            block_hash: felt.clone(),
            new_root: felt.clone(),
            old_root: felt.clone(),
            state_diff: StateDiff {
                deployed_contracts: vec![],
                storage_diffs: vec![],
                declared_contract_hashes: vec![],
                nonces: vec![],
            },
        };
        let expected = expected_result.clone();
        // Mock the `get_state_update` method of the Starknet light client.
        // Given
        // Mock dependencies
        starknet_lightclient_mock
            .expect_get_state_update()
            .return_once(move |_| Ok(expected));
        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );
        // Query the transaction data given a hash on Ethereum.
        let block_id = block_id_string_to_block_id_type("tag", "latest").unwrap();
        let result = beerus
            .starknet_lightclient
            .get_state_update(&block_id)
            .await;

        // Then
        // Assert that the `get_state_update` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the code returned by the `get_state_update` method of the Beerus light client is the expected code.

        // Note:
        // StateUpdate does not implement Eq, so I do the asserts this way.
        assert_eq!(
            result.as_ref().unwrap().block_hash,
            expected_result.block_hash
        );
        assert_eq!(result.as_ref().unwrap().new_root, expected_result.new_root);
        assert_eq!(result.as_ref().unwrap().old_root, expected_result.old_root);
    }

    /// Test the `get_state_update` when starknet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_state_update` method of the external dependencies.
    /// It tests the `get_state_update` method of the Beerus light client.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_get_state_update_then_error_is_propagated(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();
        let error_message = "error decoding response body: data did not match any variant of untagged enum JsonRpcResponse";

        // Mock the `get_state` method of the Ethereum light client.
        // Given
        // Mock dependencies
        starknet_lightclient_mock
            .expect_get_state_update()
            .return_once(move |_| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: error_message.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );
        let block_id = block_id_string_to_block_id_type("tag", "latest").unwrap();
        let result = beerus
            .starknet_lightclient
            .get_state_update(&block_id)
            .await;

        // Then
        // Assert that the `get_state_update` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_state_update` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, error_message.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
    }

    /// Test the `add_invoke_transaction` when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `add_invoke_transaction` method of the external dependencies.
    /// It tests the `add_invoke_transaction` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_query_add_invoke_transaction_then_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let expected_result = InvokeTransactionResult {
            transaction_hash: FieldElement::from_str("0x01").unwrap(),
        };
        let expected_result_value = expected_result.clone();
        // Mock the `add_invoke_transaction` method of the Ethereum light client.
        // Given
        // Mock dependencies
        starknet_lightclient_mock
            .expect_add_invoke_transaction()
            .return_once(move |_| Ok(expected_result));
        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let max_fee: FieldElement = FieldElement::from_str("0x01").unwrap();
        let signature: Vec<FieldElement> = vec![];
        let nonce: FieldElement = FieldElement::from_str("0x01").unwrap();
        let contract_address: FieldElement = FieldElement::from_str("0x01").unwrap();
        let entry_point_selector: FieldElement = FieldElement::from_str("0x01").unwrap();
        let calldata: Vec<FieldElement> = vec![];

        let transaction_data = BroadcastedInvokeTransactionV0 {
            max_fee,
            signature,
            nonce,
            contract_address,
            entry_point_selector,
            calldata,
        };

        let invoke_transaction = BroadcastedInvokeTransaction::V0(transaction_data);
        // Query the transaction data given a hash on Ethereum.
        let result = beerus
            .starknet_lightclient
            .add_invoke_transaction(&invoke_transaction)
            .await;

        // Then
        // Assert that the `add_invoke_transaction` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the code returned by the `add_invoke_transaction` method of the Beerus light client is the expected code.
        assert_eq!(
            format!("{result:?}"),
            format!("Ok({expected_result_value:?})")
        );
    }

    /// Test the `add_invoke_transaction` method when the Ethereum light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `add_invoke_transaction` method of the external dependencies.
    /// It tests the `add_invoke_transaction` method of the Beerus light client.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_add_invoke_transaction_then_error_is_propagated(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let error_message = concat!(
            "Non valid combination of from_block, to_block and blockhash. ",
            "If you want to filter blocks, then ",
            "you can only use either from_block and to_block or blockhash, not both",
        );

        // Mock dependencies.
        starknet_lightclient_mock
            .expect_add_invoke_transaction()
            .return_once(move |_| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: error_message.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let max_fee: FieldElement = FieldElement::from_str("0x01").unwrap();
        let signature: Vec<FieldElement> = vec![];
        let nonce: FieldElement = FieldElement::from_str("0x01").unwrap();
        let contract_address: FieldElement = FieldElement::from_str("0x01").unwrap();
        let entry_point_selector: FieldElement = FieldElement::from_str("0x01").unwrap();
        let calldata: Vec<FieldElement> = vec![];

        let transaction_data = BroadcastedInvokeTransactionV0 {
            max_fee,
            signature,
            nonce,
            contract_address,
            entry_point_selector,
            calldata,
        };

        let invoke_transaction = BroadcastedInvokeTransaction::V0(transaction_data);

        // Query the transaction data given a hash on Ethereum.
        let result = beerus
            .starknet_lightclient
            .add_invoke_transaction(&invoke_transaction)
            .await;

        // Then
        // Assert that the `add_invoke_transaction` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `add_invoke_transaction` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, error_message.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
    }

    /// Test the `add_deploy_transaction` when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `add_deploy_transaction` method of the external dependencies.
    /// It tests the `add_deploy_transaction` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_query_add_deploy_transaction_then_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let expected_result = DeployTransactionResult {
            transaction_hash: FieldElement::from_str("0x01").unwrap(),
            contract_address: FieldElement::from_str("0x01").unwrap(),
        };
        let expected_result_value = expected_result.clone();
        // Mock the `add_deploy_transaction` method of the Ethereum light client.
        // Given
        // Mock dependencies
        starknet_lightclient_mock
            .expect_add_deploy_transaction()
            .return_once(move |_| Ok(expected_result));
        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let program = vec![];
        let constructor = vec![LegacyContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];

        let external = vec![LegacyContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];

        let l1_handler = vec![LegacyContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];
        let entry_points_by_type = LegacyEntryPointsByType {
            constructor,
            external,
            l1_handler,
        };
        let abi = None;

        let contract_class = ContractClass::Legacy(LegacyContractClass {
            program,
            entry_points_by_type,
            abi,
        });

        let deploy_transaction = BroadcastedDeployTransaction {
            contract_class,
            version: 10,
            contract_address_salt: FieldElement::from_str("0").unwrap(),
            constructor_calldata: vec![],
        };
        // Query the transaction data given a hash on Ethereum.
        let result = beerus
            .starknet_lightclient
            .add_deploy_transaction(&deploy_transaction)
            .await;

        // Then
        // Assert that the `add_deploy_transaction` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the code returned by the `add_deploy_transaction` method of the Beerus light client is the expected code.
        assert_eq!(
            format!("{result:?}"),
            format!("Ok({expected_result_value:?})")
        );
    }

    /// Test the `add_deploy_transaction` method when the Ethereum light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `add_deploy_transaction` method of the external dependencies.
    /// It tests the `add_deploy_transaction` method of the Beerus light client.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_add_deploy_transaction_then_error_is_propagated(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let error_message = concat!(
            "Non valid combination of from_block, to_block and blockhash. ",
            "If you want to filter blocks, then ",
            "you can only use either from_block and to_block or blockhash, not both",
        );

        // Mock dependencies.
        starknet_lightclient_mock
            .expect_add_deploy_transaction()
            .return_once(move |_| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: error_message.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let program = vec![];
        let constructor = vec![LegacyContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];

        let external = vec![LegacyContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];

        let l1_handler = vec![LegacyContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];
        let entry_points_by_type = LegacyEntryPointsByType {
            constructor,
            external,
            l1_handler,
        };
        let abi = None;

        let contract_class = ContractClass::Legacy(LegacyContractClass {
            program,
            entry_points_by_type,
            abi,
        });

        let deploy_transaction = BroadcastedDeployTransaction {
            contract_class,
            version: 10,
            contract_address_salt: FieldElement::from_str("0").unwrap(),
            constructor_calldata: vec![],
        };

        // Query the transaction data given a hash on Ethereum.
        let result = beerus
            .starknet_lightclient
            .add_deploy_transaction(&deploy_transaction)
            .await;

        // Then
        // Assert that the `add_deploy_transaction` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `add_deploy_transaction` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, error_message.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
    }

    /// Test the `get_block_with_txs` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_block_with_txs` method of the external dependencies.
    /// It tests the `get_block_with_txs` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_block_with_txs_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let status = BlockStatus::Pending;
        let block_hash = FieldElement::from_dec_str("01").unwrap();
        let parent_hash = FieldElement::from_dec_str("01").unwrap();
        let block_number = 0;
        let new_root = FieldElement::from_dec_str("01").unwrap();
        let timestamp: u64 = 10;
        let sequencer_address = FieldElement::from_dec_str("01").unwrap();
        let transactions = vec![];
        let block = BlockWithTxs {
            status,
            block_hash,
            parent_hash,
            block_number,
            new_root,
            timestamp,
            sequencer_address,
            transactions,
        };
        // Mock the `get_block_with_txs` method of the Starknet light client.
        let expected_result = MaybePendingBlockWithTxs::Block(block);
        let expected_value_value = expected_result.clone();

        starknet_lightclient_mock
            .expect_get_block_with_txs()
            .return_once(move |_block_id| Ok(expected_result));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let result = beerus
            .starknet_lightclient
            .get_block_with_txs(&block_id)
            .await
            .unwrap();

        // Then
        // Assert that the block data returned by the `get_block_with_txs` method of the Beerus light client
        assert_eq!(format!("{result:?}"), format!("{expected_value_value:?}"))
    }

    /// Test the `get_block_with_txs` method when the StarkNet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_block_with_txs` method of the external dependencies.
    /// It tests the `get_block_with_txs` method of the Beerus light client.
    /// It tests the error handling of the `get_block_with_txs` method of the Beerus light client.
    #[tokio::test]
    async fn given_starknet_lightclient_error_when_call_get_block_with_txs_then_should_return_error(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `get_block_with_txs` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_get_block_with_txs()
            .times(1)
            .return_once(move |_block_id| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: STARKNET_LIGHT_CLIENT_ERROR.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let result = beerus
            .starknet_lightclient
            .get_block_with_txs(&block_id)
            .await;

        // Then
        // Assert that the `get_block_with_txs` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_block_with_txs` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, STARKNET_LIGHT_CLIENT_ERROR.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
        // Assert that the sync status of the Beerus light client is `SyncStatus::NotSynced`.
        assert_eq!(beerus.sync_status().clone(), SyncStatus::NotSynced);
    }

    /// Test the `get_transaction_by_block_id_and_index` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_transaction_by_block_id_and_index` method of the external dependencies.
    /// It tests the `get_transaction_by_block_id_and_index` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_transaction_by_block_id_and_index_then_should_return_ok(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `get_transaction_by_block_id_and_index` method of the Starknet light client.
        let transaction_hash = FieldElement::from_str("0x01").unwrap();
        let max_fee = FieldElement::from_str("0x01").unwrap();
        let signature = vec![];
        let nonce = FieldElement::from_str("0x01").unwrap();
        let contract_address = FieldElement::from_str("0x01").unwrap();
        let entry_point_selector = FieldElement::from_str("0x01").unwrap();
        let calldata = vec![];

        let invoke_transaction = InvokeTransactionV0 {
            transaction_hash,
            max_fee,
            signature,
            nonce,
            contract_address,
            entry_point_selector,
            calldata,
        };

        let expected_result =
            StarknetTransaction::Invoke(InvokeTransaction::V0(invoke_transaction));
        let expected_result_value = expected_result.clone();

        starknet_lightclient_mock
            .expect_get_transaction_by_block_id_and_index()
            .return_once(move |_block_id, _index| Ok(expected_result));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let index: u64 = 0;
        let result = beerus
            .starknet_lightclient
            .get_transaction_by_block_id_and_index(&block_id, index)
            .await
            .unwrap();

        // Then
        // Assert that the number of transactions in a block returned by the `get_transaction_by_block_id_and_index` method of the Beerus light client is the expected number of transactions in a block.
        assert_eq!(format!("{result:?}"), format!("{expected_result_value:?}"));
    }

    /// Test the `get_transaction_by_block_id_and_index` method when the StarkNet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_transaction_by_block_id_and_index` method of the external dependencies.
    /// It tests the `get_transaction_by_block_id_and_index` method of the Beerus light client.
    /// It tests the error handling of the `get_transaction_by_block_id_and_index` method of the Beerus light client.
    #[tokio::test]
    async fn given_starknet_lightclient_error_when_call_get_transaction_by_block_id_and_index_then_should_return_error(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `get_transaction_by_block_id_and_index` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_get_transaction_by_block_id_and_index()
            .times(1)
            .return_once(move |_block_id, _index| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: STARKNET_LIGHT_CLIENT_ERROR.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let index: u64 = 0;
        let result = beerus
            .starknet_lightclient
            .get_transaction_by_block_id_and_index(&block_id, index)
            .await;

        // Then
        // Assert that the `get_transaction_by_block_id_and_index` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_transaction_by_block_id_and_index` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, STARKNET_LIGHT_CLIENT_ERROR.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
        // Assert that the sync status of the Beerus light client is `SyncStatus::NotSynced`.
        assert_eq!(beerus.sync_status().clone(), SyncStatus::NotSynced);
    }

    /// Test the `pending_transactions` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `pending_transactions` method of the external dependencies.
    /// It tests the `pending_transactions` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_pending_transactions_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `pending_transactions` method of the Starknet light client.
        let expected_result = vec![];
        let expected_result_value = expected_result.clone();

        starknet_lightclient_mock
            .expect_pending_transactions()
            .return_once(move || Ok(expected_result));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );
        let result = beerus
            .starknet_lightclient
            .pending_transactions()
            .await
            .unwrap();

        // Then
        // Assert that the number of transactions in a block returned by the `pending_transactions` method of the Beerus light client is the expected number of transactions in a block.
        assert_eq!(format!("{result:?}"), format!("{expected_result_value:?}"));
    }

    /// Test the `pending_transactions` method when the StarkNet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `pending_transactions` method of the external dependencies.
    /// It tests the `pending_transactions` method of the Beerus light client.
    /// It tests the error handling of the `pending_transactions` method of the Beerus light client.
    #[tokio::test]
    async fn given_starknet_lightclient_error_when_call_pending_transactions_then_should_return_error(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `pending_transactions` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_pending_transactions()
            .times(1)
            .return_once(move || {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: STARKNET_LIGHT_CLIENT_ERROR.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus.starknet_lightclient.pending_transactions().await;

        // Then
        // Assert that the `pending_transactions` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `pending_transactions` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, STARKNET_LIGHT_CLIENT_ERROR.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
        // Assert that the sync status of the Beerus light client is `SyncStatus::NotSynced`.
        assert_eq!(beerus.sync_status().clone(), SyncStatus::NotSynced);
    }

    /// Test the `get_transaction_receipt` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_transaction_receipt` method of the external dependencies.
    /// It tests the `get_transaction_receipt` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_transaction_receipt_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();
        let felt = FieldElement::from_str("0x1").unwrap();
        // Mock the `get_transaction_receipt` method of the Starknet light client.
        let transaction_receipt = InvokeTransactionReceipt {
            transaction_hash: felt.clone(),
            actual_fee: felt.clone(),
            status: TransactionStatus::AcceptedOnL2,
            block_hash: felt.clone(),
            block_number: 0xFFF_u64,
            messages_sent: vec![],
            events: vec![],
        };
        let expected_result = MaybePendingTransactionReceipt::Receipt(TransactionReceipt::Invoke(
            transaction_receipt,
        ));
        let closure_return = expected_result.clone();
        starknet_lightclient_mock
            .expect_get_transaction_receipt()
            .return_once(move |_| Ok(closure_return));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );
        let result = beerus
            .starknet_lightclient
            .get_transaction_receipt(felt.clone())
            .await
            .unwrap();

        // Then
        // Assert that the number of transactions in a block returned by the `get_transaction_receipt` method of the Beerus light client is the expected number of transactions in a block.
        assert_eq!(format!("{result:?}"), format!("{expected_result:?}"));
    }
    /// Test the `get_transaction_receipt` method when the StarkNet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_transaction_receipt` method of the external dependencies.
    /// It tests the `get_transaction_receipt` method of the Beerus light client.
    /// It tests the error handling of the `get_transaction_receipt` method of the Beerus light client.
    #[tokio::test]
    async fn given_starknet_lightclient_error_when_call_get_transaction_receipt_then_should_return_error(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // Mock the `get_transaction_receipt` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_get_transaction_receipt()
            .times(1)
            .return_once(move |_| {
                Err(JsonRpcError {
                    code: TRANSACTION_HASH_NOT_FOUND_CODE,
                    message: TRANSACTION_HASH_NOT_FOUND.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus
            .starknet_lightclient
            .get_transaction_receipt(FieldElement::from_str("0x1").unwrap())
            .await;

        // Then
        // Assert that the `get_transaction_receipt` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // Assert that the error returned by the `get_transaction_receipt` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, TRANSACTION_HASH_NOT_FOUND.to_string());
        assert_eq!(result_err.code, TRANSACTION_HASH_NOT_FOUND_CODE);

        // Assert that the sync status of the Beerus light client is `SyncStatus::NotSynced`.
        assert_eq!(beerus.sync_status().clone(), SyncStatus::NotSynced);
    }

    /// Test the `get_block_with_tx_hashes` method when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `get_block_with_tx_hashes` method of the external dependencies.
    /// It tests the `get_block_with_tx_hashes` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_block_with_tx_hashes_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let status = BlockStatus::Pending;
        let block_hash = FieldElement::from_dec_str("01").unwrap();
        let parent_hash = FieldElement::from_dec_str("01").unwrap();
        let block_number = 0;
        let new_root = FieldElement::from_dec_str("01").unwrap();
        let timestamp: u64 = 10;
        let sequencer_address = FieldElement::from_dec_str("01").unwrap();
        let transactions = vec![];
        let block = BlockWithTxHashes {
            status,
            block_hash,
            parent_hash,
            block_number,
            new_root,
            timestamp,
            sequencer_address,
            transactions,
        };
        // Mock the `get_block_with_tx_hashes` method of the Starknet light client.
        let expected_result = MaybePendingBlockWithTxHashes::Block(block);
        let expected_value_value = expected_result.clone();

        starknet_lightclient_mock
            .expect_get_block_with_tx_hashes()
            .return_once(move |_block_id| Ok(expected_result));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let result = beerus
            .starknet_lightclient
            .get_block_with_tx_hashes(&block_id)
            .await
            .unwrap();

        // Then
        // Assert that the block data returned by the `get_block_with_tx_hashes` method of the Beerus light client
        assert_eq!(format!("{result:?}"), format!("{expected_value_value:?}"))
    }

    /// Test the `get_block_with_tx_hashes` method when the StarkNet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_block_with_tx_hashes` method of the external dependencies.
    /// It tests the `get_block_with_tx_hashes` method of the Beerus light client.
    /// It tests the error handling of the `get_block_with_tx_hashes` method of the Beerus light client.
    #[tokio::test]
    async fn given_starknet_lightclient_error_when_call_get_block_with_tx_hashes_then_should_return_error(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let expected_code = UNKNOWN_ERROR_CODE;
        let expected_message = "StarkNet light client error";

        // Mock the `get_block_with_tx_hashes` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_get_block_with_tx_hashes()
            .times(1)
            .return_once(move |_block_id| {
                Err(JsonRpcError {
                    code: expected_code,
                    message: expected_message.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let block_id = BlockId::Hash(FieldElement::from_str("0x01").unwrap());
        let result = beerus
            .starknet_lightclient
            .get_block_with_tx_hashes(&block_id)
            .await;

        // Then
        // Assert that the `get_block_with_tx_hashes` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        let unwraped_err = result.unwrap_err();
        // Assert that the error returned by the `get_block_with_tx_hashes` method of the Beerus light client is the expected error.
        assert_eq!(unwraped_err.message, expected_message);
        assert_eq!(unwraped_err.code, expected_code);
        // Assert that the sync status of the Beerus light client is `SyncStatus::NotSynced`.
        assert_eq!(beerus.sync_status().clone(), SyncStatus::NotSynced);
    }
    /// Test the `get_transaction_by_hash` method when the StarkNet light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `get_transaction_by_hash` method of the external dependencies.
    /// It tests the `get_transaction_by_hash` method of the Beerus light client.
    /// It tests the error handling of the `get_transaction_by_hash` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_call_get_transaction_by_hash_then_should_return_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let transaction_hash = FieldElement::from_str("0x01").unwrap();
        let max_fee = FieldElement::from_str("0x01").unwrap();
        let signature = vec![];
        let nonce = FieldElement::from_str("0x01").unwrap();
        let contract_address = FieldElement::from_str("0x01").unwrap();
        let entry_point_selector = FieldElement::from_str("0x01").unwrap();
        let calldata = vec![];

        let invoke_transaction = InvokeTransactionV0 {
            transaction_hash: transaction_hash.clone(),
            max_fee,
            signature,
            nonce,
            contract_address,
            entry_point_selector,
            calldata,
        };

        let expected_result =
            StarknetTransaction::Invoke(InvokeTransaction::V0(invoke_transaction));
        let expected_result_value = serde_json::to_value(expected_result.clone())
            .unwrap()
            .to_string();

        // Mock the `get_transaction_by_hash` method of the StarkNet light client.
        starknet_lightclient_mock
            .expect_get_transaction_by_hash()
            .times(1)
            .return_once(move |_| Ok(expected_result));

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let result = beerus
            .starknet_lightclient
            .get_transaction_by_hash(FieldElement::from_str(&"0x01".to_string()).unwrap())
            .await;

        // Then
        // Assert that the `get_transaction_by_hash` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        let result = serde_json::to_value(result.unwrap()).unwrap().to_string();
        // Assert that the value returned by the `get_transaction_by_hash` method of the Beerus light client is the expected value.
        assert_eq!(result, expected_result_value);
    }

    /// Test the `add_declare_transaction` when everything is fine.
    /// This test mocks external dependencies.
    /// It does not test the `add_declare_transaction` method of the external dependencies.
    /// It tests the `add_declare_transaction` method of the Beerus light client.
    #[tokio::test]
    async fn given_normal_conditions_when_query_add_declare_transaction_then_ok() {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let expected_result = DeclareTransactionResult {
            transaction_hash: FieldElement::from_str("0x01").unwrap(),
            class_hash: FieldElement::from_str("0x01").unwrap(),
        };
        let expected_result_value = expected_result.clone();
        // Mock the `add_declare_transaction` method of the Ethereum light client.
        // Given
        // Mock dependencies
        starknet_lightclient_mock
            .expect_add_declare_transaction()
            .return_once(move |_| Ok(expected_result));
        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let program = vec![];
        let constructor = vec![LegacyContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];

        let external = vec![LegacyContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];

        let l1_handler = vec![LegacyContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];
        let entry_points_by_type = LegacyEntryPointsByType {
            constructor,
            external,
            l1_handler,
        };
        let abi = None;

        let contract_class: LegacyContractClass = LegacyContractClass {
            program,
            entry_points_by_type,
            abi,
        };

        let declare_transaction =
            BroadcastedDeclareTransaction::V1(BroadcastedDeclareTransactionV1 {
                max_fee: FieldElement::from_str("1000").unwrap(),
                signature: vec![],
                nonce: FieldElement::from_str("0").unwrap(),
                contract_class,
                sender_address: FieldElement::from_str("101010").unwrap(),
            });
        // Query the transaction data given a hash on Ethereum.
        let result = beerus
            .starknet_lightclient
            .add_declare_transaction(&declare_transaction)
            .await;

        // Then
        // Assert that the `add_declare_transaction` method of the Beerus light client returns `Ok`.
        assert!(result.is_ok());
        // Assert that the code returned by the `add_declare_transaction` method of the Beerus light client is the expected code.
        assert_eq!(
            format!("{result:?}"),
            format!("Ok({expected_result_value:?})")
        );
    }

    /// Test the `add_declare_transaction` method when the Ethereum light client returns an error.
    /// This test mocks external dependencies.
    /// It does not test the `add_declare_transaction` method of the external dependencies.
    /// It tests the `add_declare_transaction` method of the Beerus light client.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_add_declare_transaction_then_error_is_propagated(
    ) {
        // Given
        // Mock config, ethereum light client and starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        let expected_message = concat!(
            "Non valid combination of from_block, to_block and blockhash. ",
            "If you want to filter blocks, then ",
            "you can only use either from_block and to_block or blockhash, not both",
        );

        // Mock dependencies.
        starknet_lightclient_mock
            .expect_add_declare_transaction()
            .return_once(move |_| {
                Err(JsonRpcError {
                    code: UNKNOWN_ERROR_CODE,
                    message: expected_message.to_string(),
                })
            });

        // When
        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        let program = vec![];
        let constructor = vec![LegacyContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];

        let external = vec![LegacyContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];

        let l1_handler = vec![LegacyContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];
        let entry_points_by_type = LegacyEntryPointsByType {
            constructor,
            external,
            l1_handler,
        };
        let abi = None;

        let contract_class: LegacyContractClass = LegacyContractClass {
            program,
            entry_points_by_type,
            abi,
        };

        let declare_transaction =
            BroadcastedDeclareTransaction::V1(BroadcastedDeclareTransactionV1 {
                max_fee: FieldElement::from_str("1000").unwrap(),
                signature: vec![],
                nonce: FieldElement::from_str("0").unwrap(),
                contract_class,
                sender_address: FieldElement::from_str("101010").unwrap(),
            });

        // Query the transaction data given a hash on Ethereum.
        let result = beerus
            .starknet_lightclient
            .add_declare_transaction(&declare_transaction)
            .await;

        // Then
        // Assert that the `add_declare_transaction` method of the Beerus light client returns `Err`.
        assert!(result.is_err());

        // Assert that the error returned by the `add_declare_transaction` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, expected_message);
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
    }

    #[tokio::test]
    async fn given_error_result_when_calling_starknet_pending_transactions_then_should_return_same_error(
    ) {
        // Given
        // Mock config and beerus light client with a mocked starknet light client.
        let (config, ethereum_lightclient_mock, mut starknet_lightclient_mock) = mock_clients();

        // The expected error is what is returned from the API Error
        let expected_error = JsonRpcError {
            code: UNKNOWN_ERROR_CODE,
            message: NETWORK_FAILURE.to_string(),
        };

        // Mock dependencies.
        starknet_lightclient_mock
            .expect_pending_transactions()
            .return_once(move || Err(expected_error)); // Return a network error

        let beerus = BeerusLightClient::new_from_clients(
            config.clone(),
            Box::new(ethereum_lightclient_mock),
            Box::new(starknet_lightclient_mock),
        );

        // When
        let result = beerus.starknet_pending_transactions().await;

        // Then
        // Assert that the `starknet_pending_transactions` method of the Beerus light client returns `Err`.
        assert!(result.is_err());
        // let actual_error = result.unwrap_err().to_string();
        // println!("expected error: {}", expected_error);
        // println!("actual error: {}", actual_error);
        // assert_eq!(actual_error, expected_error);

        // Assert that the error returned by the `starknet_pending_transactions` method of the Beerus light client is the expected error.
        let result_err = result.unwrap_err();
        assert_eq!(result_err.message, NETWORK_FAILURE.to_string());
        assert_eq!(result_err.code, UNKNOWN_ERROR_CODE);
    }
}
