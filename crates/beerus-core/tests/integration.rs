#![cfg(not(target_arch = "wasm32"))]

pub mod common;
use common::{mock_call, mock_get_contract_storage_proof, mock_get_storage_at, mock_server_config};

#[cfg(test)]
mod test {
    use super::*;
    use beerus_core::lightclient::{
        beerus::BeerusLightClient, ethereum::MockEthereumLightClient,
        starknet::StarkNetLightClientImpl,
    };
    use ethers::types::U256;
    use eyre::eyre;
    #[cfg(not(target_arch = "wasm32"))]
    use httpmock::prelude::*;
    use starknet::{core::types::FieldElement, providers::jsonrpc::models::BlockId};
    use std::str::FromStr;

    #[tokio::test]
    async fn given_normal_conditions_when_starknet_get_storage_at_should_work() {
        // Start a lightweight mock server.
        let server = MockServer::start();
        let mock_request = mock_get_storage_at(&server);
        let config = mock_server_config(&server);

        let starknet_lightclient = Box::new(StarkNetLightClientImpl::new(&config).unwrap());
        let mut helios_lightclient = MockEthereumLightClient::new();

        helios_lightclient
            .expect_starknet_last_proven_block()
            .return_once(move || Ok(U256::from(1)));
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
        let mock_request = mock_get_storage_at(&server);
        let config = mock_server_config(&server);

        let starknet_lightclient = Box::new(StarkNetLightClientImpl::new(&config).unwrap());
        let mut helios_lightclient = MockEthereumLightClient::new();
        let expected_error = "Ethereum light client error";

        // Mock the `start` method of the Ethereum light client.
        helios_lightclient
            .expect_starknet_last_proven_block()
            .return_once(move || Err(eyre!(expected_error)));
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
        let mock_request = mock_call(&server);
        let config = mock_server_config(&server);

        let starknet_lightclient = Box::new(StarkNetLightClientImpl::new(&config).unwrap());
        let mut helios_lightclient = MockEthereumLightClient::new();
        helios_lightclient
            .expect_call()
            .return_once(move |_req, _block_nb| Ok(vec![1]));
        helios_lightclient
            .expect_starknet_last_proven_block()
            .return_once(move || Ok(U256::from(1)));
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
        let mock_request = mock_get_storage_at(&server);
        let config = mock_server_config(&server);

        let starknet_lightclient = Box::new(StarkNetLightClientImpl::new(&config).unwrap());
        let mut helios_lightclient = MockEthereumLightClient::new();
        let expected_error = "Ethereum light client error";

        // Mock the `start` method of the Ethereum light client.
        helios_lightclient
            .expect_starknet_last_proven_block()
            .return_once(move || Err(eyre!(expected_error)));
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
        let config = mock_server_config(&server);

        let starknet_lightclient = Box::new(StarkNetLightClientImpl::new(&config).unwrap());
        let helios_lightclient = MockEthereumLightClient::new();

        let beerus =
            BeerusLightClient::new(config, Box::new(helios_lightclient), starknet_lightclient);

        let (mock, expected_proof) = mock_get_contract_storage_proof(&server);

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
}
