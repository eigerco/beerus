#[cfg(test)]
mod tests {
    use beerus_core::config::Config;
    use ethers::types::Address;
    use helios::config::networks::Network;
    use std::{path::PathBuf, str::FromStr};

    /// Test `new_from_env` function.
    #[test]
    fn given_normal_conditions_when_new_from_env_then_returns_config() {
        temp_env::with_vars(
            vec![
                ("ETHEREUM_NETWORK", Some("mainnet")),
                ("ETHEREUM_CONSENSUS_RPC_URL", Some("http://localhost:8545")),
                ("ETHEREUM_EXECUTION_RPC_URL", Some("http://localhost:8545")),
                ("STARKNET_RPC_URL", Some("http://localhost:8545")),
            ],
            || {
                let result = Config::new_from_env();
                assert!(result.is_ok());
                let config = result.unwrap();
                assert_eq!(config.ethereum_network, "mainnet");
                assert_eq!(config.ethereum_consensus_rpc, "http://localhost:8545");
                assert_eq!(config.ethereum_execution_rpc, "http://localhost:8545");
                assert_eq!(config.starknet_rpc, "http://localhost:8545");
            },
        );
    }

    /// Test `new_from_env` function when `ETHEREUM_NETWORK` is not set.
    /// It should use the default value.
    /// The default value is `goerli`.
    /// The default value is defined in `DEFAULT_ETHEREUM_NETWORK` constant.
    #[test]
    fn given_ethereum_network_is_not_set_when_new_from_env_then_returns_config() {
        temp_env::with_vars(
            vec![
                ("ETHEREUM_NETWORK", None),
                ("ETHEREUM_CONSENSUS_RPC_URL", Some("http://localhost:8545")),
                ("ETHEREUM_EXECUTION_RPC_URL", Some("http://localhost:8545")),
                ("STARKNET_RPC_URL", Some("http://localhost:8545")),
            ],
            || {
                let result = Config::new_from_env();
                assert!(result.is_ok());
                let config = result.unwrap();
                assert_eq!(config.ethereum_network, "goerli");
            },
        );
    }

    /// Test `new_from_env` function when `ETHEREUM_CONSENSUS_RPC_URL` is not set.
    /// It should return an error.
    #[test]
    fn given_ethereum_consensus_rpc_is_not_set_when_new_from_env_then_returns_error() {
        temp_env::with_vars(
            vec![
                ("ETHEREUM_NETWORK", Some("mainnet")),
                ("ETHEREUM_CONSENSUS_RPC_URL", None),
                ("ETHEREUM_EXECUTION_RPC_URL", Some("http://localhost:8545")),
                ("STARKNET_RPC_URL", Some("http://localhost:8545")),
            ],
            || {
                let result = Config::new_from_env();
                match result {
                    Ok(_) => panic!("Should return an error"),
                    Err(err) => {
                        assert_eq!(
                            err.to_string(),
                            "Missing mandatory environment variable: ETHEREUM_CONSENSUS_RPC_URL"
                        )
                    }
                }
            },
        );
    }

    /// Test `new_from_env` function when `ETHEREUM_EXECUTION_RPC_URL` is not set.
    /// It should return an error.
    #[test]
    fn given_ethereum_execution_rpc_is_not_set_when_new_from_env_then_returns_error() {
        temp_env::with_vars(
            vec![
                ("ETHEREUM_NETWORK", Some("mainnet")),
                ("ETHEREUM_CONSENSUS_RPC_URL", Some("http://localhost:8545")),
                ("ETHEREUM_EXECUTION_RPC_URL", None),
                ("STARKNET_RPC_URL", Some("http://localhost:8545")),
            ],
            || {
                let result = Config::new_from_env();
                match result {
                    Ok(_) => panic!("Should return an error"),
                    Err(err) => {
                        assert_eq!(
                            err.to_string(),
                            "Missing mandatory environment variable: ETHEREUM_EXECUTION_RPC_URL"
                        )
                    }
                }
            },
        );
    }

    /// Test `new_from_env` function when `STARKNET_RPC_URL` is not set.
    /// It should return an error.
    #[test]
    fn given_starknet_rpc_is_not_set_when_new_from_env_then_returns_error() {
        temp_env::with_vars(
            vec![
                ("ETHEREUM_NETWORK", Some("mainnet")),
                ("ETHEREUM_CONSENSUS_RPC_URL", Some("http://localhost:8545")),
                ("ETHEREUM_EXECUTION_RPC_URL", Some("http://localhost:8545")),
                ("STARKNET_RPC_URL", None),
            ],
            || {
                let result = Config::new_from_env();
                match result {
                    Ok(_) => panic!("Should return an error"),
                    Err(err) => {
                        assert_eq!(
                            err.to_string(),
                            "Missing mandatory environment variable: STARKNET_RPC_URL"
                        )
                    }
                }
            },
        );
    }

    /// Test `ethereum_network` function.
    /// It should return the correct value.
    #[test]
    fn given_mainnet_when_ethereum_network_then_returns_correct_value() {
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
        match config.ethereum_network().unwrap() {
            Network::MAINNET => {}
            _ => panic!("Should return Mainnet"),
        }
    }

    /// Test `ethereum_network` function.
    /// It should return the correct value.
    #[test]
    fn given_goerli_when_ethereum_network_then_returns_correct_value() {
        let config = Config {
            ethereum_network: "goerli".to_string(),
            ethereum_consensus_rpc: "http://localhost:8545".to_string(),
            ethereum_execution_rpc: "http://localhost:8545".to_string(),
            starknet_rpc: "http://localhost:8545".to_string(),
            data_dir: Some(PathBuf::from("/tmp")),
            starknet_core_contract_address: Address::from_str(
                "0x0000000000000000000000000000000000000000",
            )
            .unwrap(),
        };
        match config.ethereum_network().unwrap() {
            Network::GOERLI => {}
            _ => panic!("Should return Goerli"),
        }
    }

    /// Test `ethereum_network` function when invalid network value.
    /// It should return an error.
    #[test]
    fn given_ethereum_network_is_invalid_when_ethereum_network_then_returns_error() {
        let config = Config {
            ethereum_network: "invalid".to_string(),
            ethereum_consensus_rpc: "http://localhost:8545".to_string(),
            ethereum_execution_rpc: "http://localhost:8545".to_string(),
            starknet_rpc: "http://localhost:8545".to_string(),
            data_dir: Some(PathBuf::from("/tmp")),
            starknet_core_contract_address: Address::from_str(
                "0x0000000000000000000000000000000000000000",
            )
            .unwrap(),
        };
        match config.ethereum_network() {
            Ok(_) => panic!("Should return an error"),
            Err(err) => {
                assert_eq!(err.to_string(), "Invalid network")
            }
        }
    }

    /// Test `default` function.
    /// It should return the correct value.
    #[test]
    fn given_default_when_default_then_returns_correct_value() {
        temp_env::with_vars(
            vec![
                ("ETHEREUM_NETWORK", Some("mainnet")),
                ("ETHEREUM_CONSENSUS_RPC_URL", Some("http://localhost:8545")),
                ("ETHEREUM_EXECUTION_RPC_URL", Some("http://localhost:8545")),
                ("STARKNET_RPC_URL", Some("http://localhost:8545")),
            ],
            || {
                let config = Config::default();
                assert_eq!(config.ethereum_network, "mainnet");
                assert_eq!(config.ethereum_consensus_rpc, "http://localhost:8545");
                assert_eq!(config.ethereum_execution_rpc, "http://localhost:8545");
                assert_eq!(config.starknet_rpc, "http://localhost:8545");
            },
        );
    }
}
