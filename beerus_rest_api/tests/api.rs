#[cfg(test)]
mod test {
    use beerus_core::{
        config::Config,
        lightclient::{
            beerus::BeerusLightClient,
            ethereum::MockEthereumLightClient,
            starknet::{storage_proof::GetProofOutput, MockStarkNetLightClient},
        },
    };
    use beerus_rest_api::build_rocket_server;
    use ethers::prelude::Log;
    use ethers::types::{Address, Transaction};
    use ethers::types::{H256, U256};
    use helios::types::{ExecutionBlock, Transactions};
    use rocket::{http::Status, local::asynchronous::Client, uri};
    use starknet::{
        core::types::FieldElement,
        providers::jsonrpc::models::{
            BlockHashAndNumber, BlockStatus, BlockWithTxHashes, BlockWithTxs,
            DeployTransactionResult, InvokeTransaction, InvokeTransactionReceipt,
            InvokeTransactionResult, InvokeTransactionV0, MaybePendingBlockWithTxHashes,
            MaybePendingBlockWithTxs, MaybePendingTransactionReceipt, StateDiff, StateUpdate,
            Transaction as StarknetTransaction, TransactionReceipt, TransactionStatus,
        },
    };
    use std::path::PathBuf;
    use std::str::FromStr;

    /// Test the `send_raw_transaction` endpoint.
    /// `/ethereum/send_raw_transaction/<bytes>`
    /// Given normal conditions, when sending raw transaction, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_send_raw_transaction_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();
        let expected_value =
            H256::from_str("0xc9bb964b3fe087354bc1c1904518acc2b9df7ebedcb89215e9f3b41f47b6c31d")
                .unwrap();
        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_send_raw_transaction()
            .return_once(move |_| Ok(expected_value));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/ethereum/send_raw_transaction/0xc24215226336d22238a20a72f8e489c005b44c4a"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"response\":\"0xc9bb964b3fe087354bc1c1904518acc2b9df7ebedcb89215e9f3b41f47b6c31d\"}"
        );
    }

    /// Test the `send_raw_transaction` endpoint.
    /// `/ethereum/send_raw_transaction/<bytes>`
    /// Given Ethereum light client returns error when sending raw transaction, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_send_raw_transaction_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_send_raw_transaction()
            .return_once(move |_| Err(eyre::eyre!("cannot send raw transaction")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/ethereum/send_raw_transaction/0xc24215226336d22238a20a72f8e489c005b44c4a"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot send raw transaction\"}"
        );
    }

    /// Test the `query_balance` endpoint.
    /// `/ethereum/balance/<address>`
    /// Given normal conditions, when query balance, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_balance_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_balance()
            .return_once(move |_, _| Ok(123.into()));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/ethereum/balance/0xc24215226336d22238a20a72f8e489c005b44c4a"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), "{\"address\":\"0xc24215226336d22238a20a72f8e489c005b44c4a\",\"balance\":\"0.000000000000000123\",\"unit\":\"ETH\"}");
    }

    /// Test the `query_balance` endpoint.
    /// `/ethereum/balance/<address>`
    /// Given Ethereum light client returns error when query balance, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_balance_then_error_is_propagated()
    {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_balance()
            .return_once(move |_, _| Err(eyre::eyre!("cannot query balance")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/ethereum/balance/0xc24215226336d22238a20a72f8e489c005b44c4a"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query balance\"}"
        );
    }

    /// Test the `nonce_balance` endpoint.
    /// `/ethereum/nonce/<address>`
    /// Given normal conditions, when query nonce, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_nonce_then_ok() {
        //Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_nonce()
            .return_once(move |_, _| Ok(123));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("Valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/ethereum/nonce/0xc24215226336d22238a20a72f8e489c005b44c4a"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"address\":\"0xc24215226336d22238a20a72f8e489c005b44c4a\",\"nonce\":123}"
        )
    }

    /// Test the `nonce_balance` endpoint.
    /// `/ethereum/nonce/<address>`
    /// Given Ethereum light client returns error when query nonce, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_nonce_then_error_is_propagates() {
        // Build mocks
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_nonce()
            .return_once(move |_, _| Err(eyre::eyre!("cannot query nonce")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/ethereum/nonce/0xc24215226336d22238a20a72f8e489c005b44c4a"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query nonce\"}"
        );
    }

    /// Test the `query_block_number` endpoint.
    /// `/ethereum/block_number`
    /// Given normal conditions, when query block number, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_block_number_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_block_number()
            .return_once(move || Ok(123));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/ethereum/block_number")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"block_number\":123}"
        );
    }

    /// Test the `query_block_number` endpoint.
    /// `/ethereum/block_number`
    /// Given Ethereum light client returns error when query block number, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_block_number_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_block_number()
            .return_once(move || Err(eyre::eyre!("cannot query block number")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/ethereum/block_number")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query block number\"}"
        );
    }

    /// Test the `chain_id` endpoint.
    /// `/ethereum/chain_id`
    /// Given normal conditions, when query chain id, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_chain_id_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_chain_id()
            .return_once(move || 1);

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/ethereum/chain_id")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), "{\"chain_id\":1}");
    }

    /// Test the `query_ethereum_block_by_number` endpoint.
    /// `ethereum/get_block_by_number/<block_number>/<full_txs>`
    /// Given normal conditions, when query block by number, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_block_by_number_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        let expected_block = ExecutionBlock {
            number: 0,
            base_fee_per_gas: Default::default(),
            difficulty: Default::default(),
            extra_data: vec![],
            gas_limit: 0,
            gas_used: 0,
            hash: Default::default(),
            logs_bloom: vec![],
            miner: Default::default(),
            mix_hash: Default::default(),
            nonce: "".to_string(),
            parent_hash: Default::default(),
            receipts_root: Default::default(),
            sha3_uncles: Default::default(),
            size: 0,
            state_root: Default::default(),
            timestamp: 0,
            total_difficulty: 0,
            transactions: Transactions::Full(vec![]),
            transactions_root: Default::default(),
            uncles: vec![],
        };
        let block_string = serde_json::to_string(&expected_block).unwrap();
        let expected_block_value: serde_json::Value =
            serde_json::from_str(block_string.as_str()).unwrap();
        let expected_response = format!("{{\"block\":{expected_block_value}}}");
        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_block_by_number()
            .return_once(move |_, _| Ok(Some(expected_block)));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!("/ethereum/get_block_by_number/1/true"))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), expected_response);
    }

    /// Test the `query_starknet_state_root` endpoint.
    /// `/starknet/state/root`
    /// Given normal conditions, when query starknet state root, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_state_root_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Expected state root.
        let expected_starknet_state_root =
            U256::from_str("0x5bb9692622e817c39663e69dce50777daf4c167bdfa95f3e5cef99c6b8a344d")
                .unwrap();
        // Convert to bytes because that's what the mock returns.
        let mut expected_starknet_state_root_bytes: Vec<u8> = vec![0; 32];
        expected_starknet_state_root.to_big_endian(&mut expected_starknet_state_root_bytes);
        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_call()
            .return_once(move |_, _| Ok(expected_starknet_state_root_bytes));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/state/root")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), "{\"state_root\":\"2593003852473857760763774375943570015682902311385614557145528717605591462989\"}");
    }

    /// Test the `get_code` endpoint.
    /// `/ethereum/code`
    /// Given normal conditions, when query code, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_code_then_ok() {
        // Build mocks
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        let check_value = vec![0, 0, 0, 1];
        // Given
        // Mock dependencies
        ethereum_lightclient
            .expect_get_code()
            .return_once(move |_, _| Ok(check_value));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        let response = client
            .get(uri!(
                "/ethereum/code/0xc24215226336d22238a20a72f8e489c005b44c4a"
            ))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"code\":[0,0,0,1]}"
        )
    }

    /// Test the `query_code` endpoint.
    /// `/ethereum/code`
    /// Given Ethereum light client returns error when query code, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_code_then_error_is_propagated() {
        // Build mocks
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies
        ethereum_lightclient
            .expect_get_code()
            .return_once(move |_, _| Err(eyre::eyre!("Cannot query code")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        let response = client
            .get(uri!(
                "/ethereum/code/0xc24215226336d22238a20a72f8e489c005b44c4a"
            ))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"Cannot query code\"}"
        )
    }

    /// Test the `query_tx_count` endpoint.
    /// `/ethereum/query_tx_count/<address>/<block>`
    /// Given normal conditions, when `query_tx_count`, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_tx_count_then_ok() {
        // Build mocks
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        let expected_result: u64 = 120;
        // Given
        // Mock dependencies
        ethereum_lightclient
            .expect_get_transaction_count()
            .return_once(move |_, _| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        let response = client
            .get(uri!(
                "/ethereum/tx_count/0xc24215226336d22238a20a72f8e489c005b44c4a/latest"
            ))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), "{\"tx_count\":120}")
    }

    /// Test the `query_tx_count` endpoint.
    /// `/ethereum/query_tx_count/<address>/<block>`
    /// Given Ethereum light client returns error when `query_tx_count`, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_tx_count_then_error_is_propagated()
    {
        // Build mocks
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies
        ethereum_lightclient
            .expect_get_transaction_count()
            .return_once(move |_, _| Err(eyre::eyre!("Cannot query tx count")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        let response = client
            .get(uri!(
                "/ethereum/tx_count/0xc24215226336d22238a20a72f8e489c005b44c4a/latest"
            ))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"Cannot query tx count\"}"
        )
    }

    /// Test the `query_block_transaction_count_by_number` endpoint.
    /// `/ethereum/tx_count_by_block_number`
    /// Given normal conditions, when `query_block_transaction_count_by_number`, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_tx_count_by_block_number_then_ok() {
        // Build mocks
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        let check_value: u64 = 120;
        // Given
        // Mock dependencies
        ethereum_lightclient
            .expect_get_block_transaction_count_by_number()
            .return_once(move |_| Ok(check_value));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        let response = client
            .get(uri!("/ethereum/tx_count_by_block_number/1"))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), "{\"tx_count\":120}")
    }

    /// Test the `query_block_transaction_count_by_number` endpoint.
    /// `/ethereum/tx_count_by_block_number/1`
    /// Given Ethereum light client returns error when `query_block_transaction_count_by_number`, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_tx_count_by_block_number_then_error_is_propagated(
    ) {
        // Build mocks
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies
        ethereum_lightclient
            .expect_get_block_transaction_count_by_number()
            .return_once(move |_| Err(eyre::eyre!("Cannot query block tx count")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        let response = client
            .get(uri!("/ethereum/tx_count_by_block_number/1"))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"Cannot query block tx count\"}"
        )
    }

    /// Test the `query_block_transaction_count_by_hash` endpoint.
    /// `/ethereum/tx_count_by_block_hash/0xc24215226336d22238a20a72f8e489c005b44c4a`
    /// Given normal conditions, when `query_block_transaction_count_by_hash`, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_tx_count_by_block_hash_then_ok() {
        // Build mocks
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        let check_value: u64 = 120;
        // Given
        // Mock dependencies
        ethereum_lightclient
            .expect_get_block_transaction_count_by_hash()
            .return_once(move |_| Ok(check_value));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        let response = client
            .get(uri!(
                "/ethereum/tx_count_by_block_hash/0xc24215226336d22238a20a72f8e489c005b44c4a"
            ))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), "{\"tx_count\":120}")
    }

    /// Test the `query_block_transaction_count_by_hash` endpoint.
    /// `/ethereum/tx_count_by_block_hash/0xc24215226336d22238a20a72f8e489c005b44c4a`
    /// Given Ethereum light client returns error when `query_block_transaction_count_by_hash`, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_tx_count_by_block_hash_then_error_is_propagated(
    ) {
        // Build mocks
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies
        ethereum_lightclient
            .expect_get_block_transaction_count_by_hash()
            .return_once(move |_| Err(eyre::eyre!("Cannot query block tx count")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        let response = client
            .get(uri!(
                "/ethereum/tx_count_by_block_hash/0xc24215226336d22238a20a72f8e489c005b44c4a"
            ))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"Cannot query block tx count\"}"
        )
    }

    /// Test the `query_transaction_by_hash` endpoint.
    /// `/ethereum/query_transaction_by_hash`
    /// Given normal conditions, when `query_transaction_by_hash`, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_transaction_by_hash_then_ok() {
        // Build mocks
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        let transaction = Transaction::default();

        // Given
        // Mock dependencies
        ethereum_lightclient
            .expect_get_transaction_by_hash()
            .return_once(move |_| Ok(Some(transaction)));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        let response = client
            .get(uri!("/ethereum/tx_by_hash/0xc9bb964b3fe087354bc1c1904518acc2b9df7ebedcb89215e9f3b41f47b6c31d"))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), "{\"tx_data\":\"Some(Transaction { hash: 0x0000000000000000000000000000000000000000000000000000000000000000, nonce: 0, block_hash: None, block_number: None, transaction_index: None, from: 0x0000000000000000000000000000000000000000, to: None, value: 0, gas_price: None, gas: 0, input: Bytes(0x), v: 0, r: 0, s: 0, transaction_type: None, access_list: None, max_priority_fee_per_gas: None, max_fee_per_gas: None, chain_id: None, other: OtherFields { inner: {} } })\"}")
    }

    /// Test the `query_transaction_by_hash` endpoint.
    /// `/ethereum/query_transaction_by_hash/1`
    /// Given Ethereum light client returns error when `query_transaction_by_hash`, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_transaction_by_hash_then_error_is_propagated(
    ) {
        // Build mocks
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies
        ethereum_lightclient
            .expect_get_transaction_by_hash()
            .return_once(move |_| Err(eyre::eyre!("Cannot query tx data")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        let response = client
            .get(uri!("/ethereum/tx_by_hash/0xc9bb964b3fe087354bc1c1904518acc2b9df7ebedcb89215e9f3b41f47b6c31d"))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"Cannot query tx data\"}"
        )
    }

    /// Test the `query_gas_price` endpoint.
    /// `/ethereum/gas_price`
    /// Given normal conditions, when query block number, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_gas_price_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_gas_price()
            .return_once(move || Ok(U256::default()));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/ethereum/gas_price")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"gas_price\":\"0\"}"
        );
    }

    /// Test the `query_gas_price` endpoint.
    /// `/ethereum/block_number`
    /// Given Ethereum light client returns error when query gas price, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_gas_price_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_gas_price()
            .return_once(move || Err(eyre::eyre!("cannot query block number")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/ethereum/gas_price")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query block number\"}"
        );
    }

    /// Test the `query_estimate_gas` endpoint.
    /// `/ethereum/estimate_gas`
    /// Given normal conditions, when query estimate gas, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_estimate_gas_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_estimate_gas()
            .return_once(move |_| Ok(10));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .post(uri!("/ethereum/estimate_gas"))
            .body(r#"{"from":"0x0000000000000000000000000000000000000000","to":"0x0000000000000000000000000000000000000000","value":"10","data":"0x41"}"#)
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), "{\"quantity\":10}");
    }

    /// Test the `query_estimate_gas` endpoint.
    /// `/ethereum/estimate_gas`
    /// Given Ethereum light client returns error when query estimate gas, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_estimate_gas_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_estimate_gas()
            .return_once(move |_| Err(eyre::eyre!("cannot query estimate gas")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .post(uri!("/ethereum/estimate_gas"))
            .body(r#"{"from":"0x0000000000000000000000000000000000000000","to":"0x0000000000000000000000000000000000000000","value":"10","data":"0x41"}"#)
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query estimate gas\"}"
        );
    }

    /// Test the `query_ethereum_block_by_hash` endpoint.
    /// `ethereum/get_block_by_hash<block_hash>/<full_txs>`
    /// Given normal conditions, when query block by hash, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_block_by_hash_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        let expected_block = ExecutionBlock {
            number: 0,
            base_fee_per_gas: Default::default(),
            difficulty: Default::default(),
            extra_data: vec![],
            gas_limit: 0,
            gas_used: 0,
            hash: Default::default(),
            logs_bloom: vec![],
            miner: Default::default(),
            mix_hash: Default::default(),
            nonce: "".to_string(),
            parent_hash: Default::default(),
            receipts_root: Default::default(),
            sha3_uncles: Default::default(),
            size: 0,
            state_root: Default::default(),
            timestamp: 0,
            total_difficulty: 0,
            transactions: Transactions::Full(vec![]),
            transactions_root: Default::default(),
            uncles: vec![],
        };
        let block_string = serde_json::to_string(&expected_block).unwrap();
        let expected_block_value: serde_json::Value =
            serde_json::from_str(block_string.as_str()).unwrap();
        let expected_response = format!("{{\"block\":{expected_block_value}}}");
        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_block_by_hash()
            .return_once(move |_, _| Ok(Some(expected_block)));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/ethereum/get_block_by_hash/0xc24215226336d22238a20a72f8e489c005b44c4a/true"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), expected_response);
    }

    /// Test the `query_block_by_hash` endpoint.
    /// `ethereum/get_block_by_hash<block_hash>/<full_txs>`
    /// Given Ethereum light client returns error when query block by hash, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_block_by_hash_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        let expected_block = ExecutionBlock {
            number: 0,
            base_fee_per_gas: Default::default(),
            difficulty: Default::default(),
            extra_data: vec![],
            gas_limit: 0,
            gas_used: 0,
            hash: Default::default(),
            logs_bloom: vec![],
            miner: Default::default(),
            mix_hash: Default::default(),
            nonce: "".to_string(),
            parent_hash: Default::default(),
            receipts_root: Default::default(),
            sha3_uncles: Default::default(),
            size: 0,
            state_root: Default::default(),
            timestamp: 0,
            total_difficulty: 0,
            transactions: Transactions::Full(vec![]),
            transactions_root: Default::default(),
            uncles: vec![],
        };
        let block_string = serde_json::to_string(&expected_block).unwrap();
        let expected_block_value: serde_json::Value =
            serde_json::from_str(block_string.as_str()).unwrap();
        let _expected_response = format!("{{\"block\":{expected_block_value}}}");
        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_block_by_hash()
            .return_once(move |_, _| Err(eyre::eyre!("cannot query block by hash")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/ethereum/get_block_by_hash/0xc24215226336d22238a20a72f8e489c005b44c4a/true"
            ))
            .dispatch()
            .await;
        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query block by hash\"}"
        );
    }

    /// Test the `query_priority_fee` endpoint.
    /// `/ethereum/priority_fee`
    /// Given normal conditions, when query block number, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_priority_fee_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_priority_fee()
            .return_once(move || Ok(U256::default()));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/ethereum/priority_fee")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"priority_fee\":\"0\"}"
        );
    }

    /// Test the `query_priority_fee` endpoint.
    /// `/ethereum/priority_fee`
    /// Given Ethereum light client returns error when query priority fee, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_priority_fee_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_priority_fee()
            .return_once(move || Err(eyre::eyre!("cannot query priority fee")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/ethereum/priority_fee")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query priority fee\"}"
        );
    }

    /// Test the `/ethereum/logs` endpoint.
    /// Given normal conditions, when query logs, then errors is propagated.
    #[tokio::test]
    async fn given_normal_conditions_when_query_logs_then_error_is_propagated() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        let error_msg = concat!(
            "Non valid combination of from_block, to_block and blockhash. ",
            "If you want to filter blocks, then ",
            "you can only use either from_block and to_block or blockhash, not both",
        );
        ethereum_lightclient
            .expect_get_logs()
            .return_once(move |_, _, _, _, _| Err(eyre::eyre!(error_msg)));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .post(uri!("/ethereum/logs"))
            .body(r#"{"fromBlock":"finalized","toBlock":"finalized","blockHash": "0x01"}"#)
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(response.into_string().await.unwrap(),
            "{\"error_message\":\"Non valid combination of from_block, to_block and blockhash. If you want to filter blocks, then you can only use either from_block and to_block or blockhash, not both\"}"
        );
    }

    /// Test the `/ethereum/logs` endpoint.
    /// Given normal conditions, when query , then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_logs_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();
        let mut log = Log::default();
        log.topics = vec![ethers::types::TxHash::from_str(
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
        )
        .unwrap()];
        let logs = vec![log];
        ethereum_lightclient
            .expect_get_logs()
            .return_once(move |_, _, _, _, _| Ok(logs));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .post(uri!("/ethereum/logs"))
            .body(r#"{"fromBlock":"finalized","toBlock":"finalized", "topics":["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"]}"#)
            .dispatch()
            .await;

        let expected =
              "{\"logs\":[{\"address\":\"0x0000000000000000000000000000000000000000\",\"topics\":[\"0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef\"],\"data\":\"0x\",\"blockHash\":null,\"blockNumber\":null,\"transactionHash\":null,\"transactionIndex\":null,\"logIndex\":null,\"transactionLogIndex\":null,\"logType\":null,\"removed\":null}]}";

        assert_eq!(response.into_string().await.unwrap(), expected);
    }
    /// Test the `query_starknet_state_root` endpoint.
    /// `/starknet/state/root`
    /// Given Ethereum light client returns error when query balance, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_state_root_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_call()
            .return_once(move |_, _| Err(eyre::eyre!("cannot query state root")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/state/root")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query state root\"}"
        );
    }

    /// Test the `query_starknet_get_storage_at` endpoint.
    /// `/starknet/storage/<contract>/<key>`
    /// Given normal conditions, when query starknet get storage at, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_starknet_get_storage_at_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let expected_result = FieldElement::from_hex_be("298305742194").unwrap();
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_storage_at()
            .times(1)
            .return_once(move |_address, _key, _block_nb| Ok(expected_result));
        ethereum_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Ok(vec![2]));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/storage/0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7/0x341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"result\":\"45642708951444\"}"
        );
    }

    /// Test the `query_starknet_get_storage_at` endpoint.
    /// `/starknet/storage/<contract>/<key>`
    /// Given StarkNet light client returns error when query starknet get storage at, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_query_starknet_get_storage_at_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_storage_at()
            .times(1)
            .return_once(move |_address, _key, _block_nb| {
                Err(eyre::eyre!("cannot query starknet get storage at"))
            });
        ethereum_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Ok(vec![2]));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/storage/0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7/0x341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query starknet get storage at\"}"
        );
    }

    /// Test the `query_starknet_contract_view` endpoint.
    /// `/starknet/view/<contract>/<selector>?<calldata>`
    /// Given normal conditions, when query starknet contract view, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_starknet_contract_view_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let expected_result = vec![
            FieldElement::from_hex_be("0x298305742194").unwrap(),
            FieldElement::from_hex_be("0x00").unwrap(),
        ];
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Ok(expected_result));
        ethereum_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Ok(vec![2]));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/view/0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7/0x341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1?calldata=0x341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"result\":[\"45642708951444\",\"0\"]}"
        );
    }

    /// Test the `query_starknet_contract_view` endpoint.
    /// `/starknet/view/<contract>/<selector>?<calldata>`
    /// Given StarkNet light client returns error when query starknet contract view, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_query_starknet_contract_view_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Err(eyre::eyre!("cannot query starknet call")));
        ethereum_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Ok(vec![2]));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/view/0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7/0x341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1?calldata=0x341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query starknet call\"}"
        );
    }

    /// Test the `query_starknet_get_nonce` endpoint.
    /// `/starknet/nonce/<contract>/`
    /// Given normal conditions, when query starknet get_nonce, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_starknet_get_nonce_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let expected_result = FieldElement::from_hex_be("298305742194").unwrap();

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_nonce()
            .return_once(move |_block_nb, _address| Ok(expected_result));
        ethereum_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Ok(vec![2]));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/nonce/0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"result\":\"45642708951444\"}"
        );
    }

    /// Test the `query_starknet_get_nonce` endpoint.
    /// `/starknet/nonce/<contract>/`
    /// Given StarkNet light client returns error when query starknet get_nonce, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_query_starknet_get_nonce_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_nonce()
            .return_once(move |_block_nb, _address| {
                Err(eyre::eyre!("cannot query starknet address nonce"))
            });
        ethereum_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Ok(vec![2]));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/nonce/0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query starknet address nonce\"}"
        );
    }

    /// Test the `query_l1_to_l2_message_cancellations` endpoint.
    /// `/starknet/messaging/l1_to_l2_message_cancellations/<msg_hash>`
    /// Given normal conditions, when query_l1_to_l2_message_cancellations, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_l1_to_l2_message_cancellations_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        let expected_timestamp = U256::from(1234);
        // Convert to bytes because that's what the mock returns.
        let mut expected_timestamp_bytes: Vec<u8> = vec![0; 32];
        expected_timestamp.to_big_endian(&mut expected_timestamp_bytes);

        // Set the expected return value for the Ethereum light client mock.
        ethereum_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_call_opts, _block_tag| Ok(expected_timestamp_bytes));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/messaging/l1_to_l2_message_cancellations/0x6cf645167cb162944d98f74709dfc8beb8244cc74a34fbcaf59562b4fdbacafa")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"result\":\"1234\"}"
        );
    }

    /// Test the `query_l1_to_l2_message_cancellations` endpoint.
    /// `/starknet/messaging/l1_to_l2_message_cancellations/<msg_hash>`
    /// Given Ethereum light client returns error when query_l1_to_l2_message_cancellations, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_ligthclient_returns_error_when_query_l1_to_l2_message_cancellations_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        ethereum_lightclient
            .expect_call()
            .return_once(move |_block_nb, _address| Err(eyre::eyre!("cannot query")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/messaging/l1_to_l2_message_cancellations/0x6cf645167cb162944d98f74709dfc8beb8244cc74a34fbcaf59562b4fdbacafa")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query\"}"
        );
    }

    /// Test the `query_l1_to_l2_messages` endpoint.
    /// `/starknet/messaging/l1_to_l2_messages/<msg_hash>`
    /// Given normal conditions, when query_l1_to_l2_messages, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_l1_to_l2_messages_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        let expected_timestamp = U256::from(1234);
        // Convert to bytes because that's what the mock returns.
        let mut expected_timestamp_bytes: Vec<u8> = vec![0; 32];
        expected_timestamp.to_big_endian(&mut expected_timestamp_bytes);

        // Set the expected return value for the Ethereum light client mock.
        ethereum_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_call_opts, _block_tag| Ok(expected_timestamp_bytes));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/messaging/l1_to_l2_messages/0x6cf645167cb162944d98f74709dfc8beb8244cc74a34fbcaf59562b4fdbacafa")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"result\":\"1234\"}"
        );
    }

    /// Test the `query_l1_to_l2_messages` endpoint.
    /// `/starknet/messaging/l1_to_l2_messages/<msg_hash>`
    /// Given Ethereum light client returns error when query_l1_to_l2_messages, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_ligthclient_returns_error_when_query_l1_to_l2_messages_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        ethereum_lightclient
            .expect_call()
            .return_once(move |_block_nb, _address| Err(eyre::eyre!("cannot query")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/messaging/l1_to_l2_messages/0x6cf645167cb162944d98f74709dfc8beb8244cc74a34fbcaf59562b4fdbacafa")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query\"}"
        );
    }

    /// Test the `query_l2_to_l1_message` endpoint.
    /// `/starknet/messaging/l2_to_l1_messages/<msg_hash>`
    /// Given normal conditions, when query_l2_to_l1_message, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_l2_to_l1_message_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        let expected_fee = U256::from(1234);
        // Convert to bytes because that's what the mock returns.
        let mut expected_fee_bytes: Vec<u8> = vec![0; 32];
        expected_fee.to_big_endian(&mut expected_fee_bytes);

        // Set the expected return value for the Ethereum light client mock.
        ethereum_lightclient
            .expect_call()
            .return_once(move |_call_opts, _block_tag| Ok(expected_fee_bytes));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/messaging/l2_to_l1_messages/0x1f83c4cce9da5a3a089a76501b8da5f7400e80f398594c4f1715ad1cb1a14012")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"result\":\"1234\"}"
        );
    }

    /// Test the `query_l2_to_l1_message` endpoint.
    /// `/starknet/messaging/l2_to_l1_messages/<msg_hash>`
    /// Given StarkNet light client returns error when query_l2_to_l1_message, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_query_l2_to_l1_message_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the Ethereum light client mock.
        ethereum_lightclient
            .expect_call()
            .return_once(move |_call_opts, _block_tag| Err(eyre::eyre!("cannot query")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/messaging/l2_to_l1_messages/0x1f83c4cce9da5a3a089a76501b8da5f7400e80f398594c4f1715ad1cb1a14012")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query\"}"
        );
    }

    /// Test the `chain_id` endpoint.
    /// `/starknet/chain_id`
    #[tokio::test]
    async fn query_starknet_chain_id_should_return_chain_id() {
        // Given
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();
        let expected_result = FieldElement::from_dec_str("123").unwrap();
        starknet_lightclient
            .expect_chain_id()
            .return_once(move || Ok(expected_result));
        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/chain_id")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"chain_id\":\"123\"}"
        );
    }

    /// Test the `block_number` endpoint.
    /// `/starknet/block_number`
    /// Given normal conditions, when query starknet block_number, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_starknet_block_number_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let expected_result: u64 = 123456;

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_block_number()
            .return_once(move || Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/block_number")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"block_number\":\"123456\"}"
        );
    }

    /// Test the `block_number` endpoint.
    /// `/starknet/block_number`
    /// Given StarkNet light client returns error when query starknet block_number, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_query_starknet_block_number_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_block_number()
            .return_once(move || Err(eyre::eyre!("cannot query starknet address block number")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/block_number")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query starknet address block number\"}"
        );
    }

    /// Test the `block_hash_and_number` endpoint.
    /// `/starknet/block_hash_and_number`
    /// Given normal conditions, when query starknet block_hash_and_number, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_starknet_block_hash_and_number_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let expected_result = BlockHashAndNumber {
            block_hash: FieldElement::from_dec_str("123456").unwrap(),
            block_number: 123456,
        };

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_block_hash_and_number()
            .return_once(move || Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!("/starknet/block_hash_and_number"))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"block_hash\":\"123456\",\"block_number\":\"123456\"}"
        );
    }

    /// Test the `block_hash_and_number` endpoint.
    /// `/starknet/block_hash_and_number`
    /// Given StarkNet light client returns error when query starknet block_hash_and_number, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_query_starknet_block_hash_and_number_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_block_hash_and_number()
            .return_once(move || Err(eyre::eyre!("cannot query starknet address block number")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!("/starknet/block_hash_and_number"))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query starknet address block number\"}"
        );
    }

    /// Test the `query_l1_to_l2_message_nonce` endpoint.
    /// `/starknet/messaging/l1_to_l2_message_nonce`
    /// Given normal conditions, when query_l1_to_l2_messages, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_l1_to_l2_message_nonce_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        let expected_nonce = U256::from(1234);
        // Convert to bytes because that's what the mock returns.
        let mut expected_nonce_bytes: Vec<u8> = vec![0; 32];
        expected_nonce.to_big_endian(&mut expected_nonce_bytes);

        // Set the expected return value for the Ethereum light client mock.
        ethereum_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_call_opts, _block_tag| Ok(expected_nonce_bytes));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!("/starknet/messaging/l1_to_l2_message_nonce"))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"result\":\"1234\"}"
        );
    }

    /// Test the `query_l1_to_l2_message_nonce` endpoint.
    /// `/starknet/messaging/l1_to_l2_message_nonce`
    /// Given Ethereum light client returns error when query_l1_to_l2_message_nonce, then error is propagated.
    #[tokio::test]
    async fn given_ethereum_ligthclient_returns_error_when_query_l1_to_l2_message_nonce_then_error()
    {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Set the expected return value for the StarkNet light client mock.
        ethereum_lightclient
            .expect_call()
            .return_once(move |_block_nb, _address| Err(eyre::eyre!("Ethereum client error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!("/starknet/messaging/l1_to_l2_message_nonce"))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"Ethereum client error\"}"
        );
    }

    /// Test the `get_class` endpoint.
    /// `/starknet/contract/class/<class_hash>?<block_id>&<block_id_type>`
    /// Given normal conditions, when query starknet get_class, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_get_class_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let (expected_result, expected_result_value) =
            beerus_core::starknet_helper::create_mock_contract_class();

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_class()
            .return_once(move |_block_id, _class_hash| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/starknet/contract/class/0x123?block_id=123&block_id_type=number"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            serde_json::to_string(&expected_result_value).unwrap()
        );
    }

    /// Test the `get_class` endpoint.
    /// `/starknet/contract/class/<class_hash>?<block_id>&<block_id_type>`
    /// Given StarkNet light client returns error when query starknet get_class, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_get_class_then_error_is_propagated() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_class()
            .return_once(move |_block_id, _class_hash| {
                Err(eyre::eyre!("cannot query starknet contract class"))
            });

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/starknet/contract/class/0x123?block_id=123&block_id_type=number"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query starknet contract class\"}"
        );
    }

    /// Test the `get_class_at` endpoint.
    /// `/starknet/contract/class_att/<class_hash>?<block_id>&<block_id_type>`
    /// Given normal conditions, when query starknet get_class_at, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_get_class_at_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let (expected_result, expected_result_value) =
            beerus_core::starknet_helper::create_mock_contract_class();

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_class_at()
            .return_once(move |_block_id, _contract_address| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/starknet/contract/class_at/0x123?block_id=123&block_id_type=number"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            serde_json::to_string(&expected_result_value).unwrap()
        );
    }

    /// Test the `get_class_at` endpoint.
    /// `/starknet/contract/class/<class_hash>?<block_id>&<block_id_type>`
    /// Given StarkNet light client returns error when query starknet get_class_at, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_get_class_at_then_error_is_propagated() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient.expect_get_class_at().return_once(
            move |_block_id, _contract_address| {
                Err(eyre::eyre!("cannot query starknet contract class"))
            },
        );

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/starknet/contract/class_at/0x123?block_id=123&block_id_type=number"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query starknet contract class\"}"
        );
    }

    /// Test the `get_class_hash` endpoint.
    /// `/starknet/contract/class_hash/<contract_address>?<block_id>&<block_id_type>`
    /// Given normal conditions, when query starknet get_class, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_get_class_hash_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let expected_result = FieldElement::from_str("123").unwrap();

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_class_hash_at()
            .return_once(move |_block_id, _contract_address| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/starknet/contract/class_hash/0x123?block_id=123&block_id_type=number"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"class_hash\":\"123\"}".to_string()
        );
    }

    /// Test the `get_class_hash` endpoint.
    /// `/starknet/contract/class_hash/<contract_address>?<block_id>&<block_id_type>`
    /// Given StarkNet light client returns error when query starknet get_class, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_get_class_hash_then_error_is_propagated()
    {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient.expect_get_class_hash_at().return_once(
            move |_block_id, _contract_address| {
                Err(eyre::eyre!("cannot query starknet address class hash"))
            },
        );

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/starknet/contract/class_hash/0x123?block_id=123&block_id_type=number"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query starknet address class hash\"}"
        );
    }

    fn config_and_mocks() -> (Config, MockEthereumLightClient, MockStarkNetLightClient) {
        let config = Config {
            ethereum_network: "mainnet".to_string(),
            ethereum_consensus_rpc: "http://localhost:8545".to_string(),
            ethereum_execution_rpc: "http://localhost:8545".to_string(),
            starknet_rpc: "http://localhost:8545".to_string(),
            data_dir: Some(PathBuf::from("/tmp")),
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

    /// Test the `get_block_transaction_count` endpoint.
    /// `/starknet/block_transaction_count?<block_id>&<block_id_type>`
    /// Given normal conditions, when query starknet get_block_transaction_count, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_starknet_get_block_transaction_count_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let expected_result: u64 = 34;

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_block_transaction_count()
            .return_once(move |_block_id| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/starknet/block_transaction_count?block_id=123&block_id_type=number"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"block_transaction_count\":\"34\"}"
        );
    }

    /// Test the `get_block_transaction_count` endpoint.
    /// `/starknet/block_transaction_count?<block_id>&<block_id_type>`
    /// Given StarkNet light client returns error when query starknet get_block_transaction_count, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_query_starknet_get_block_transaction_count_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_block_transaction_count()
            .return_once(move |_block_id| {
                Err(eyre::eyre!("cannot query starknet block transaction count"))
            });

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/starknet/block_transaction_count?block_id=123&block_id_type=number"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query starknet block transaction count\"}"
        );
    }

    /// Test the `get_class_at` endpoint.
    /// `/starknet/contract/class_att/<class_hash>?<block_id>&<block_id_type>`
    /// Given normal conditions, when query starknet syncing, then ok.
    /// Case: Node Starknet is syncing.
    #[tokio::test]
    async fn given_normal_conditions_when_query_syncing_case_status_syncing_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let (expected_result, expected_result_value, _) =
            beerus_core::starknet_helper::create_mock_syncing_case_syncing();

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_syncing()
            .return_once(move || Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/syncing")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            serde_json::to_string(&expected_result_value).unwrap()
        );
    }

    /// Test the `syncing` endpoint.
    /// `/starknet/syncing`
    /// Given normal conditions, when query starknet syncing, then ok.
    /// Case: Node Starknet is not syncing.
    #[tokio::test]
    async fn given_normal_conditions_when_query_syncing_case_status_not_syncing_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let (expected_result, expected_result_value) =
            beerus_core::starknet_helper::create_mock_syncing_case_not_syncing();

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_syncing()
            .return_once(move || Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/syncing")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            serde_json::to_string(&expected_result_value).unwrap()
        );
    }

    /// Test the `syncing` endpoint.
    /// `/starknet/syncing`
    /// Given StarkNet light client returns error when query starknet syncing, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_syncing_then_error_is_propagated() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_syncing()
            .return_once(move || Err(eyre::eyre!("cannot query starknet syncing")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client.get(uri!("/starknet/syncing")).dispatch().await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query starknet syncing\"}"
        );
    }

    /// Test the `state_update` endpoint.
    /// `/starknet/state_update?<block_id>&<block_id_type>`
    /// Given StarkNet light client returns error when query starknet state_update, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_query_starknet_update_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_state_update()
            .return_once(move |_block_id| Err(eyre::eyre!("Invalid Tag")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/starknet/state_update?block_id=non_existent&block_id_type=tag"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"Invalid Tag\"}"
        );
    }

    #[tokio::test]
    async fn given_normal_conditions_when_query_starknet_state_update_count_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();
        // Given
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

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_state_update()
            .return_once(move |_block_id| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/starknet/state_update?block_id_type=tag&block_id=latest"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"block_hash\":\"1\",\"new_root\":\"1\",\"old_root\":\"1\",\"state_diff\":{\"storage_diffs\":[],\"declared_contract_hash\":[],\"deployed_contracts\":[],\"nonces\":[]}}"
                )
    }

    /// Test the `/ethereum/add_invoke_transaction` endpoint.
    /// Given normal conditions, when query , then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_add_invoke_transaction_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        let expected_result = InvokeTransactionResult {
            transaction_hash: FieldElement::from_str("0x01").unwrap(),
        };
        let expected_result_value = expected_result.clone();
        starknet_lightclient
            .expect_add_invoke_transaction()
            .return_once(move |_| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");
        // When
        let response = client
            .post(uri!("/starknet/add_invoke_transaction"))
            .body(r#"{"maxFee":"0","signature":[], "nonce":"10", "contractAddress":"0", "entryPointSelector":"0","calldata":[] }"#)
            .dispatch()
            .await;

        let expected = "{\"transaction_hash\":\"1\"}";

        assert_eq!(response.into_string().await.unwrap(), expected);
    }

    /// Test the `/ethereum/add_invoke_transaction` endpoint.
    /// Given normal conditions, when query add_invoke_transaction, then errors is propagated.
    #[tokio::test]
    async fn given_normal_conditions_when_add_invoke_transaction_error_is_propagated() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        let error_msg = concat!("Failed to send invoke transaction");
        starknet_lightclient
            .expect_add_invoke_transaction()
            .return_once(move |_| Err(eyre::eyre!(error_msg)));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .post(uri!("/starknet/add_invoke_transaction"))
            .body(r#"{"maxFee":"0","signature":[], "nonce":"10", "contractAddress":"0", "entryPointSelector":"0","calldata":[] }"#)
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"Failed to send invoke transaction\"}"
        );
    }

    /// Test the `/starknet/add_deploy_transaction` endpoint.
    /// Given normal conditions, when query , then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_add_deploy_transaction_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        let expected_result = DeployTransactionResult {
            transaction_hash: FieldElement::from_str("0x01").unwrap(),
            contract_address: FieldElement::from_str("0x02").unwrap(),
        };
        let expected_result_value = expected_result.clone();
        starknet_lightclient
            .expect_add_deploy_transaction()
            .return_once(move |_| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        let body = r#"{
                "contractClass": "{\"program\":\"\",\"entry_points_by_type\":{\"CONSTRUCTOR\":[{\"offset\":\"0xa\",\"selector\":\"0x0\"}],\"EXTERNAL\":[{\"offset\":\"0xa\",\"selector\":\"0x0\"}],\"L1_HANDLER\":[{\"offset\":\"0xa\",\"selector\":\"0x0\"}]}}",
                "version": "10",
                "contractAddressSalt": "0",
                "constructorCalldata": ["10"]
        }"#;

        // When
        let response = client
            .post(uri!("/starknet/add_deploy_transaction"))
            .body(body)
            .dispatch()
            .await;

        let expected = "{\"transaction_hash\":\"1\",\"contract_address\":\"2\"}";

        assert_eq!(response.into_string().await.unwrap(), expected);
    }

    /// Test the `/starknet/add_deploy_transaction` endpoint.
    /// Given normal conditions, when query add_deploy_transaction, then errors is propagated.
    #[tokio::test]
    async fn given_normal_conditions_when_add_deploy_transaction_error_is_propagated() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        let error_msg = concat!("Failed to send deploy transaction");
        starknet_lightclient
            .expect_add_deploy_transaction()
            .return_once(move |_| Err(eyre::eyre!(error_msg)));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        let body = r#"{
                "contractClass": "{\"program\":\"\",\"entry_points_by_type\":{\"CONSTRUCTOR\":[{\"offset\":\"0xa\",\"selector\":\"0x0\"}],\"EXTERNAL\":[{\"offset\":\"0xa\",\"selector\":\"0x0\"}],\"L1_HANDLER\":[{\"offset\":\"0xa\",\"selector\":\"0x0\"}]}}",
                "version": "10",
                "contractAddressSalt": "0",
                "constructorCalldata": ["10"]
            }"#;

        // When
        let response = client
            .post(uri!("/starknet/add_deploy_transaction"))
            .body(body)
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"Failed to send deploy transaction\"}"
        );
    }

    /// Test the `get_block_with_txs` endpoint.
    /// `/starknet/block_with_txs/<block_id>?<block_id_type>`
    /// Given normal conditions, when query starknet get_block_with_txs, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_get_block_with_txs_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

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

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_block_with_txs()
            .return_once(move |_block_id| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!("/starknet/block_with_txs/123?block_id_type=number"))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"block_with_txs\":\"Block(BlockWithTxs { status: Pending, block_hash: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, parent_hash: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, block_number: 0, new_root: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, timestamp: 10, sequencer_address: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, transactions: [] })\"}",
        );
    }

    /// Test the `get_block_with_txs` endpoint.
    /// `/starknet/get_block_with_txs/<block_id>?<block_id_type>`
    /// Given StarkNet light client returns error when query starknet get_block_with_txs, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_get_block_with_txs_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_block_with_txs()
            .return_once(move |_block_id| Err(eyre::eyre!("cannot query starknet block with txs")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!("/starknet/block_with_txs/123?block_id_type=number"))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query starknet block with txs\"}"
        );
    }

    /// Test the `get_transaction_by_block_id_and_index` endpoint.
    /// `/starknet/transaction_by_block_and_index/<index>?<block_id>&<block_id_type>"`
    /// Given normal conditions, when query starknet transaction_by_block_id_and_index, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_starknet_get_transaction_by_block_id_and_index_then_ok(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

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

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_transaction_by_block_id_and_index()
            .return_once(move |_block_id, _index| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");
        // When
        let response = client
            .get(uri!(
                "/starknet/transaction_by_block_and_index/0?block_id=123&block_id_type=number"
            ))
            .dispatch()
            .await;

        // Then
        let expected_result_value = "{\"transaction_data\":\"Invoke(V0(InvokeTransactionV0 { transaction_hash: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, max_fee: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, signature: [], nonce: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, contract_address: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, entry_point_selector: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, calldata: [] }))\"}".to_string();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), expected_result_value);
    }

    /// Test the `transaction_by_block_id_and_index` endpoint.
    /// `/starknet/transaction_by_block_and_index/<index>?<block_id>&<block_id_type>"`
    /// Given StarkNet light client returns error when query starknet transaction_by_block_id_and_index, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_query_starknet_get_transaction_by_block_id_and_index_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_transaction_by_block_id_and_index()
            .return_once(move |_block_id, _index| {
                Err(eyre::eyre!("cannot query starknet transaction"))
            });

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/starknet/transaction_by_block_and_index/0?block_id=123&block_id_type=number"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query starknet transaction\"}"
        );
    }

    /// Test the `transaction_receipt` endpoint.
    /// `/starknet/transaction_receipt/<tx_hash>`
    /// Given normal conditions, when query starknet transaction_receipt, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_starknet_transaction_receipt_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
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

        starknet_lightclient
            .expect_get_transaction_receipt()
            .return_once(move |_| Ok(closure_return));

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_transaction_receipt()
            .return_once(move |_| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!("/starknet/transaction_receipt/0x1"))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"tx_receipt\":\"Receipt(Invoke(InvokeTransactionReceipt { transaction_hash: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, actual_fee: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, status: AcceptedOnL2, block_hash: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, block_number: 4095, messages_sent: [], events: [] }))\"}"
        );
    }

    /// Test the `transaction_receipt` endpoint.
    /// `/starknet/transaction_receipt/<tx_hash>`
    /// Given StarkNet light client returns error when query starknet transaction_receipt, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_query_starknet_transaction_receipt_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_transaction_receipt()
            .return_once(move |_| {
                Err(eyre::eyre!(
                    r#"{{"error_message":"JSON-RPC error: code=25, message=\"Transaction hash not found\""}}"#
                ))
            });

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!("/starknet/transaction_receipt/0xAF"))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"{\\\"error_message\\\":\\\"JSON-RPC error: code=25, message=\\\\\\\"Transaction hash not found\\\\\\\"\\\"}\"}"
        );
    }
    /// Test the `pending_transactions` endpoint.
    /// `/starknet/block_transaction_count?<block_id>&<block_id_type>`
    /// Given normal conditions, when query starknet pending_transactions, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_starknet_pending_transactions_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let expected_result = vec![];

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_pending_transactions()
            .return_once(move || Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!("/starknet/pending_transactions"))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"pending_transactions\":\"[]\"}"
        );
    }

    /// Test the `pending_transactions` endpoint.
    /// `/starknet/block_transaction_count?<block_id>&<block_id_type>`
    /// Given StarkNet light client returns error when query starknet pending_transactions, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_query_starknet_pending_transactions_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_pending_transactions()
            .return_once(move || Err(eyre::eyre!("cannot query starknet pending transactions")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!("/starknet/pending_transactions"))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query starknet pending transactions\"}"
        );
    }

    /// Test the `get_block_with_tx_hashes` endpoint.
    /// `/starknet/block_with_txs/<block_id>?<block_id_type>`
    /// Given normal conditions, when query starknet get_block_with_tx_hashes, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_get_block_with_tx_hashes_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

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

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_block_with_tx_hashes()
            .return_once(move |_block_id| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/starknet/block_with_tx_hashes/123?block_id_type=number"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"block_with_tx_hashes\":\"Block(BlockWithTxHashes { status: Pending, block_hash: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, parent_hash: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, block_number: 0, new_root: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, timestamp: 10, sequencer_address: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, transactions: [] })\"}",
        );
    }

    /// Test the `get_block_with_tx_hashes` endpoint.
    /// `/starknet/get_block_with_tx_hashes/<block_id>?<block_id_type>`
    /// Given StarkNet light client returns error when query starknet get_block_with_tx_hashes, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_get_block_with_tx_hashes_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_block_with_tx_hashes()
            .return_once(move |_block_id| Err(eyre::eyre!("cannot query starknet block with txs")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/starknet/block_with_tx_hashes/123?block_id_type=number"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query starknet block with txs\"}"
        );
    }
    /// Test the `transaction_by_hash` endpoint.
    /// `/starknet/transaction_by_hash>?<block_id>&<block_id_type>"`
    /// Given StarkNet light client returns error when query starknet transaction_by_hash, then error is propagated.
    #[tokio::test]
    async fn given_starknet_ligthclient_returns_error_when_query_starknet_get_transaction_by_hash_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_transaction_by_hash()
            .return_once(move |_block_id| Err(eyre::eyre!("cannot query starknet transaction")));
        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!("/starknet/transaction_by_hash/0x06986c739c4ab040"))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"cannot query starknet transaction\"}"
        );
    }
    /// Test the `get_transaction_by_hash` endpoint.
    /// `/starknet/transaction_by_hash>?<block_id>&<block_id_type>"`
    /// Given normal conditions, when query starknet transaction_by_hash, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_starknet_get_transaction_by_hash_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Mock the `get_transaction_by_hash` method of the Starknet light client.
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

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_transaction_by_hash()
            .return_once(move |_| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");
        // When
        let response = client
            .get(uri!(
                "/starknet/transaction_by_hash/0?block_id=123&block_id_type=number"
            ))
            .dispatch()
            .await;

        // Then
        let expected_result_value = "{\"transaction\":\"Invoke(V0(InvokeTransactionV0 { transaction_hash: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, max_fee: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, signature: [], nonce: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, contract_address: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, entry_point_selector: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, calldata: [] }))\"}".to_string();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), expected_result_value);
    }

    #[tokio::test]
    async fn given_normal_condition_when_query_starknet_contract_storage_proof_then_should_work() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        let proof: GetProofOutput = serde_json::from_str(
            r#"{
            "contract_proof": [
            {
                "binary": {
                    "left": "0x15e7882b80e22844ca62d3e3260a21d0d45c2b0c1744328e2763b4b486de738",
                    "right": "0x7779bcf84c8a6a4cca695c2d44d1455db0cb13d457ea7a01887676b9f779455"
                }
            },
            {
                "edge": {
                    "path": {
                        "value": "0x1",
                        "len": 1
                    },
                    "child": "0x173d276dbe8497dd2d59b88aa7eaebeb760e450e7a34a1ae5d513a930a3bf9d"
                }
            }
            ],
            "contract_data": null
        }"#,
        )
        .unwrap();

        let return_value = proof.clone();
        starknet_lightclient
            .expect_get_contract_storage_proof()
            .return_once(move |_contract_address, _keys, _block_id| Ok(return_value));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");

        // When
        let response = client
            .get(uri!(
                "/starknet/contract_storage_proof/0x4d4e07157aeb54abeb64f5792145f2e8db1c83bda01a8f06e050be18cfb8153/0x1?block_id_type=number&block_id=123"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"proof\":{\"contract_proof\":[{\"binary\":{\"left\":\"0x15e7882b80e22844ca62d3e3260a21d0d45c2b0c1744328e2763b4b486de738\",\"right\":\"0x7779bcf84c8a6a4cca695c2d44d1455db0cb13d457ea7a01887676b9f779455\"}},{\"edge\":{\"path\":{\"value\":\"0x1\",\"len\":1},\"child\":\"0x173d276dbe8497dd2d59b88aa7eaebeb760e450e7a34a1ae5d513a930a3bf9d\"}}],\"contract_data\":null}}"
        );
    }

    #[tokio::test]
    async fn given_invalid_contract_address_when_query_starknet_contract_storage_proof_then_should_fail(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        starknet_lightclient
            .expect_get_contract_storage_proof()
            .return_once(|_contract_address, _keys, _block_id| Err(eyre::eyre!("mocked error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");
        let response = client
            .get(uri!(
                "/starknet/contract_storage_proof/INVALID/0x1?block_id_type=number&block_id=123"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"invalid character\"}"
        );
    }

    #[tokio::test]
    async fn given_invalid_key_when_query_starknet_contract_storage_proof_then_should_fail() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        starknet_lightclient
            .expect_get_contract_storage_proof()
            .return_once(|_contract_address, _keys, _block_id| Err(eyre::eyre!("mocked error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Build the Rocket instance.
        let client = Client::tracked(build_rocket_server(beerus).await)
            .await
            .expect("valid rocket instance");
        let response = client
            .get(uri!(
                "/starknet/contract_storage_proof/0x123/INVALID?block_id_type=number&block_id=123"
            ))
            .dispatch()
            .await;

        // Then
        assert_eq!(response.status(), Status::InternalServerError);
        assert_eq!(
            response.into_string().await.unwrap(),
            "{\"error_message\":\"invalid character\"}"
        );
    }
}
