#[cfg(test)]
mod test {
    use beerus_core::{
        config::Config,
        lightclient::{
            beerus::BeerusLightClient, ethereum::MockEthereumLightClient,
            starknet::StarkNetLightClientImpl,
        },
    };
    use ethers::types::Address;
    use eyre::eyre;
    use httpmock::{prelude::*, Mock};
    use serde_json::json;
    use starknet::core::types::FieldElement;
    use std::str::FromStr;

    #[tokio::test]
    async fn given_normal_conditions_when_starknet_get_storage_at_should_work() {
        // Start a lightweight mock server.
        let server = MockServer::start();
        let mock_request = get_storage_at_mock(&server);
        let config = mock_config(&server);

        let starknet_lightclient = Box::new(StarkNetLightClientImpl::new(&config).unwrap());
        let mut helios_lightclient = MockEthereumLightClient::new();
        helios_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Ok(vec![1]));
        let beerus =
            BeerusLightClient::new(config, Box::new(helios_lightclient), starknet_lightclient);
        let storage_var = beerus
            .starknet_get_storage_at(
                FieldElement::from_str("0x00").unwrap(),
                FieldElement::from_str("0x00").unwrap(),
            )
            .await
            .unwrap();

        mock_request.assert();
        assert_eq!(storage_var, FieldElement::from_str("0x01").unwrap());
    }

    #[tokio::test]
    async fn given_ethereum_light_client_returns_error_when_starknet_get_storage_at_should_fail() {
        let server = MockServer::start();
        let mock_request = get_storage_at_mock(&server);
        let config = mock_config(&server);

        let starknet_lightclient = Box::new(StarkNetLightClientImpl::new(&config).unwrap());
        let mut helios_lightclient = MockEthereumLightClient::new();
        let expected_error = "Ethereum light client error";

        // Mock the `start` method of the Ethereum light client.
        helios_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Err(eyre!(expected_error)));
        let beerus =
            BeerusLightClient::new(config, Box::new(helios_lightclient), starknet_lightclient);
        let res = beerus
            .starknet_get_storage_at(
                FieldElement::from_str("0x00").unwrap(),
                FieldElement::from_str("0x00").unwrap(),
            )
            .await;
        assert_eq!(mock_request.hits(), 0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), expected_error.to_string());
    }

    #[tokio::test]
    async fn given_normal_conditions_when_starknet_call_should_work() {
        // Start a lightweight mock server.
        let server = MockServer::start();
        let mock_request = call_mock(&server);
        let config = mock_config(&server);

        let starknet_lightclient = Box::new(StarkNetLightClientImpl::new(&config).unwrap());
        let mut helios_lightclient = MockEthereumLightClient::new();
        helios_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Ok(vec![1]));
        let beerus =
            BeerusLightClient::new(config, Box::new(helios_lightclient), starknet_lightclient);
        let storage_var = beerus
            .starknet_call_contract(
                FieldElement::from_str("0x00").unwrap(),
                FieldElement::from_str("0x00").unwrap(),
                vec![],
            )
            .await
            .unwrap();

        mock_request.assert();
        assert_eq!(storage_var, vec![FieldElement::from_str("0x01").unwrap()]);
    }

    #[tokio::test]
    async fn given_ethereum_light_client_returns_error_when_starknet_starknet_call_should_fail() {
        let server = MockServer::start();
        let mock_request = get_storage_at_mock(&server);
        let config = mock_config(&server);

        let starknet_lightclient = Box::new(StarkNetLightClientImpl::new(&config).unwrap());
        let mut helios_lightclient = MockEthereumLightClient::new();
        let expected_error = "Ethereum light client error";

        // Mock the `start` method of the Ethereum light client.
        helios_lightclient
            .expect_call()
            .times(1)
            .return_once(move |_req, _block_nb| Err(eyre!(expected_error)));
        let beerus =
            BeerusLightClient::new(config, Box::new(helios_lightclient), starknet_lightclient);
        let res = beerus
            .starknet_call_contract(
                FieldElement::from_str("0x00").unwrap(),
                FieldElement::from_str("0x00").unwrap(),
                vec![],
            )
            .await;
        assert_eq!(mock_request.hits(), 0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), expected_error.to_string());
    }

    fn get_storage_at_mock(server: &MockServer) -> Mock {
        server.mock(|when, then| {
            when.method(POST).path("/").json_body(json!({
                "id":1,
                "jsonrpc":"2.0",
                "method":"starknet_getStorageAt",
                "params":[
                    "0x0",
                    "0x0",
                    {
                        "block_number":1
                    }
                ]
            }));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": "0x0000000000000000000000000000000000000000000000000000000000000001"
                }));
        })
    }
    fn call_mock(server: &MockServer) -> Mock {
        server.mock(|when, then| {
            when.method(POST).path("/").json_body(json!({
                "id":1,
                "jsonrpc":"2.0",
                "method":"starknet_call",
                "params":[
                    {
                        "calldata":[

                        ],
                        "contract_address":"0x0",
                        "entry_point_selector":"0x0"
                    },
                    {
                        "block_number":1
                    }
                ]
            }));
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": ["0x0000000000000000000000000000000000000000000000000000000000000001"]
                }));
        })
    }
    fn mock_config(server: &MockServer) -> Config {
        Config {
            ethereum_network: "mainnet".to_string(),
            ethereum_consensus_rpc: server.base_url(),
            ethereum_execution_rpc: server.base_url(),
            starknet_rpc: server.base_url(),
            starknet_core_contract_address: Address::from_str(
                "0x0000000000000000000000000000000000000000",
            )
            .unwrap(),
        }
    }
}
