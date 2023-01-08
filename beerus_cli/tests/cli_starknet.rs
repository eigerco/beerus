#[cfg(test)]
mod test {
    use std::str::FromStr;

    use beerus_cli::{
        model::{Cli, Commands, EthereumCommands, EthereumSubCommands},
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
