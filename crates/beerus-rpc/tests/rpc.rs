mod common;

#[cfg(test)]
mod tests {
    use crate::common::setup_beerus_rpc;
    use beerus_core::starknet_helper::create_mock_get_events;
    use beerus_rpc::{
        api::{BeerusApiError, BeerusApiServer},
        models::{BlockId, EventFilter},
    };
    use jsonrpsee::types::error::ErrorObjectOwned;
    use starknet::core::types::FieldElement;
    use starknet::providers::jsonrpc::models::{
        InvokeTransaction, InvokeTransactionV1, Transaction,
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
            .starknet_get_block_transaction_count("tag".to_string(), "latest".to_string())
            .await
            .unwrap();

        assert_eq!(transaction_count, 90);
    }

    #[tokio::test]
    async fn starknet_error_response_block_not_found() {
        let beerus_rpc = setup_beerus_rpc().await;
        let err = beerus_rpc
            .starknet_get_block_with_tx_hashes("number".to_string(), "22050".to_string())
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
    async fn starknet_get_transaction_by_block_id_and_index_ok() {
        let beerus_rpc = setup_beerus_rpc().await;
        let transaction = beerus_rpc
            .starknet_get_transaction_by_block_id_and_index("tag", "latest", "5")
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
}
