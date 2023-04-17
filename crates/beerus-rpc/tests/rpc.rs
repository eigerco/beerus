mod common;

#[cfg(test)]
mod tests {

    use crate::common::setup_beerus_rpc;
    use beerus_core::starknet_helper::create_mock_get_events;
    use beerus_rpc::api::{BeerusApiError, BeerusApiServer};
    use jsonrpsee::types::error::ErrorObjectOwned;
    use starknet::core::types::FieldElement;
    use starknet::providers::jsonrpc::models::{
        BlockId, BlockStatus, BlockTag, BlockWithTxHashes, EventFilter, FeeEstimate, FunctionCall,
        InvokeTransaction, InvokeTransactionV1, MaybePendingBlockWithTxHashes, SyncStatusType,
        Transaction,
    };
    #[tokio::test]
    async fn starknet_block_number_ok() {
        let beerus_rpc = setup_beerus_rpc().await;
        let block_number = beerus_rpc.block_number().await.unwrap();
        assert_eq!(block_number, 19640);
    }

    #[tokio::test]
    async fn starknet_block_transaction_count_ok() {
        let beerus_rpc = setup_beerus_rpc().await;
        let transaction_count = beerus_rpc
            .get_block_transaction_count(BlockId::Tag(BlockTag::Latest))
            .await
            .unwrap();

        assert_eq!(transaction_count, 90);
    }

    #[tokio::test]
    async fn starknet_error_response_block_not_found() {
        let beerus_rpc = setup_beerus_rpc().await;
        let err = beerus_rpc
            .get_block_with_tx_hashes(BlockId::Number(22050))
            .await
            .unwrap_err();

        let beerus_rpc_err = ErrorObjectOwned::from(err);
        assert_eq!(beerus_rpc_err.code(), BeerusApiError::BlockNotFound as i32);
        assert_eq!(
            beerus_rpc_err.message(),
            BeerusApiError::BlockNotFound.to_string()
        );
    }

    #[tokio::test]
    async fn test_get_events() {
        let beerus_rpc = setup_beerus_rpc().await;
        // TODO: avoid duplicating the input values in wiremock.rs
        let filter = EventFilter {
            from_block: Some(BlockId::Number(800)),
            to_block: Some(BlockId::Number(1701)),
            address: None,
            keys: None,
        };
        let continuation_token = Some("1000".to_string());
        let chunk_size = 1000;

        let events_page = beerus_rpc
            .get_events(filter, continuation_token, chunk_size)
            .await
            .unwrap();

        let (expected_events_page, _) = create_mock_get_events();

        assert_eq!(
            events_page.continuation_token,
            expected_events_page.continuation_token
        );

        for (event, expected_event) in events_page
            .events
            .iter()
            .zip(expected_events_page.events.iter())
        {
            assert_eq!(expected_event.from_address, event.from_address);
            assert_eq!(expected_event.block_hash, event.block_hash);
            assert_eq!(expected_event.block_number, event.block_number);
            assert_eq!(expected_event.data, event.data);
            assert_eq!(expected_event.keys, event.keys);
        }
    }

    #[tokio::test]
    async fn test_get_estimate_fee_ok() {
        let beerus_rpc = setup_beerus_rpc().await;

        let block_hash = BlockId::Hash(
            FieldElement::from_hex_be(
                "0x0147c4b0f702079384e26d9d34a15e7758881e32b219fc68c076b09d0be13f8c",
            )
            .unwrap(),
        );
        let broadcasted_transaction = "{ \"type\": \"INVOKE\", \"nonce\": \"0x0\", \"max_fee\": \"0x12C72866EFA9B\", \"version\": \"0x0\", \"signature\": [ \"0x10E400D046147777C2AC5645024E1EE81C86D90B52D76AB8A8125E5F49612F9\", \"0x0ADB92739205B4626FEFB533B38D0071EB018E6FF096C98C17A6826B536817B\" ], \"contract_address\": \"0x0019fcae2482de8fb3afaf8d4b219449bec93a5928f02f58eef645cc071767f4\", \"calldata\": [ \"0x0000000000000000000000000000000000000000000000000000000000000001\", \"0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7\", \"0x0083afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e\", \"0x0000000000000000000000000000000000000000000000000000000000000000\", \"0x0000000000000000000000000000000000000000000000000000000000000003\", \"0x0000000000000000000000000000000000000000000000000000000000000003\", \"0x04681402a7ab16c41f7e5d091f32fe9b78de096e0bd5962ce5bd7aaa4a441f64\", \"0x000000000000000000000000000000000000000000000000001d41f6331e6800\", \"0x0000000000000000000000000000000000000000000000000000000000000000\", \"0x0000000000000000000000000000000000000000000000000000000000000001\" ], \"entry_point_selector\": \"0x015d40a3d6ca2ac30f4031e42be28da9b056fef9bb7357ac5e85627ee876e5ad\" }".to_string();

        let expected = FeeEstimate {
            gas_consumed: 0x1de6,
            gas_price: 0x5df32828e,
            overall_fee: 0xaf8f402b6194,
        };

        let actual = beerus_rpc
            .estimate_fee(block_hash, broadcasted_transaction)
            .await
            .unwrap();

        assert_eq!(expected.gas_consumed, actual.gas_consumed);
        assert_eq!(expected.gas_price, actual.gas_price);
        assert_eq!(expected.overall_fee, actual.overall_fee);
    }

