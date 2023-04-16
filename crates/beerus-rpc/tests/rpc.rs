mod common;

#[cfg(test)]
mod tests {

    use crate::common::setup_beerus_rpc;
    use beerus_rpc::models::{EventFilterWithPage, ResultPageRequest};
    use beerus_core::starknet_helper::{
        create_mock_broadcasted_transaction, create_mock_get_events,
    };
    use beerus_rpc::api::{BeerusApiError, BeerusRpcServer};
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
        let block_number = beerus_rpc.starknet_block_number().await.unwrap();
        assert_eq!(block_number, 19640);
    }

    #[tokio::test]
    async fn starknet_block_transaction_count_ok() {
        let beerus_rpc = setup_beerus_rpc().await;
        let transaction_count = beerus_rpc
            .starknet_get_block_transaction_count(BlockId::Tag(BlockTag::Latest))
            .await
            .unwrap();

        assert_eq!(transaction_count, 90);
    }

    #[tokio::test]
    async fn starknet_error_response_block_not_found() {
        let beerus_rpc = setup_beerus_rpc().await;
        let err = beerus_rpc
            .starknet_get_block_with_tx_hashes(BlockId::Number(22050))
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

        let custom_filter = EventFilterWithPage {
            filter,
            page: ResultPageRequest {
                continuation_token,
                chunk_size,
            },
        };

        let events_page = beerus_rpc.starknet_get_events(custom_filter).await.unwrap();

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
        let broadcasted_transaction = create_mock_broadcasted_transaction();

        let expected = FeeEstimate {
            gas_consumed: 0x1de6,
            gas_price: 0x5df32828e,
            overall_fee: 0xaf8f402b6194,
        };

        let actual = beerus_rpc
<<<<<<< HEAD
            .starknet_estimate_fee(block_hash, broadcasted_transaction)
=======
            .estimate_fee(block_hash, broadcasted_transaction.0)
>>>>>>> 45fb0c3 (Refactor starknet_getEstimateFee + error handling)
            .await
            .unwrap();

        assert_eq!(expected.gas_consumed, actual.gas_consumed);
        assert_eq!(expected.gas_price, actual.gas_price);
        assert_eq!(expected.overall_fee, actual.overall_fee);
    }

    #[tokio::test]
    async fn starknet_syncing_ok() {
        let beerus_rpc = setup_beerus_rpc().await;

        let sync_status = beerus_rpc.starknet_syncing().await.unwrap();

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
        let result = beerus_rpc.starknet_block_hash_and_number().await.unwrap();
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
            .starknet_get_transaction_by_block_id_and_index(BlockId::Tag(BlockTag::Latest), "5")
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
            .starknet_get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
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

        let call_result: Vec<FieldElement> = beerus_rpc
            .starknet_call(request, BlockId::Tag(BlockTag::Latest))
            .await
            .unwrap();
        let expected: Vec<FieldElement> = vec![FieldElement::from_hex_be("298305742194").unwrap()];

        assert_eq!(call_result, expected);
    }
}
