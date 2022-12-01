#[cfg(test)]
mod tests {
    use beerus_core::{
        config::Config,
        lightclient::beerus::{Beerus, BeerusLightClient},
    };
    use ethers::{
        abi::Abi,
        contract::Contract,
        providers::{Http, Provider},
        types::Address,
    };
    use primitive_types::U256;

    use std::convert::TryFrom;

    #[tokio::test]
    async fn starknet_state_root_works() {
        let config = Config::default();
        let mut beerus = BeerusLightClient::new(config).unwrap();
        beerus.start().await.unwrap();

        beerus.starknet_state_root().await.unwrap();
    }

    #[tokio::test]
    async fn starknet_state_root_works_with_untrusted_rpc() {
        let starknet_core_contract_address = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e"
            .parse::<Address>()
            .unwrap();
        let abi: Abi = serde_json::from_str(
                r#"[{"inputs":[],"name":"stateRoot","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"}]"#,
            ).unwrap();
        let ethereum_execution_rpc = std::env::var("ETHEREUM_EXECUTION_RPC_URL").unwrap();

        // connect to the network
        let client = Provider::<Http>::try_from(ethereum_execution_rpc).unwrap();
        // create the contract object at the address
        let contract = Contract::new(starknet_core_contract_address, abi, client);
        let starknet_root: U256 = contract
            .method::<_, U256>("stateRoot", ())
            .unwrap()
            .call()
            .await
            .unwrap();

        println!("StarkNet state root: {:?}", starknet_root);
    }
}
