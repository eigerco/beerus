use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::MockEthereumLightClient,
        starknet::StarkNetLightClientImpl,
    },
};
use beerus_rpc::{beerus_rpc_server::BeerusRpc, utils::wiremock::setup_wiremock};
use std::path::PathBuf;

pub const DEFAULT_DATA_DIR: &str = "/tmp";

pub async fn setup_beerus_rpc() -> BeerusRpc {
    let mock_starknet_rpc = setup_wiremock().await;
    let config = Config {
        ethereum_network: "goerli".to_string(),
        ethereum_consensus_rpc: "".to_string(),
        ethereum_execution_rpc: "".to_string(),
        starknet_rpc: mock_starknet_rpc,
        starknet_core_contract_address: Default::default(),
        data_dir: Some(PathBuf::from(DEFAULT_DATA_DIR)),
    };

    let ethereum_lightclient = MockEthereumLightClient::new();
    let starknet_lightclient = StarkNetLightClientImpl::new(&config).unwrap();

    let beerus = BeerusLightClient::new(
        config,
        Box::new(ethereum_lightclient),
        Box::new(starknet_lightclient),
    );
    BeerusRpc::new(beerus)
}
