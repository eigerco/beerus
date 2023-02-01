#[cfg(test)]
mod test {
    use beerus_core::{
        config::Config,
        lightclient::{
            beerus::BeerusLightClient,
            ethereum::MockEthereumLightClient,
            starknet::{storage_proof::GetProofOutput, StarkNetLightClientImpl},
        },
    };
    use ethers::types::Address;
    use eyre::eyre;
    use httpmock::{prelude::*, Mock};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use starknet::{core::types::FieldElement, providers::jsonrpc::models::BlockId};
    use std::fs;
    use std::path::PathBuf;
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

    #[tokio::test]
    async fn given_normal_conditions_when_starknet_get_storage_proof_should_work() {
        // Start a lightweight mock server.
        let server = MockServer::start();
        let config = mock_config(&server);

        let starknet_lightclient = Box::new(StarkNetLightClientImpl::new(&config).unwrap());
        let helios_lightclient = MockEthereumLightClient::new();

        let beerus =
            BeerusLightClient::new(config, Box::new(helios_lightclient), starknet_lightclient);

        let (mock, expected_proof) = get_contract_storage_proof_mock(&server);

        let keys = [FieldElement::ONE];
        let contract_address = FieldElement::from_hex_be(
            "0x4d4e07157aeb54abeb64f5792145f2e8db1c83bda01a8f06e050be18cfb8153",
        )
        .unwrap();

        let proof = beerus
            .starknet_lightclient
            .get_contract_storage_proof(contract_address, Vec::from(keys), &BlockId::Number(1))
            .await;

        mock.assert();
        assert_eq!(proof.unwrap(), expected_proof);
    }

    fn get_contract_storage_proof_mock(server: &MockServer) -> (Mock, GetProofOutput) {
        let path = "tests/data.json";
        let s = fs::read_to_string(path).unwrap();

        #[derive(Debug, Serialize, Deserialize)]
        struct JsonOutput {
            result: GetProofOutput,
        }
        let output: JsonOutput = serde_json::from_str(&s).unwrap();

        let mock = server.mock(|when, then| {
            when.method(POST).path("/").json_body(json!({
                "id":1,
                "jsonrpc":"2.0",
                "method":"pathfinder_getProof",
                "params":[
                    {
                        "block_number":1
                    },
                    "0x4d4e07157aeb54abeb64f5792145f2e8db1c83bda01a8f06e050be18cfb8153",
                    ["0x1"]
                ]
            }));
            then.status(200)
                .header("content-type", "application/json")
                .body_from_file(path);
        });
        (mock, output.result)
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
            data_dir: Some(PathBuf::from("/tmp")),
            starknet_rpc: server.base_url(),
            starknet_core_contract_address: Address::from_str(
                "0x0000000000000000000000000000000000000000",
            )
            .unwrap(),
        }
    }
}
