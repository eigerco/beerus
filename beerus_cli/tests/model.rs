#[cfg(test)]
mod tests {
    use beerus_cli::model::CommandResponse;
    use starknet::{core::types::FieldElement, providers::jsonrpc::models::BlockHashAndNumber};
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

    #[test]
    fn test_display_starknet_query_block_hash_and_number() {
        let block_hash_and_number = BlockHashAndNumber {
            block_hash: FieldElement::from_dec_str("123456").unwrap(),
            block_number: 123456,
        };
        let response = CommandResponse::StarknetQueryBlockHashAndNumber(block_hash_and_number);
        assert_eq!(
            response.to_string(),
            "Block hash: 123456, Block number: 123456"
        );
    }

    #[test]
    fn test_display_starknet_query_l1_to_l2_message_nonce() {
        let response = CommandResponse::StarkNetL1ToL2MessageNonce(123.into());
        assert_eq!(response.to_string(), "L1 to L2 Message Nonce: 123");
    }

    #[test]
    fn test_display_starknet_get_class_hash_at() {
        let response =
            CommandResponse::StarknetQueryGetClassHash(FieldElement::from_dec_str("123").unwrap());
        assert_eq!(response.to_string(), "Class hash: 123");
    }
}
