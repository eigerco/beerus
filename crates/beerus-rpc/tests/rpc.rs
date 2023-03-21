mod common;

#[cfg(test)]
mod tests {
    use crate::common::setup_beerus_rpc;
    use beerus_rpc::api::{BeerusApiError, BeerusApiServer};
    use jsonrpsee::types::error::ErrorObjectOwned;

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
}
