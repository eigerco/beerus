#[cfg(test)]
mod tests {
    use beerus_cli::model::CommandResponse;
    use starknet::core::types::FieldElement;
    #[test]
    fn test_display_ethereum_query_balance() {
        let response = CommandResponse::EthereumQueryBalance("1".to_string());
        assert_eq!(response.to_string(), "1 ETH");
    }

    #[test]
    fn test_display_ethereum_query_chain_id() {
        let response = CommandResponse::EthereumQueryChainId(1);
        assert_eq!(response.to_string(), "1");
    }

    #[test]
    fn test_display_starknet_query_state_root() {
        let response = CommandResponse::StarkNetQueryStateRoot(1.into());
        assert_eq!(response.to_string(), "1");
    }

    #[test]
    fn test_display_starknet_query_contract() {
        let response = CommandResponse::StarkNetQueryContract(vec![
            FieldElement::from_dec_str("123").unwrap(),
            FieldElement::from_dec_str("456").unwrap(),
        ]);
        assert_eq!(response.to_string(), "[123, 456]");
    }

    #[test]
    fn test_display_starknet_query_get_storage_at() {
        let response =
            CommandResponse::StarkNetQueryGetStorageAt(FieldElement::from_dec_str("123").unwrap());
        assert_eq!(response.to_string(), "123");
    }

    #[test]
    fn test_display_starknet_query_chain_id() {
        let response =
            CommandResponse::StarknetQueryChainId(FieldElement::from_dec_str("123").unwrap());
        assert_eq!(response.to_string(), "Chain id: 123");
    }

    #[test]
    fn test_display_starknet_query_block_number() {
        let response = CommandResponse::StarknetQueryBlockNumber(123456);
        assert_eq!(response.to_string(), "Block number: 123456");
    }
}
