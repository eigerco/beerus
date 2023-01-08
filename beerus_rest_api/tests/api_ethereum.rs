#[cfg(test)]
mod test {
    use std::str::FromStr;

    use beerus_core::{
        config::Config,
        lightclient::{
            beerus::BeerusLightClient, ethereum::MockEthereumLightClient,
            starknet::MockStarkNetLightClient,
        },
    };
    use beerus_rest_api::build_rocket_server;
    use ethers::types::{Address, Transaction};
    use ethers::types::{H256, U256};
    use helios::types::{ExecutionBlock, Transactions};
    use rocket::{http::Status, local::asynchronous::Client, uri};

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

    fn config_and_mocks() -> (Config, MockEthereumLightClient, MockStarkNetLightClient) {
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
