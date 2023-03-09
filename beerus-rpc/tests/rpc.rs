mod utils;

#[cfg(test)]
mod tests {
    use crate::utils::setup_beerus_rpc;
    use beerus_rpc::beerus_rpc_server::BeerusApiServer;

    #[tokio::test]
    async fn test_block_number_is_ok() {
        let beerus_rpc = setup_beerus_rpc().await;
        let block_number = beerus_rpc.stark_block_number().await.unwrap();
        assert_eq!(block_number, 19640);
    }
}