    #[tokio::test]
    async fn starknet_syncing_ok() {
        let beerus_rpc = setup_beerus_rpc().await;

        let sync_status = beerus_rpc.syncing().await.unwrap();

        let expected_current_block = FieldElement::from_hex_be(
            "0x7f65231188b64236c1142ae6a894e826583725bef6b9172f46b6ad5f9d87469",
        )
        .unwrap();
        let expected_starting_block = FieldElement::from_hex_be(
            "0x54cfb11a0c61c26b2e84c6d085a8317e5a1a437fa092d59a97564936afe2438",
        )
        .unwrap();

        match sync_status {
            SyncStatusType::Syncing(result) => {
                assert_eq!(result.current_block_hash, expected_current_block);
                assert_eq!(result.current_block_num, 27468);
                assert_eq!(result.highest_block_hash, expected_current_block);
                assert_eq!(result.highest_block_num, 27468);
                assert_eq!(result.starting_block_hash, expected_starting_block);
                assert_eq!(result.starting_block_num, 24317);
            }
            SyncStatusType::NotSyncing => panic!("Syncing status should be true"),
        }
    }

    #[tokio::test]
    async fn starknet_starknet_block_hash_and_number_ok() {
        let beerus_rpc = setup_beerus_rpc().await;
        let result = beerus_rpc.block_hash_and_number().await.unwrap();
        assert_eq!(result.block_number, 27461);
        assert_eq!(
            result.block_hash,
            FieldElement::from_hex_be(
                "0x63813d0cd71bf351dfe3217f9d2dcd8871cf4d56c0ffe3563980b3d02b6898d"
            )
            .unwrap()
        );
    }

    #[tokio::test]
    async fn starknet_get_transaction_by_block_id_and_index_ok() {
        let beerus_rpc = setup_beerus_rpc().await;
        let transaction = beerus_rpc
            .get_transaction_by_block_id_and_index(BlockId::Tag(BlockTag::Latest), "5")
            .await
            .unwrap();

        let felt = FieldElement::from_hex_be("0x1").unwrap();
        let invoke_transaction = InvokeTransactionV1 {
            transaction_hash: felt.clone(),
            max_fee: felt.clone(),
            signature: vec![felt.clone()],
            nonce: felt.clone(),
            sender_address: felt.clone(),
            calldata: vec![felt.clone()],
        };
        let expected_transaction = Transaction::Invoke(InvokeTransaction::V1(invoke_transaction));

        assert_eq!(
            serde_json::to_string(&transaction).unwrap(),
            serde_json::to_string(&expected_transaction).unwrap()
        );
    }

    #[tokio::test]
    async fn starknet_get_block_with_tx_hashes_ok() {
        let beerus_rpc = setup_beerus_rpc().await;
        let block_with_tx_hashes = beerus_rpc
            .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
            .await
            .unwrap();

        let felt = FieldElement::from_hex_be("0x1").unwrap();
        let block = BlockWithTxHashes {
            status: BlockStatus::AcceptedOnL2,
            block_hash: felt.clone(),
            parent_hash: felt.clone(),
            block_number: 1,
            new_root: felt.clone(),
            timestamp: 10,
            sequencer_address: felt.clone(),
            transactions: vec![felt.clone()],
        };
        let expected_block_with_tx_hashes = MaybePendingBlockWithTxHashes::Block(block);

        assert_eq!(
            serde_json::to_string(&block_with_tx_hashes).unwrap(),
            serde_json::to_string(&expected_block_with_tx_hashes).unwrap()
        );
    }

    #[tokio::test]
    async fn starknet_call() {
        let beerus_rpc = setup_beerus_rpc().await;

        let request = FunctionCall {
            contract_address: FieldElement::from_hex_be(
                "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
            )
            .unwrap(),
            entry_point_selector: FieldElement::from_hex_be(
                "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60",
            )
            .unwrap(),
            calldata: Vec::new(),
        };
        let block_id = BlockId::Tag(BlockTag::Latest);

        let call_result: Vec<FieldElement> = beerus_rpc.call(request, block_id).await.unwrap();
        let expected: Vec<FieldElement> = vec![FieldElement::from_hex_be("298305742194").unwrap()];

        assert_eq!(call_result, expected);
    }
}
