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
    use starknet::providers::jsonrpc::models::FeeEstimate;

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
    async fn test_get_estimate_fee_ok() {
        let beerus_rpc = setup_beerus_rpc().await;

        let block_type = "hash".to_string();
        let block_hash =
            "0x0147c4b0f702079384e26d9d34a15e7758881e32b219fc68c076b09d0be13f8c".to_string();
        let broadcasted_transaction = "{ \"type\": \"INVOKE\", \"nonce\": \"0x0\", \"max_fee\": \"0x12C72866EFA9B\", \"version\": \"0x0\", \"signature\": [ \"0x10E400D046147777C2AC5645024E1EE81C86D90B52D76AB8A8125E5F49612F9\", \"0x0ADB92739205B4626FEFB533B38D0071EB018E6FF096C98C17A6826B536817B\" ], \"contract_address\": \"0x0019fcae2482de8fb3afaf8d4b219449bec93a5928f02f58eef645cc071767f4\", \"calldata\": [ \"0x0000000000000000000000000000000000000000000000000000000000000001\", \"0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7\", \"0x0083afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e\", \"0x0000000000000000000000000000000000000000000000000000000000000000\", \"0x0000000000000000000000000000000000000000000000000000000000000003\", \"0x0000000000000000000000000000000000000000000000000000000000000003\", \"0x04681402a7ab16c41f7e5d091f32fe9b78de096e0bd5962ce5bd7aaa4a441f64\", \"0x000000000000000000000000000000000000000000000000001d41f6331e6800\", \"0x0000000000000000000000000000000000000000000000000000000000000000\", \"0x0000000000000000000000000000000000000000000000000000000000000001\" ], \"entry_point_selector\": \"0x015d40a3d6ca2ac30f4031e42be28da9b056fef9bb7357ac5e85627ee876e5ad\" }".to_string();

        let expected = FeeEstimate {
            gas_consumed: 0x1de6,
            gas_price: 0x5df32828e,
            overall_fee: 0xaf8f402b6194,
        };

        let actual = beerus_rpc
            .starknet_estimate_fee(block_type, block_hash, broadcasted_transaction)
            .await
            .unwrap();

        assert_eq!(expected.gas_consumed, actual.gas_consumed);
        assert_eq!(expected.gas_price, actual.gas_price);
        assert_eq!(expected.overall_fee, actual.overall_fee);
    }
}
