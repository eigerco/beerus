use rstest::{fixture, rstest};
use starknet::macros::felt;
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::providers::Provider;
use url::Url;

#[fixture]
fn sn_test_client() -> JsonRpcClient<HttpTransport> {
    // TODO: setup katana to act as untrusted and mock helios
    JsonRpcClient::new(HttpTransport::new(Url::parse("http://127.0.0.1:5050").unwrap()))
}

#[rstest]
async fn read_endpoints(sn_test_client: JsonRpcClient<HttpTransport>) {
    let chain_id = sn_test_client.chain_id().await.unwrap();
    assert_eq!(felt!("0x4b4154414e41"), chain_id);
}
