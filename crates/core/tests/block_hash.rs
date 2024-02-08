use beerus_core::block_hash::compute_block_hash;
use starknet::{
    core::types::{
        BlockId, BlockTag, Event, EventFilter, MaybePendingBlockWithTxs,
    },
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
};
use url::Url;

#[tokio::test]
async fn verify_latest_block_hash() {
    let rpc_client = {
        let rpc_url = std::env::var("STARKNET_RPC")
            .expect("Missing STARKNET_RPC env var");
        JsonRpcClient::new(HttpTransport::new(Url::parse(&rpc_url).unwrap()))
    };

    let block_id = BlockId::Tag(BlockTag::Latest);
    let block = rpc_client.get_block_with_txs(block_id).await.unwrap();

    let block = match block {
        MaybePendingBlockWithTxs::Block(block) => block,
        _ => panic!("unexpected block response type"),
    };

    let page_size = 1024;
    let mut events = vec![];
    let mut last_token = None;
    let mut continue_ = true;

    while continue_ {
        let events_page = rpc_client
            .get_events(
                EventFilter {
                    from_block: Some(block_id),
                    to_block: Some(block_id),
                    address: None,
                    keys: None,
                },
                last_token,
                page_size,
            )
            .await
            .unwrap();
        last_token = events_page.continuation_token;

        if last_token.is_none() {
            continue_ = false;
        }

        let mut new_events = events_page
            .events
            .into_iter()
            .map(|e| Event {
                from_address: e.from_address,
                keys: e.keys,
                data: e.data,
            })
            .collect::<Vec<Event>>();
        events.append(&mut new_events);
    }

    let expected = block.block_hash;
    assert_eq!(compute_block_hash(&block, &events), expected);
}
