#[cfg(test)]
mod tests {
    use beerus_core::config::{
        Config, DEFAULT_DATA_DIR, DEFAULT_ETHEREUM_NETWORK, DEFAULT_POLL_INTERVAL_SECS,
        STARKNET_GOERLI_CC_ADDRESS,
    };
    use ethers::types::Address;
    use serial_test::serial;
    use std::env;
    use std::{path::PathBuf, str::FromStr};


    /// Test `ethereum_network` function.
    /// It should return the correct value.
    #[test]
    fn mainnet_file_config_returns_correct_values() {
        let mainnet_file_config: Config =
            Config::from_file(&PathBuf::from("tests/data/mainnet.toml"));

        assert_eq!(mainnet_file_config.ethereum_network, "mainnet");
        assert_eq!(
            mainnet_file_config.ethereum_consensus_rpc,
            "https://www.lightclientdata.org"
        );
        assert_eq!(
            mainnet_file_config.ethereum_execution_rpc,
            "https://eth-mainnet.g.alchemy.com/v2/XXXXX"
        );
        assert_eq!(mainnet_file_config.poll_interval_secs, Some(10));
    }

    /// Test `ethereum_network` function.
    /// It should return the correct value.
    #[test]
    fn goerli_file_config_returns_correct_values() {
        let goerli_file_config: Config =
            Config::from_file(&PathBuf::from("tests/data/goerli.toml"));

        assert_eq!(
            goerli_file_config.ethereum_network,
            DEFAULT_ETHEREUM_NETWORK
        );
        assert_eq!(
            goerli_file_config.ethereum_consensus_rpc,
            "http://testing.prater.beacon-api.nimbus.team"
        );
        assert_eq!(
            goerli_file_config.ethereum_execution_rpc,
            "https://eth-goerli.g.alchemy.com/v2/XXXXX"
        );
        assert_eq!(
            goerli_file_config.poll_interval_secs,
            Some(DEFAULT_POLL_INTERVAL_SECS)
        );
    }

    /// Test `default` function.
    /// It should return the correct value.
    #[test]
    fn default_returns_correct_values() {
        let conf = Config::default();
        assert_eq!(conf.ethereum_network, DEFAULT_ETHEREUM_NETWORK);
        assert_eq!(conf.ethereum_consensus_rpc, "http://localhost:8545");
        assert_eq!(conf.ethereum_execution_rpc, "http://localhost:5054");
        assert_eq!(conf.starknet_rpc, "http://localhost:9545");
        assert_eq!(
            conf.starknet_core_contract_address,
            Address::from_str(STARKNET_GOERLI_CC_ADDRESS).unwrap()
        );
        assert_eq!(conf.data_dir, PathBuf::from(DEFAULT_DATA_DIR));
        assert_eq!(conf.poll_interval_secs, Some(DEFAULT_POLL_INTERVAL_SECS));
    }


    /// Test `from_env` function.
    #[test]
    #[serial]
    fn all_envs_set_returns_config() {
        Config::clean_env();
        env::set_var("ETHEREUM_NETWORK", "mainnet");
        env::set_var("ETHEREUM_CONSENSUS_RPC_URL", "http://localhost:8545");
        env::set_var("ETHEREUM_EXECUTION_RPC_URL", "http://localhost:8545");
        env::set_var("STARKNET_RPC_URL", "http://localhost:8545");
        env::set_var("DATA_DIR", "/tmp");

        let cfg = Config::from_env();
        assert_eq!(cfg.ethereum_network, "mainnet");
        assert_eq!(cfg.ethereum_consensus_rpc, "http://localhost:8545");
        assert_eq!(cfg.ethereum_execution_rpc, "http://localhost:8545");
        assert_eq!(cfg.starknet_rpc, "http://localhost:8545");
    }

    /// Test `from_env` function when `ETHEREUM_NETWORK` is not set.
    /// It should use the default value.
    /// The default value is `goerli`.
    /// The default value is defined in `DEFAULT_ETHEREUM_NETWORK` constant.
    #[test]
    #[serial]
    fn ethereum_network_env_not_set_returns_config() {
        Config::clean_env();
        env::set_var("ETHEREUM_CONSENSUS_RPC_URL", "http://localhost:8545");
        env::set_var("ETHEREUM_EXECUTION_RPC_URL", "http://localhost:8545");
        env::set_var("STARKNET_RPC_URL", "http://localhost:8545");
        env::set_var("DATA_DIR", "/tmp");

        let cfg = Config::from_env();
        assert_eq!(cfg.ethereum_network, "goerli");
    }

    /// Test `from_env` function when `ETHEREUM_CONSENSUS_RPC_URL` is not set.
    /// It should return an error.
    #[test]
    #[serial]
    #[should_panic]
    fn ethereum_consensus_env_not_set_panics() {
        Config::clean_env();
        env::set_var("ETHEREUM_NETWORK", "mainnet");
        env::set_var("ETHEREUM_EXECUTION_RPC_URL", "http://localhost:8545");
        env::set_var("STARKNET_RPC_URL", "http://localhost:8545");

        let _cfg = Config::from_env();
    }

    /// Test `from_env` function when `ETHEREUM_EXECUTION_RPC_URL` is not set.
    /// It should return an error.
    #[test]
    #[serial]
    #[should_panic]
    fn ethereum_execution_env_not_set_panics() {
        Config::clean_env();
        env::set_var("ETHEREUM_NETWORK", "mainnet");
        env::set_var("ETHEREUM_CONSENSUS_RPC_URL", "http://localhost:8545");
        env::set_var("STARKNET_RPC_URL", "http://localhost:8545");

        let _cfg = Config::from_env();
    }

    /// Test `from_env` function when `STARKNET_RPC_URL` is not set.
    /// It should return an error.
    #[test]
    #[serial]
    #[should_panic]
    fn starknet_env_not_set_panics() {
        Config::clean_env();
        env::set_var("ETHEREUM_NETWORK", "mainnet");
        env::set_var("ETHEREUM_CONSENSUS_RPC_URL", "http://localhost:8545");
        env::set_var("ETHEREUM_EXECUTION_RPC_URL", "http://localhost:8545");

        let _cfg = Config::from_env();
    }

    /// Env Var `BEERUS_CONFIG`
    /// Should override the config parsing to the file
    #[test]
    #[serial]
    fn beerus_config_env_var_should_override() {
        env::set_var("BEERUS_CONFIG", "tests/data/goerli.toml");
        let cfg = Config::from_env();

        assert_eq!(
            cfg.ethereum_consensus_rpc,
            "http://testing.prater.beacon-api.nimbus.team"
        );
        assert_eq!(
            cfg.ethereum_execution_rpc,
            "https://eth-goerli.g.alchemy.com/v2/XXXXX"
        );
    }
}
