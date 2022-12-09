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
    use ethers::types::Address;
    use primitive_types::U256;
    use starknet::core::types::FieldElement;

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

    /// Test the `query_contract` CLI command.
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
