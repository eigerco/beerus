use std::collections::HashMap;

use beerus::gen::client::Client;

use super::node::StarknetNode;

#[allow(dead_code)]
#[derive(Eq, Hash, PartialEq)]
pub enum StarknetMatcher {
    AddDeclareTransaction,
    AddDeclareTransactionMalicious,
    ChainId,
    ChainIdMalicious,
    ClassError,
    ClassSuccess,
    ClassMalicious,
    EstimateFee,
    EstimateFeeMalicious,
    Nonce,
    NonceMalicious,
    SpecVersion,
    SpecVersionMalicious,
}

#[allow(dead_code)]
pub async fn setup_client_with_mock_starknet_node(
    methods: Vec<StarknetMatcher>,
) -> (Client, StarknetNode) {
    let mut starknet_node = StarknetNode::new().await;
    let mut map_methods = HashMap::new();
    for method in methods {
        *map_methods.entry(method).or_insert(0) += 1;
    }
    starknet_node.add_methods(map_methods).await;
    let client = Client::new(&starknet_node.server.uri());
    (client, starknet_node)
}

#[macro_export]
macro_rules! setup {
    () => {{
        let run: bool = std::env::var("BEERUS_TEST_RUN")
            .ok()
            .map(|value| &value == "1")
            .unwrap_or_default();
        if !run {
            return Ok(());
        }
        if let Some(ctx) = common::context::ctx().await {
            ctx
        } else {
            panic!("Invalid test setup");
        }
    }};
}

#[macro_export]
macro_rules! client {
    () => {{
        let run: bool = std::env::var("BEERUS_TEST_RUN")
            .ok()
            .map(|value| &value == "1")
            .unwrap_or_default();
        if !run {
            return Ok(());
        }
        if let Ok(url) = std::env::var("BEERUS_TEST_STARKNET_URL") {
            Client::new(&url)
        } else {
            panic!("Invalid test setup");
        }
    }};
}
