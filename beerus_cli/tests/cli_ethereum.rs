#[cfg(test)]
mod test {
    use std::str::FromStr;

    use beerus_cli::{
        model::{
            Cli, Commands, EthereumCommands, EthereumSubCommands, StarkNetCommands,
            StarkNetSubCommands,
        },
        runner,
    };
    use beerus_core::{
        config::Config,
        lightclient::{
            beerus::BeerusLightClient, ethereum::MockEthereumLightClient,
            starknet::MockStarkNetLightClient,
        },
    };
    use ethers::types::{Address, Transaction, H256, U256};
    use helios::types::{ExecutionBlock, Transactions};
    use starknet::{core::types::FieldElement, providers::jsonrpc::models::BlockHashAndNumber};

    /// Test the `query_state_root` CLI command.
    /// Given normal conditions, when query state root, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_state_root_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
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

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryStateRoot {},
            }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!(
            "2593003852473857760763774375943570015682902311385614557145528717605591462989",
            result.to_string()
        );
    }

    /// Test the `query_state_root` CLI command.
    /// Given ethereum lightclient returns an error, when query state root, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_state_root_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_call()
            .return_once(move |_, _| Err(eyre::eyre!("ethereum_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryStateRoot {},
            }),
        };
        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("ethereum_lightclient_error", e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }
    }

    /// Test the `query_storage` CLI command.
    /// Given normal conditions, when query storage, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_query_storage_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let expected_result = FieldElement::from_dec_str("298305742194").unwrap();
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

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryGetStorageAt {
                    address: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7"
                        .to_string(),
                    key: "0x341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1"
                        .to_string(),
                },
            }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!("298305742194", result.to_string());
    }

    /// Test the `query_storage` CLI command.
    /// Given starknet lightclient returns an error, when query storage, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_query_storage_then_error_is_propagated()
    {
        // Build mocks.
        let (config, mut ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_storage_at()
            .times(1)
            .return_once(move |_address, _key, _block_nb| {
                Err(eyre::eyre!("starknet_lightclient_error"))
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

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryGetStorageAt {
                    address: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7"
                        .to_string(),
                    key: "0x341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1"
                        .to_string(),
                },
            }),
        };
        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("starknet_lightclient_error", e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }
    }

    /// Test the `query_nonce` CLI command.
    /// Given normal conditions, when query contract, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_query_contract_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let expected_result = vec![
            FieldElement::from_dec_str("123").unwrap(),
            FieldElement::from_dec_str("456").unwrap(),
        ];

        // Set the expected return value for the Ethereum light client mock.
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

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryContract {
                    address: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7"
                        .to_string(),
                    selector: "0x341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1"
                        .to_string(),
                    calldata: vec![
                        "0x341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1"
                            .to_string(),
                        "0x341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1"
                            .to_string(),
                    ],
                },
            }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();
        // Then
        assert_eq!("[123, 456]", result.to_string());
    }

    /// Test the `query_nonce` CLI command.
    /// Given normal conditions, when query nonce, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_query_nonce_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let expected_result = FieldElement::from_dec_str("298305742194").unwrap();
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

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryNonce {
                    address: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7"
                        .to_string(),
                },
            }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!("298305742194", result.to_string());
    }

    /// Test the `query_nonce` CLI command.
    /// Given starknet lightclient returns an error, when query nonce, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_query_nonce_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_nonce()
            .return_once(move |_block_nb, _address| Err(eyre::eyre!("starknet_lightclient_error")));
        ethereum_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Ok(vec![2]));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryNonce {
                    address: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7"
                        .to_string(),
                },
            }),
        };
        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("starknet_lightclient_error", e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }
    }

    /// Test the `query_chain_id` CLI command.
    /// Given normal conditions, when query chain_id, then ok.
    /// Success case.
    #[tokio::test]
    async fn starknet_chain_id_should_return_the_chain_id() {
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
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryChainId {},
            }),
        };

        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!("Chain id: 123", result.to_string());
    }

    /// Test the `query_block_number` CLI command.
    /// Given normal conditions, when query block_number, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_query_block_number_then_ok() {
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

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryBlockNumber {},
            }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!("Block number: 123456", result.to_string());
    }

    /// Test the `query_block_number` CLI command.
    /// Given starknet lightclient returns an error, when query block_number, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_query_block_number_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_block_number()
            .return_once(move || Err(eyre::eyre!("starknet_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryBlockNumber {},
            }),
        };
        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("starknet_lightclient_error", e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }
    }

    /// Test the `query_block_hash_and_number` CLI command.
    /// Given normal conditions, when query block_hash_and_number, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_query_block_hash_and_number_then_ok() {
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

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryBlockHashAndNumber {},
            }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!(
            "Block hash: 123456, Block number: 123456",
            result.to_string()
        );
    }

    /// Test the `query_block_hash_and_number` CLI command.
    /// Given starknet lightclient returns an error, when query block_hash_and_number, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_query_block_hash_and_number_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_block_hash_and_number()
            .return_once(move || Err(eyre::eyre!("starknet_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryBlockHashAndNumber {},
            }),
        };
        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("starknet_lightclient_error", e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }
    }

    /// Test the `get_class` CLI command.
    /// Given normal conditions, when query get_class, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_get_class_then_ok() {
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

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryGetClass {
                    block_id_type: "number".to_string(),
                    block_id: "123".to_string(),
                    class_hash: "0x123".to_string(),
                },
            }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!(
            result.to_string(),
            serde_json::to_string(&expected_result_value).unwrap()
        );
    }

    /// Test the `get_class` CLI command.
    /// Given starknet lightclient returns an error, when query get_class, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_get_class_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_class()
            .return_once(move |_block_id, _class_hash| {
                Err(eyre::eyre!("starknet_lightclient_error"))
            });

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryGetClass {
                    block_id_type: "number".to_string(),
                    block_id: "123".to_string(),
                    class_hash: "0x123".to_string(),
                },
            }),
        };
        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("starknet_lightclient_error", e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }
    }

    /// Test the `query_starknet_l2_to_l1_messages ` CLI command.
    /// Given normal conditions, when query starknet l2 to l1 messages, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_query_starknet_l2_to_l1_messages_then_ok() {
        // Given
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Expected block number.
        let expected_fee = U256::from(1234);
        // Convert to bytes because that's what the mock returns.
        let mut expected_fee_bytes: Vec<u8> = vec![0; 32];
        expected_fee.to_big_endian(&mut expected_fee_bytes);

        ethereum_lightclient
            .expect_call()
            .return_once(move |_call_opts, _block_tag| Ok(expected_fee_bytes));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::L2ToL1Messages {
                    msg_hash: "0".to_string(),
                },
            }),
        };

        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!(expected_fee.to_string(), result.to_string());
    }

    /// Test the `query_starknet_l2_to_l1_messages ` CLI command.
    /// Given starknet lightclient returns an error, when query starknet l2 to l1 messages, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_starknet_query_starknet_l2_to_l1_messages_then_error_is_propagated(
    ) {
        // Given
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();
        ethereum_lightclient
            .expect_call()
            .return_once(move |_call_opts, _block_tag| {
                Err(eyre::eyre!("ethereum_lightclient_error"))
            });

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::L2ToL1Messages {
                    msg_hash: "0".to_string(),
                },
            }),
        };

        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("ethereum_lightclient_error", e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }
    }

    /// Test the `starknet_l1_to_l2_message_nonce` CLI command.
    /// Given normal conditions, when query nonce, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_l1_to_l2_message_nonce_then_ok() {
        // Given
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Expected block number.
        let expected_nonce = U256::from(1234);
        // Convert to bytes because that's what the mock returns.
        let mut expected_nonce_bytes: Vec<u8> = vec![0; 32];
        expected_nonce.to_big_endian(&mut expected_nonce_bytes);

        // Mock the next call to the Ethereum light client (starknet_core.l1ToL2MessageNonce)
        ethereum_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_call_opts, _block_tag| Ok(expected_nonce_bytes));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::L1ToL2MessageNonce {},
            }),
        };

        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!("L1 to L2 Message Nonce: 1234", result.to_string());
    }

    /// Test the `starknet_l1_to_l2_message_nonce` CLI command.
    /// Given normal conditions, when query nonce, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_ethereum_client_error_when_starknet_l1_to_l2_message_nonce_then_error() {
        // Given
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        let expected_error = "Ethereum light client error";

        // Mock the next call to the Ethereum light client (starknet_core.l1ToL2MessageNonce)
        ethereum_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_call_opts, _block_tag| Err(eyre::eyre!(expected_error)));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::L1ToL2MessageNonce {},
            }),
        };

        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!(expected_error, e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }
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
