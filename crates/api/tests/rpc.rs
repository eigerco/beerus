use beerus_api::gen::Rpc;

#[tokio::test]
async fn test_rpc() -> Result<(), iamgroot::jsonrpc::Error> {
    let client = beerus_api::gen::client::Client::new("http://localhost:9000/rpc");
    assert_eq!(client.specVersion().await?, "0.5.1");
    Ok(())
}
