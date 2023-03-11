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
}
