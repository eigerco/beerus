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
    fn test_display_ethereum_query_tx_count() {
        let response = CommandResponse::EthereumQueryTxCount(1);
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

    #[test]
    fn test_display_starknet_get_contract_storage_proof() {
        let json = r#"{
            "contract_proof": [
            {
                "binary": {
                    "left": "0x15e7882b80e22844ca62d3e3260a21d0d45c2b0c1744328e2763b4b486de738",
                    "right": "0x7779bcf84c8a6a4cca695c2d44d1455db0cb13d457ea7a01887676b9f779455"
                }
            },
            {
                "edge": {
                    "path": {
                        "value": "0x1",
                        "len": 1
                    },
                    "child": "0x173d276dbe8497dd2d59b88aa7eaebeb760e450e7a34a1ae5d513a930a3bf9d"
                }
            }
            ],
            "contract_data": null
        }"#;
        let response =
            CommandResponse::StarknetQueryContractStorageProof(serde_json::from_str(json).unwrap());

        assert_eq!(
            response.to_string(),
            "GetProofOutput { contract_proof: [Binary(Binary { left: FieldElement { inner: 0x015e7882b80e22844ca62d3e3260a21d0d45c2b0c1744328e2763b4b486de738 }, right: FieldElement { inner: 0x07779bcf84c8a6a4cca695c2d44d1455db0cb13d457ea7a01887676b9f779455 } }), Edge(Edge { path: Path { value: FieldElement { inner: 0x0000000000000000000000000000000000000000000000000000000000000001 }, len: 1 }, child: FieldElement { inner: 0x0173d276dbe8497dd2d59b88aa7eaebeb760e450e7a34a1ae5d513a930a3bf9d } })], contract_data: None }"
        );
    }
}
