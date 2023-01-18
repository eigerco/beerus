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
    use starknet::{
        core::types::FieldElement,
        providers::jsonrpc::models::{
            BlockHashAndNumber, BlockStatus, BlockWithTxs, ContractClass, ContractEntryPoint,
            DeployTransactionResult, EntryPointsByType, InvokeTransaction, InvokeTransactionResult,
            InvokeTransactionV0, InvokeTransactionV1, MaybePendingBlockWithTxs, StateDiff,
            StateUpdate, Transaction as StarknetTransaction,
        },
    };

    /// Test the `send_raw_transaction` CLI command.
    /// Given normal conditions, when sending raw transaction, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_send_raw_transaction_ok() {
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

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::SendRawTransaction {
                    bytes: "0xc24215226336d22238a20a72f8e489c005b44c4a".to_string(),
                },
            }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();

        println!("{}", &result);
        // Then
        assert_eq!("0xc9bb…c31d", format!("{}", result));
    }

    /// Test the `send_raw_transaction` CLI command.
    /// Given ethereum lightclient returns an error, when sending a raw transaction, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_send_raw_transaction_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_send_raw_transaction()
            .return_once(move |_| Err(eyre::eyre!("ethereum_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::SendRawTransaction {
                    bytes: "0xc24215226336d22238a20a72f8e489c005b44c4a".to_string(),
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

    /// Test the `query_balance` CLI command.
    /// Given normal conditions, when query balance, then ok.
    /// Success case.
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

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryBalance {
                    address: "0xc24215226336d22238a20a72f8e489c005b44c4a".to_string(),
                },
            }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!("0.000000000000000123 ETH", result.to_string());
    }

    /// Test `query_nonce` CLI command.
    /// Given normal conditions, when query nonce, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_query_nonce_then_ok() {
        // Build mocks
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

        // Mock the command line arguments,
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryNonce {
                    address: "0xc24215226336d22238a20a72f8e489c005b44c4a".to_string(),
                },
            }),
        };

        // When
        let result = runner::run(beerus, cli).await.unwrap();

        //Then
        assert_eq!("Nonce: 123", result.to_string());
    }

    /// Test `query_block_number` CLI command.
    /// Given normal conditions, when query block_number, then ok.
    /// Success case.
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

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryBlockNumber {},
            }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!("123", result.to_string());
    }

    /// Test the `query_balance` CLI command.
    /// Given ethereum lightclient returns an error, when query balance, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_balance_then_error_is_propagated()
    {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_balance()
            .return_once(move |_, _| Err(eyre::eyre!("ethereum_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryBalance {
                    address: "0xc24215226336d22238a20a72f8e489c005b44c4a".to_string(),
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

    /// Test the `query_nonce` CLI command.
    /// Given ethereum lightclient returns an error, when query nonce, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_nonce_then_error_is_propagated() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_nonce()
            .return_once(move |_, _| Err(eyre::eyre!("ethereum_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryNonce {
                    address: "0xc24215226336d22238a20a72f8e489c005b44c4a".to_string(),
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

    /// Test `query_block_number` CLI command.
    /// Given ethereum lightclient returns an error, when query block_number, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_block_number_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_block_number()
            .return_once(move || Err(eyre::eyre!("ethereum_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryBlockNumber {},
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

    /// Test the `query_chain_id` CLI command.
    /// Given normal conditions, when query chain_id, then ok.
    /// Success case.
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

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryChainId {},
            }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!("1", result.to_string());
    }

    /// Test the `query_code` CLI command.
    /// Given normal conditions, when query code, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_query_code_then_ok() {
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

        let address = "0xc24215226336d22238a20a72f8e489c005b44c4a".to_owned();
        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryCode { address },
            }),
        };

        let result = runner::run(beerus, cli).await.unwrap();

        assert_eq!("[0, 0, 0, 1]", result.to_string());
    }

    /// Test `query-code` CLI command.
    /// Given ethereum lightclient returns an error, when query code, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_code_then_error_is_propagated() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_code()
            .return_once(move |_, _| Err(eyre::eyre!("ethereum_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let address = "0xc24215226336d22238a20a72f8e489c005b44c4a".to_owned();

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryCode { address },
            }),
        };

        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("ethereum_lightclient_error", e.to_string()),
            Ok(_) => panic!("Expected error,got ok"),
        }
    }

    /// Test the `query_block_tx_count_by_number` CLI command.
    /// Given normal conditions, when `query_block_tx_count_by_number`, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_query_tx_count_by_block_number_then_ok() {
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

        let block: u64 = 1;
        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryBlockTxCountByNumber { block },
            }),
        };

        let result = runner::run(beerus, cli).await.unwrap();

        assert_eq!("120", result.to_string());
    }

    /// Test `query_block_tx_count_by_number` CLI command.
    /// Given ethereum lightclient returns an error, when `query_block_tx_count_by_number`, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_tx_count_by_block_number_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_block_transaction_count_by_number()
            .return_once(move |_| Err(eyre::eyre!("ethereum_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let block: u64 = 1;
        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryBlockTxCountByNumber { block },
            }),
        };

        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("ethereum_lightclient_error", e.to_string()),
            Ok(_) => panic!("Expected error,got ok"),
        }
    }

    /// Test the `query-tx-count` CLI command.
    /// Given normal conditions, when `query-tx-count`, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_query_tx_count_then_ok() {
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        let expected_result: u64 = 123;
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

        let address = "0xc24215226336d22238a20a72f8e489c005b44c4a".to_string();
        let block = "latest".to_string();

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryTxCount { address, block },
            }),
        };

        let result = runner::run(beerus, cli).await.unwrap();

        assert_eq!("123", result.to_string());
    }

    /// Test `query-tx-count` CLI command.
    /// Given ethereum lightclient returns an error, when `query-tx-count`, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_tx_count_then_error_is_propagated()
    {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_transaction_count()
            .return_once(move |_, _| Err(eyre::eyre!("ethereum_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let address = "0xc24215226336d22238a20a72f8e489c005b44c4a".to_string();
        let block = "latest".to_string();

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryTxCount { address, block },
            }),
        };

        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("ethereum_lightclient_error", e.to_string()),
            Ok(_) => panic!("Expected error,got ok"),
        }
    }

    /// Test the `query_block_tx_count_by_hash` CLI command.
    /// Given normal conditions, when `query_block_tx_count_by_hash`, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_query_tx_count_by_block_hash_then_ok() {
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

        let hash = "0xc24215226336d22238a20a72f8e489c005b44c4a".to_string();
        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryBlockTxCountByHash { hash },
            }),
        };

        let result = runner::run(beerus, cli).await.unwrap();

        assert_eq!("120", result.to_string());
    }

    /// Test `query_block_tx_count_by_hash` CLI command.
    /// Given ethereum lightclient returns an error, when `query_block_tx_count_by_hash`, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_tx_count_by_block_hash_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_block_transaction_count_by_hash()
            .return_once(move |_| Err(eyre::eyre!("ethereum_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let hash = "0xc24215226336d22238a20a72f8e489c005b44c4a".to_string();
        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryBlockTxCountByHash { hash },
            }),
        };

        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("ethereum_lightclient_error", e.to_string()),
            Ok(_) => panic!("Expected error,got ok"),
        }
    }

    /// Test the `query_transaction_by_hash` CLI command.
    /// Given normal conditions, when `query_transaction_by_hash`, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_query_transaction_by_hash_then_ok() {
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        let transaction = Transaction::default();
        let _transaction = transaction.clone();
        // Given
        // Mock dependencies
        ethereum_lightclient
            .expect_get_transaction_by_hash()
            .return_once(move |_| Ok(Some(_transaction)));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let hash = "0xc9bb964b3fe087354bc1c1904518acc2b9df7ebedcb89215e9f3b41f47b6c31d".to_string();

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryTxByHash { hash },
            }),
        };

        let result = runner::run(beerus, cli).await.unwrap();

        assert_eq!(result.to_string(), "Transaction Data: \"Some(Transaction { hash: 0x0000000000000000000000000000000000000000000000000000000000000000, nonce: 0, block_hash: None, block_number: None, transaction_index: None, from: 0x0000000000000000000000000000000000000000, to: None, value: 0, gas_price: None, gas: 0, input: Bytes(0x), v: 0, r: 0, s: 0, transaction_type: None, access_list: None, max_priority_fee_per_gas: None, max_fee_per_gas: None, chain_id: None, other: OtherFields { inner: {} } })\"");
    }

    /// Test `query_query_transaction_by_hash` CLI command.
    /// Given ethereum lightclient returns an error, when `query_transaction_by_hash`, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_transaction_by_hash_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_transaction_by_hash()
            .return_once(move |_| Err(eyre::eyre!("ethereum_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let hash = "0xc9bb964b3fe087354bc1c1904518acc2b9df7ebedcb89215e9f3b41f47b6c31d".to_string();
        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryTxByHash { hash },
            }),
        };

        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("ethereum_lightclient_error", e.to_string()),
            Ok(_) => panic!("Expected error,got ok"),
        }
    }

    /// Test the `query_get_gas_price` CLI command.
    /// Given normal conditions, when `query_get_gas_price`, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_query_get_gas_price_then_ok() {
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        let gas_price = U256::default();

        // Given
        // Mock dependencies
        ethereum_lightclient
            .expect_get_gas_price()
            .return_once(move || Ok(gas_price));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryGasPrice {},
            }),
        };

        let result = runner::run(beerus, cli).await.unwrap();

        assert_eq!(result.to_string(), "0");
    }

    /// Test `query_get_gas_price` CLI command.
    /// Given ethereum lightclient returns an error, when `query_get_gas_price`, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_get_gas_price_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_gas_price()
            .return_once(move || Err(eyre::eyre!("ethereum_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryGasPrice {},
            }),
        };

        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("ethereum_lightclient_error", e.to_string()),
            Ok(_) => panic!("Expected error,got ok"),
        }
    }

    /// Test the `query_estimate_gas` CLI command.
    /// Given normal conditions, when `query_estimate_gas`, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_query_estimate_gas_then_ok() {
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies
        ethereum_lightclient
            .expect_estimate_gas()
            .return_once(move |_| Ok(10));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let params = (r#"{"from":"0x0000000000000000000000000000000000000000","to":"0x0000000000000000000000000000000000000000","value":"10","data":"0x41"}"#).to_string();
        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryEstimateGas { params },
            }),
        };

        let result = runner::run(beerus, cli).await.unwrap();

        assert_eq!(result.to_string(), "10");
    }

    /// Test `query_estimate_gas` CLI command.
    /// Given ethereum lightclient returns an error, when `query_estimate_gas`, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_estimate_gas_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_estimate_gas()
            .return_once(move |_| Err(eyre::eyre!("ethereum_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let params = (r#"{"from":"0x0000000000000000000000000000000000000000","to":"0x0000000000000000000000000000000000000000","value":"10","data":"0x41"}"#).to_string();
        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryEstimateGas { params },
            }),
        };

        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("ethereum_lightclient_error", e.to_string()),
            Ok(_) => panic!("Expected error,got ok"),
        }
    }

    /// Test the `query_block_by_hash` CLI command.
    /// Given normal conditions, when query block by hash, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_query_block_by_hash_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Expected block return.
        let expected_block = ExecutionBlock {
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
        };

        let expected_block_json = serde_json::to_string(&expected_block).unwrap();

        ethereum_lightclient
            .expect_get_block_by_hash()
            .return_once(move |_, _| Ok(Some(expected_block)));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryBlockByHash {
                    hash: "0xc24215226336d22238a20a72f8e489c005b44c4a".to_string(),
                    full_tx: false,
                },
            }),
        };

        // When
        let result = runner::run(beerus, cli).await.unwrap();
        // Then
        assert_eq!(expected_block_json, result.to_string());
    }

    /// Test the `query_block_by_hash` CLI command.
    /// Given ethereum lightclient returns an error, when query block by hash, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_block_by_hash_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        ethereum_lightclient
            .expect_get_block_by_hash()
            .return_once(move |_, _| Err(eyre::eyre!("ethereum_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryBlockByHash {
                    hash: "0xc24215226336d22238a20a72f8e489c005b44c4a".to_string(),
                    full_tx: false,
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

    /// Given ethereum lightclient returns an error, when wrong address input, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_wrong_address_input_then_error_is_propagated(
    ) {
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

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryBalance {
                    // Testing wrong address input
                    address: "ABCDE".to_string(),
                },
            }),
        };
        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("Invalid input length", e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }
    }

    /// Test the `query_get_priority_fee` CLI command.
    /// Given normal conditions, when `query_get_priority_fee`, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_query_get_priority_fee_then_ok() {
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        let gas_price = U256::default();

        // Given
        // Mock dependencies
        ethereum_lightclient
            .expect_get_priority_fee()
            .return_once(move || Ok(gas_price));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryPriorityFee {},
            }),
        };

        let result = runner::run(beerus, cli).await.unwrap();

        assert_eq!(result.to_string(), "0");
    }

    /// Test `query_get_priority_fee` CLI command.
    /// Given ethereum lightclient returns an error, when `query_get_priority_fee`, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_get_priority_fee_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Mock dependencies.
        ethereum_lightclient
            .expect_get_priority_fee()
            .return_once(move || Err(eyre::eyre!("ethereum_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryPriorityFee {},
            }),
        };

        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("ethereum_lightclient_error", e.to_string()),
            Ok(_) => panic!("Expected error,got ok"),
        }
    }

    /// Test the `query_block_by_number` CLI command.
    /// Given normal conditions, when query block by number, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_query_block_by_number_then_ok() {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        // Expected block return.
        let expected_block = ExecutionBlock {
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
        };

        let expected_block_json = serde_json::to_string(&expected_block).unwrap();

        ethereum_lightclient
            .expect_get_block_by_number()
            .return_once(move |_, _| Ok(Some(expected_block)));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryBlockByNumber {
                    block: "1".to_string(),
                    full_tx: false,
                },
            }),
        };

        // When
        let result = runner::run(beerus, cli).await.unwrap();
        // Then
        assert_eq!(expected_block_json, result.to_string());
    }

    /// Test the `query_block_by_number` CLI command.
    /// Given ethereum lightclient returns an error, when query block by number, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_ethereum_lightclient_returns_error_when_query_block_by_number_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        // Given
        ethereum_lightclient
            .expect_get_block_by_number()
            .return_once(move |_, _| Err(eyre::eyre!("ethereum_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands {
                command: EthereumSubCommands::QueryBlockByNumber {
                    block: "1".to_string(),
                    full_tx: false,
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
    async fn given_ethereum_lightclient_returns_error_when_query_logs_then_error_is_propagated() {
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        let mut log = ethers::types::Log::default();
        let topic =
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".to_string();
        let data = ethers::types::Bytes::from_str(
            "0x0000000000000000000000000000000000000000000000000000000000016931",
        )
        .unwrap();
        let block_hash = "0x92ef607b2b14dc2e6bf866325a1e84c9129ef741a5c2bc169dc36ea282d9d060";
        log.topics = vec![ethers::types::TxHash::from_str(&topic).unwrap()];
        log.data = data.clone();
        log.block_hash = Some(ethers::types::H256::from_str(&block_hash).unwrap());
        let logs = vec![log];

        // Given
        // Mock dependencies
        ethereum_lightclient
            .expect_get_logs()
            .return_once(move |_, _, _, _, _| Ok(logs));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let params = EthereumSubCommands::QueryLogs {
            address: None,
            blockhash: Some(block_hash.to_string()),
            from_block: None,
            to_block: None,
            topics: Some(vec![topic]),
        };

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands { command: params }),
        };

        let result = runner::run(beerus, cli).await.unwrap();
        let expected = "[{\"address\":\"0x0000000000000000000000000000000000000000\",\"topics\":[\"0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef\"],\"data\":\"0x0000000000000000000000000000000000000000000000000000000000016931\",\"blockHash\":\"0x92ef607b2b14dc2e6bf866325a1e84c9129ef741a5c2bc169dc36ea282d9d060\"}]";
        assert_eq!(result.to_string(), expected);
    }

    /// Test the `query_logs` CLI command.
    /// Given normal conditions, when `query_logs`, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_query_logs_then_ok() {
        let (config, mut ethereum_lightclient, starknet_lightclient) = config_and_mocks();

        let mut log = ethers::types::Log::default();
        let topic =
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".to_string();
        let block_hash = "0x92ef607b2b14dc2e6bf866325a1e84c9129ef741a5c2bc169dc36ea282d9d060";
        log.topics = vec![ethers::types::TxHash::from_str(&topic).unwrap()];
        log.data = ethers::types::Bytes::from_str(
            "0x0000000000000000000000000000000000000000000000000000000000016931",
        )
        .unwrap();
        log.block_hash = Some(ethers::types::H256::from_str(&block_hash).unwrap());
        let logs = vec![log];

        // Given
        // Mock dependencies
        ethereum_lightclient
            .expect_get_logs()
            .return_once(move |_, _, _, _, _| Ok(logs));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let params = EthereumSubCommands::QueryLogs {
            address: None,
            blockhash: Some("0x1".to_string()),
            from_block: Some("0x1".to_string()),
            to_block: Some("0x1".to_string()),
            topics: Some(vec![topic]),
        };

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::Ethereum(EthereumCommands { command: params }),
        };

        let result = runner::run(beerus, cli).await.unwrap();
        let expected = "[{\"address\":\"0x0000000000000000000000000000000000000000\",\"topics\":[\"0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef\"],\"data\":\"0x0000000000000000000000000000000000000000000000000000000000016931\",\"blockHash\":\"0x92ef607b2b14dc2e6bf866325a1e84c9129ef741a5c2bc169dc36ea282d9d060\"}]";
        assert_eq!(result.to_string(), expected);
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

    /// Test the `get_class_hash` CLI command.
    /// Given normal conditions, when query get_class_hash, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_get_class_hash_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let expected_result = FieldElement::from_str("1234").unwrap();

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_class_hash_at()
            .return_once(move |_block_id, _contract_address| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryGetClassHash {
                    block_id_type: "number".to_string(),
                    block_id: "123".to_string(),
                    contract_address: "0x123".to_string(),
                },
            }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!(result.to_string(), "Class hash: 1234".to_string());
    }

    /// Test the `get_class_hash` CLI command.
    /// Given starknet lightclient returns an error, when query get_class_hash, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_get_class_hash_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient.expect_get_class_hash_at().return_once(
            move |_block_id, _contract_address| Err(eyre::eyre!("starknet_lightclient_error")),
        );

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryGetClassHash {
                    block_id_type: "number".to_string(),
                    block_id: "123".to_string(),
                    contract_address: "0x123".to_string(),
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

    /// Test the `get_class_at` CLI command.
    /// Given normal conditions, when query get_class_at, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_get_class_at_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given

        let (expected_result, expected_result_value) =
            beerus_core::starknet_helper::create_mock_contract_class();

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_class_at()
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
                command: StarkNetSubCommands::QueryGetClassAt {
                    block_id_type: "number".to_string(),
                    block_id: "123".to_string(),
                    contract_address: "0x123".to_string(),
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

    /// Test the `get_class_at` CLI command.
    /// Given starknet lightclient returns an error, when query get_class_at, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_get_class_at_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        // Set the expected return value for the StarkNet light client mock.

        starknet_lightclient.expect_get_class_at().return_once(
            move |_block_id, _contract_address| Err(eyre::eyre!("starknet_lightclient_error")),
        );

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryGetClassAt {
                    block_id_type: "number".to_string(),
                    block_id: "123".to_string(),
                    contract_address: "0x123".to_string(),
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

    /// Test the `get_block_transaction_count` CLI command.
    /// Given normal conditions, when query get_block_transaction_count, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_get_block_transaction_count_then_ok() {
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

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryGetBlockTransactionCount {
                    block_id_type: "number".to_string(),
                    block_id: "123".to_string(),
                },
            }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!("Block transaction count: 34", result.to_string());
    }

    /// Test the `get_block_transaction_count` CLI command.
    /// Given starknet lightclient returns an error, when query get_block_transaction_count, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_get_block_transaction_count_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_block_transaction_count()
            .return_once(move |_block_id| Err(eyre::eyre!("starknet_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryGetBlockTransactionCount {
                    block_id_type: "number".to_string(),
                    block_id: "123".to_string(),
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

    /// Test the `syncing` CLI command.
    /// Given normal conditions, when query syncing, then ok.
    /// Case: Nodo starknet is syncing.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_query_syncing_case_status_syncing_then_ok() {
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

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QuerySyncing {},
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

    /// Test the `syncing` CLI command.
    /// Given normal conditions, when query syncing, then ok.
    /// Case: Nodo starknet is not syncing.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_query_syncing_case_status_not_syncing_then_ok() {
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

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QuerySyncing {},
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

    /// Test the `syncing` CLI command.
    /// Given starknet lightclient returns an error, when query syncing, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_syncing_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_syncing()
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
                command: StarkNetSubCommands::QuerySyncing {},
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
    /// Test the `get_state_update` CLI command.
    /// Given normal conditions, when query get_state_update, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_get_state_update_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

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

        // Given
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_state_update()
            .return_once(move |_block_id| Ok(expected));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let params = StarkNetSubCommands::QueryGetStateUpdate {
            block_id: "latest".to_string(),
            block_id_type: "tag".to_string(),
        };

        // Mock the command line arguments.
        let cli = Cli {
            config: None,

            command: Commands::StarkNet(StarkNetCommands { command: params }),
        };
        // When
        let result = runner::run(beerus, cli).await;

        // Then
        assert!(result.is_ok());
    }

    /// Test the `get_state_update` CLI command.
    /// Given starknet lightclient returns an error, when query get_state_update, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_get_state_update_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_state_update()
            .return_once(move |_block_id| Err(eyre::eyre!("Error: Invalid Tag")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let params = StarkNetSubCommands::QueryGetStateUpdate {
            block_id: "nonvalid".to_string(),
            block_id_type: "tag".to_string(),
        };

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands { command: params }),
        };
        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("Invalid Tag", e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }
    }

    /// Test the `add_invoke_transaction` CLI command.
    /// Given normal conditions, when query add_invoke_transaction, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_add_invoke_transaction_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let expected_result = InvokeTransactionResult {
            transaction_hash: FieldElement::from_str("0x01").unwrap(),
        };
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_add_invoke_transaction()
            .return_once(move |_| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let params = StarkNetSubCommands::AddInvokeTransaction {
            max_fee: "0".to_string(),
            signature: vec![10.to_string()],
            nonce: "0".to_string(),
            contract_address: "0".to_string(),
            entry_point_selector: "0".to_string(),
            calldata: vec![10.to_string()],
        };

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands { command: params }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!("InvokeTransactionResult { transaction_hash: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 } }", result.to_string());
    }

    /// Test the `add_invoke_transaction` CLI command.
    /// Given starknet lightclient returns an error, when query add_invoke_transaction, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_add_invoke_transaction_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_add_invoke_transaction()
            .return_once(move |_| Err(eyre::eyre!("starknet_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::AddInvokeTransaction {
                    max_fee: "0".to_string(),
                    signature: vec![],
                    nonce: "0".to_string(),
                    contract_address: "0".to_string(),
                    entry_point_selector: "0".to_string(),
                    calldata: vec![],
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

    /// Test the `add_deploy_transaction` CLI command.
    /// Given normal conditions, when query add_deploy_transaction, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_add_deploy_transaction_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        let expected_result = DeployTransactionResult {
            transaction_hash: FieldElement::from_str("0x01").unwrap(),
            contract_address: FieldElement::from_str("0x01").unwrap(),
        };
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_add_deploy_transaction()
            .return_once(move |_| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let program = vec![];
        let constructor = vec![ContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];

        let external = vec![ContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];

        let l1_handler = vec![ContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];
        let entry_points_by_type = EntryPointsByType {
            constructor,
            external,
            l1_handler,
        };
        let abi = None;

        let contract_class: ContractClass = ContractClass {
            program,
            entry_points_by_type,
            abi,
        };

        let contract_class_string = serde_json::to_string(&contract_class).unwrap();

        println!("Contract Class {contract_class_string:?}");

        let params = StarkNetSubCommands::AddDeployTransaction {
            contract_class: contract_class_string,
            version: "10".to_string(),
            contract_address_salt: "0".to_string(),
            constructor_calldata: vec![10.to_string()],
        };

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands { command: params }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!("DeployTransactionResult { transaction_hash: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, contract_address: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 } }", result.to_string());
    }

    /// Test the `add_deploy_transaction` CLI command.
    /// Given starknet lightclient returns an error, when query add_deploy_transaction, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_add_deploy_transaction_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_add_deploy_transaction()
            .return_once(move |_| Err(eyre::eyre!("starknet_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let program = vec![];
        let constructor = vec![ContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];

        let external = vec![ContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];

        let l1_handler = vec![ContractEntryPoint {
            offset: 10,
            selector: FieldElement::from_str("0").unwrap(),
        }];
        let entry_points_by_type = EntryPointsByType {
            constructor,
            external,
            l1_handler,
        };
        let abi = None;

        let contract_class: ContractClass = ContractClass {
            program,
            entry_points_by_type,
            abi,
        };

        let contract_class_string = serde_json::to_string(&contract_class).unwrap();

        println!("Contract Class {contract_class_string:?}");

        let params = StarkNetSubCommands::AddDeployTransaction {
            contract_class: contract_class_string,
            version: "10".to_string(),
            contract_address_salt: "0".to_string(),
            constructor_calldata: vec![10.to_string()],
        };

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands { command: params }),
        };
        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(e) => assert_eq!("starknet_lightclient_error", e.to_string()),
            Ok(_) => panic!("Expected error, got ok"),
        }
    }

    /// Test the `get_block_with_txs` CLI command.
    /// Given normal conditions, when query get_block_with_txs, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_get_block_with_txs_then_ok() {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
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
        let expected_result_value = expected_result.clone();

        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_block_with_txs()
            .return_once(move |_block_id| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryBlockWithTxs {
                    block_id_type: "number".to_string(),
                    block_id: "123".to_string(),
                },
            }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!(
            result.to_string(),
            format!("Block hash: {expected_result_value:?}")
        );
    }

    /// Test the `get_block_with_txs` CLI command.
    /// Given starknet lightclient returns an error, when query get_block_with_txs, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_get_block_with_txs_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_block_with_txs()
            .return_once(move |_block_id| Err(eyre::eyre!("starknet_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryBlockWithTxs {
                    block_id_type: "number".to_string(),
                    block_id: "123".to_string(),
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

    /// Test the `get_transaction_by_block_id_and_index` CLI command.
    /// Given normal conditions, when query get_transaction_by_block_id_and_index, then ok.
    #[tokio::test]
    async fn given_normal_conditions_when_starknet_get_transaction_by_block_id_and_index_then_ok() {
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
        let expected_result_value = expected_result.clone();
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_transaction_by_block_id_and_index()
            .return_once(move |_block_id, _index| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryTransactionByBlockIdAndIndex {
                    block_id_type: "number".to_string(),
                    block_id: "123".to_string(),
                    index: "0".to_string(),
                },
            }),
        };
        // When
        let result = runner::run(beerus, cli).await.unwrap();

        // Then
        assert_eq!(
            format!("Transaction: {expected_result_value:?}"),
            result.to_string()
        );
    }

    /// Test the `get_transaction_by_block_id_and_index` CLI command.
    /// Given starknet lightclient returns an error, when query get_transaction_by_block_id_and_index, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_starknet_get_transaction_by_block_id_and_index_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        // Given
        // Set the expected return value for the StarkNet light client mock.
        starknet_lightclient
            .expect_get_transaction_by_block_id_and_index()
            .return_once(move |_block_id, _index| Err(eyre::eyre!("starknet_lightclient_error")));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryTransactionByBlockIdAndIndex {
                    block_id_type: "number".to_string(),
                    block_id: "123".to_string(),
                    index: "0".to_string(),
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

    /// Test the starknet `query_transaction_by_hash` CLI command.
    /// Given normal conditions, when `query_transaction_by_hash`, then ok.
    /// Success case.
    #[tokio::test]
    async fn given_normal_conditions_when_query_starknet_transaction_by_hash_then_ok() {
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();

        let felt = FieldElement::from_hex_be("0x1").unwrap();
        let transaction = InvokeTransactionV1 {
            transaction_hash: felt.clone(),
            max_fee: felt.clone(),
            signature: vec![felt.clone()],
            nonce: felt.clone(),
            sender_address: felt.clone(),
            calldata: vec![felt.clone()],
        };

        let expected_result = StarknetTransaction::Invoke(InvokeTransaction::V1(transaction));
        // let transaction = StarknetTransaction::Invoke(InvokeTransactionV0);
        // let _transaction = transaction.clone();
        // Given
        // Mock dependencies
        starknet_lightclient
            .expect_get_transaction_by_hash()
            .return_once(move |_| Ok(expected_result));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let hash = "0x06986c739c4ab040c13a609d9f171ac4480e970dd6fe318eaeff5da9617bb854".to_string();
        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryTransactionByHash { hash },
            }),
        };

        let result = runner::run(beerus, cli).await.unwrap();
        assert_eq!(result.to_string(), "{\"calldata\":[\"0x1\"],\"max_fee\":\"0x1\",\"nonce\":\"0x1\",\"sender_address\":\"0x1\",\"signature\":[\"0x1\"],\"transaction_hash\":\"0x1\",\"type\":\"INVOKE\",\"version\":\"0x1\"}")
    }

    /// Test the starknet `query_transaction_by_hash` CLI command.
    /// Given starknet lightclient returns an error, when `query_transaction_by_hash`, then the error is propagated.
    /// Error case.
    #[tokio::test]
    async fn given_starknet_lightclient_returns_error_when_query_transaction_by_hash_then_error_is_propagated(
    ) {
        // Build mocks.
        let (config, ethereum_lightclient, mut starknet_lightclient) = config_and_mocks();
        let err_msg = r#"Error: JSON-RPC error: code=25, message="Transaction hash not found""#;
        // Given
        // Mock dependencies.
        starknet_lightclient
            .expect_get_transaction_by_hash()
            .return_once(move |_| Err(eyre::eyre!(err_msg.clone())));

        let beerus = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        let hash = "0x06".to_string();
        // Mock the command line arguments.
        let cli = Cli {
            config: None,
            command: Commands::StarkNet(StarkNetCommands {
                command: StarkNetSubCommands::QueryTransactionByHash { hash },
            }),
        };

        // When
        let result = runner::run(beerus, cli).await;

        // Then
        match result {
            Err(msg) => assert_eq!(msg.to_string(), err_msg),
            Ok(_) => panic!("Expected an err but got an Ok"),
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
