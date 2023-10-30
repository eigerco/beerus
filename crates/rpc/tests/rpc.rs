mod common;

use common::setup_beerus_rpc;
use rstest::rstest;
// use starknet::macros::felt;

#[rstest]
#[ignore]
async fn read_endpoints() {
    let beerus_rpc = setup_beerus_rpc().await;
    todo!();
}
