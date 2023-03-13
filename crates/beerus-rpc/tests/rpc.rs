mod utils;

#[cfg(test)]
mod tests {
    use crate::utils::setup_beerus_rpc;
    use beerus_rpc::server::BeerusApiServer;

    #[tokio::test]
    async fn test_block_number_is_ok() {
        let beerus_rpc = setup_beerus_rpc().await;
        let block_number = beerus_rpc.starknet_block_number().await.unwrap();
        assert_eq!(block_number, 19640);
    }

    #[tokio::test]
    async fn test_get_block_transaction_count_is_ok() {
        let beerus_rpc = setup_beerus_rpc().await;
        let block_id_type = "tag".to_string();
        let block_id = "latest".to_string();
        let transaction_count = beerus_rpc
            .starknet_get_block_transaction_count(block_id_type, block_id)
            .await
            .unwrap();
        assert_eq!(transaction_count, 90);
    }
}
