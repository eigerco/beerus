use beerus_core::{
    config::Config,
    lightclient::{
        beerus::BeerusLightClient, ethereum::MockEthereumLightClient,
        starknet::StarkNetLightClientImpl,
    },
};
use beerus_rpc::{server::BeerusRpc, utils::wiremock::setup_wiremock};

pub async fn setup_beerus_rpc() -> BeerusRpc {
    let mock_starknet_rpc = setup_wiremock().await;
    set_mandatory_envs(mock_starknet_rpc);
    let config = Config::default();

    let ethereum_lightclient = MockEthereumLightClient::new();
    let starknet_lightclient = StarkNetLightClientImpl::new(&config).unwrap();

    let beerus_client = BeerusLightClient::new(
        config,
        Box::new(ethereum_lightclient),
        Box::new(starknet_lightclient),
    );
    BeerusRpc::new(beerus_client)
}

fn set_mandatory_envs(starknet_rpc: String) {
    std::env::set_var("ETHEREUM_CONSENSUS_RPC_URL", "");
    std::env::set_var("ETHEREUM_EXECUTION_RPC_URL", "");
    std::env::set_var("STARKNET_RPC_URL", starknet_rpc);
}
