mod utils;

#[cfg(test)]
mod tests {
    use starknet::providers::jsonrpc::models::{FeeEstimate};
    use crate::utils::setup_beerus_rpc;
    use beerus_rpc::server::BeerusApiServer;

    #[tokio::test]
    async fn test_block_number_is_ok() {
        let beerus_rpc = setup_beerus_rpc().await;
        let block_number = beerus_rpc.starknet_block_number().await.unwrap();
        assert_eq!(block_number, 19640);
    }

    #[tokio::test]
    async fn test_get_estimate_fee_ok() {
        let beerus_rpc = setup_beerus_rpc().await;

        let block_type = "hash".to_string();
        let block_hash = "0x0147c4b0f702079384e26d9d34a15e7758881e32b219fc68c076b09d0be13f8c".to_string();
        let broadcasted_transaction = "{ \"type\": \"INVOKE\", \"nonce\": \"0x0\", \"max_fee\": \"0x12C72866EFA9B\", \"version\": \"0x0\", \"signature\": [ \"0x10E400D046147777C2AC5645024E1EE81C86D90B52D76AB8A8125E5F49612F9\", \"0x0ADB92739205B4626FEFB533B38D0071EB018E6FF096C98C17A6826B536817B\" ], \"contract_address\": \"0x0019fcae2482de8fb3afaf8d4b219449bec93a5928f02f58eef645cc071767f4\", \"calldata\": [ \"0x0000000000000000000000000000000000000000000000000000000000000001\", \"0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7\", \"0x0083afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e\", \"0x0000000000000000000000000000000000000000000000000000000000000000\", \"0x0000000000000000000000000000000000000000000000000000000000000003\", \"0x0000000000000000000000000000000000000000000000000000000000000003\", \"0x04681402a7ab16c41f7e5d091f32fe9b78de096e0bd5962ce5bd7aaa4a441f64\", \"0x000000000000000000000000000000000000000000000000001d41f6331e6800\", \"0x0000000000000000000000000000000000000000000000000000000000000000\", \"0x0000000000000000000000000000000000000000000000000000000000000001\" ], \"entry_point_selector\": \"0x015d40a3d6ca2ac30f4031e42be28da9b056fef9bb7357ac5e85627ee876e5ad\" }".to_string();

        let expected = FeeEstimate { gas_consumed: 0x1de6, gas_price: 0x5df32828e, overall_fee: 0xaf8f402b6194 };
        let actual = beerus_rpc.starknet_estimate_fee(broadcasted_transaction, block_type, block_hash).await.unwrap();

        assert_eq!(expected.gas_consumed, actual.gas_consumed);
        assert_eq!(expected.gas_price, actual.gas_price);
        assert_eq!(expected.overall_fee, actual.overall_fee);
    }
}
