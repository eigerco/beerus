pub mod api;

use crate::api::{ethereum, starknet};
use beerus_core::lightclient::beerus::BeerusLightClient;
use rocket::{Build, Rocket};
#[macro_use]
extern crate rocket;

pub async fn build_rocket_server(beerus: BeerusLightClient) -> Rocket<Build> {
    env_logger::init();
    // Create the Rocket instance.
    rocket::build().manage(beerus).mount(
        "/",
        routes![
            index,
            ethereum::endpoints::query_balance,
            starknet::endpoints::query_starknet_state_root
        ],
    )
}

#[get("/")]
pub fn index() -> &'static str {
    "Hakai!"
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::rocket;
    use crate::build_rocket_server;
    use beerus_core::{
        config::Config,
        lightclient::{
            beerus::BeerusLightClient, ethereum::MockEthereumLightClient,
            starknet::MockStarkNetLightClient,
        },
    };
    use ethers::types::Address;
    use rocket::{http::Status, local::asynchronous::Client};
    /// Test the `query_balance` endpoint.
    /// `/ethereum/balance/<address>`
    #[tokio::test]
    // For now we ignore this test because it requires to mock the Beerus light client.
    #[ignore]
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
