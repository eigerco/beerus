//triggering CI
#![cfg(not(target_arch = "wasm32"))]

#[cfg(test)]
mod tests {
    use beerus_core::config::{
        Config, DEFAULT_DATA_DIR, DEFAULT_ETHEREUM_NETWORK, DEFAULT_POLL_INTERVAL_SECS,
        STARKNET_GOERLI_CC_ADDRESS,
    };
    use ethers::types::Address;
    use serial_test::serial;
    use shellexpand;
    use std::env;
    use std::{path::PathBuf, str::FromStr};

    /// Test `ethereum_network` function.
    /// It should return the correct value.
    #[test]
    fn mainnet_file_config_returns_correct_values() {
        let mainnet_file_config: Config =
            Config::from_file(&PathBuf::from("tests/common/data/mainnet.toml"));

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
            Config::from_file(&PathBuf::from("tests/common/data/goerli.toml"));

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

    /////////////////////////////////my addition//////////////////////////
    /// Test `from_file` function with a bad config file.
    /// It should fail.
    #[test]
    #[should_panic]
    fn bad_config_file_panics() {
        let goerli_file_config: Config =
            Config::from_file(&PathBuf::from("tests/common/data/bad.toml"));
    }

    /// Test `from_file` function with missing config file.
    /// It should fail.
    #[test]
    #[should_panic]
    fn missing_config_file_panics() {
        let missing_file_config: Config =
            Config::from_file(&PathBuf::from("tests/file/that/doesnt/exist.toml"));
    }

    /////////////////////////////end//////////////////////////////////////

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

        assert_eq!(
            conf.data_dir,
            PathBuf::from(shellexpand::tilde(DEFAULT_DATA_DIR).to_string())
        );

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

    /////////////////////////////my addition////////////////
    /// Test `from_env` function with "goerli" set as ETHEREUM_NETWORK
    #[test]
    #[serial]
    fn ethereum_network_env_goerli_setting_returns_config() {
        Config::clean_env();
        env::set_var("ETHEREUM_NETWORK", "goerli");
        env::set_var("ETHEREUM_CONSENSUS_RPC_URL", "http://localhost:8545");
        env::set_var("ETHEREUM_EXECUTION_RPC_URL", "http://localhost:8545");
        env::set_var("STARKNET_RPC_URL", "http://localhost:8545");
        env::set_var("DATA_DIR", "/tmp");

        let cfg = Config::from_env();
        assert_eq!(cfg.ethereum_network, "goerli");
        assert_eq!(cfg.ethereum_consensus_rpc, "http://localhost:8545");
        assert_eq!(cfg.ethereum_execution_rpc, "http://localhost:8545");
        assert_eq!(cfg.starknet_rpc, "http://localhost:8545");
    }

    /////////////////END//////////////////////////////////////////

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

    ///Test `from_env` with unacceptable ethereum_network
    ///It should panic
    #[test]
    #[serial]
    #[should_panic]
    fn ethereum_network_env_erroneous_setting_panics() {
        Config::clean_env();
        env::set_var("ETHEREUM_NETWORK", "sepolia");
        env::set_var("ETHEREUM_EXECUTION_RPC_URL", "http://localhost:8545");
        env::set_var("ETHEREUM_CONSENSUS_RPC_URL", "http://localhost:8545");
        env::set_var("STARKNET_RPC_URL", "http://localhost:8545");
        env::set_var("DATA_DIR", "/tmp");

        let _cfg = Config::from_env();
    }

    /// Env Var `BEERUS_CONFIG`
    /// Should override the config parsing to the file
    #[test]
    #[serial]
    fn beerus_config_env_var_should_override() {
        env::set_var("BEERUS_CONFIG", "tests/common/data/goerli.toml");
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

    /// Test ethereum custom checkpoint allowed valued.
    /// It should pass accepting 'clear' and explicit endpoint to sync.
    #[test]
    #[serial]
    fn ethereum_checkpoint_clear() {
        Config::clean_env();
        env::set_var("ETHEREUM_NETWORK", "mainnet");
        env::set_var("ETHEREUM_CONSENSUS_RPC_URL", "http://localhost:8545");
        env::set_var("ETHEREUM_EXECUTION_RPC_URL", "http://localhost:8545");
        env::set_var("STARKNET_RPC_URL", "http://localhost:8545");
        env::set_var("ETHEREUM_CHECKPOINT", "clear");

        let _cfg = Config::from_env();

        env::set_var(
            "ETHEREUM_CHECKPOINT",
            "0x85e6151a246e8fdba36db27a0c7678a575346272fe978c9281e13a8b26cdfa68",
        );

        let _cfg = Config::from_env();
    }

    /// Test ethereum custom checkpoint with unexpected random string.
    /// It should panic.
    #[test]
    #[serial]
    #[should_panic]
    fn ethereum_checkpoint_bad_string() {
        Config::clean_env();
        env::set_var("ETHEREUM_NETWORK", "mainnet");
        env::set_var("ETHEREUM_CONSENSUS_RPC_URL", "http://localhost:8545");
        env::set_var("ETHEREUM_EXECUTION_RPC_URL", "http://localhost:8545");
        env::set_var("STARKNET_RPC_URL", "http://localhost:8545");
        env::set_var("ETHEREUM_CHECKPOINT", "somestring");

        let _cfg = Config::from_env();
    }

    /// Test ethereum custom checkpoint with invalid hex string.
    /// It should panic.
    #[test]
    #[serial]
    #[should_panic]
    fn ethereum_checkpoint_invalid_hex() {
        Config::clean_env();
        env::set_var("ETHEREUM_NETWORK", "mainnet");
        env::set_var("ETHEREUM_CONSENSUS_RPC_URL", "http://localhost:8545");
        env::set_var("ETHEREUM_EXECUTION_RPC_URL", "http://localhost:8545");
        env::set_var("STARKNET_RPC_URL", "http://localhost:8545");
        env::set_var("ETHEREUM_CHECKPOINT", "0x1234poepk");

        let _cfg = Config::from_env();
    }

    /// Test ethereum custom checkpoint with missing '0x' prefix.
    /// It should panic.
    #[test]
    #[serial]
    #[should_panic]
    fn ethereum_checkpoint_missing_prefix() {
        Config::clean_env();
        env::set_var("ETHEREUM_NETWORK", "mainnet");
        env::set_var("ETHEREUM_CONSENSUS_RPC_URL", "http://localhost:8545");
        env::set_var("ETHEREUM_EXECUTION_RPC_URL", "http://localhost:8545");
        env::set_var("STARKNET_RPC_URL", "http://localhost:8545");
        env::set_var(
            "ETHEREUM_CHECKPOINT",
            "85e6151a246e8fdba36db27a0c7678a575346272fe978c9281e13a8b26cdfa68",
        );

        let _cfg = Config::from_env();
    }
}
