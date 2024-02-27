use reqwest::Url;
use starknet::{
    core::types::{
        BlockHashAndNumber, BlockId, BlockTag, BlockWithTxHashes, BlockWithTxs,
        MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs,
    },
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
};

#[tokio::test]
async fn rpc_test() {
    let rpc_url: Url =
        "http://localhost:3030".parse().expect("Invalid RPC URL");
    let client = JsonRpcClient::new(HttpTransport::new(rpc_url));

    let block = match client
        .get_block_with_txs(BlockId::Tag(BlockTag::Latest))
        .await
        .expect("Failed to retrieve the latest block")
    {
        MaybePendingBlockWithTxs::Block(block) => block,
        MaybePendingBlockWithTxs::PendingBlock(_) => {
            panic!("The latest block shouldn't be a pending block")
        }
    };

    // TODO In the case the block just changed, retry
    // starknet_blockNumber
    {
        let block_number = client
            .block_number()
            .await
            .expect("Failed to retrieve the latest block number");
        assert_eq!(block_number, block.block_number);
    }

    // starknet_blockHashAndNumber
    {
        let BlockHashAndNumber { block_hash, block_number } = client
            .block_hash_and_number()
            .await
            .expect("Failed to retrieve the latest hash & block number");
        assert_eq!(block_number, block.block_number);
        assert_eq!(block_hash, block.block_hash);
    }

    // TODO starknet_call
    // TODO starknet_chainId
    // TODO starknet_estimateFee
    // TODO starknet_estimateFeeSingle
    // TODO starknet_getBalance

    let block_id = BlockId::Number(block.block_number);

    // starknet_getBlockTransactionCount
    {
        let tx_count = client
            .get_block_transaction_count(block_id)
            .await
            .expect("Failed to retrieve the transaction count");
        assert_eq!(tx_count, block.transactions.len() as u64);
    }

    // starknet_getBlockWithTxHashes
    {
        let BlockWithTxHashes {
            status,
            block_hash,
            parent_hash,
            block_number,
            new_root,
            timestamp,
            sequencer_address,
            transactions,
        } = match client.get_block_with_tx_hashes(block_id).await.expect("Failed to retrieve the block with transaction hashes") {
            MaybePendingBlockWithTxHashes::Block(with_tx_hashes) => with_tx_hashes,
            MaybePendingBlockWithTxHashes::PendingBlock(_) => panic!("This block was already verified as not pending, it shouldn't be a pending block now"),
        };

        assert_eq!(status, block.status);
        assert_eq!(block_hash, block.block_hash);
        assert_eq!(parent_hash, block.parent_hash);
        assert_eq!(block_number, block.block_number);
        assert_eq!(new_root, block.new_root);
        assert_eq!(timestamp, block.timestamp);
        assert_eq!(sequencer_address, block.sequencer_address);

        block
            .transactions
            .iter()
            .map(|tx| tx.transaction_hash().to_owned())
            .zip(transactions)
            .for_each(|(expected, actual)| assert_eq!(actual, expected));
    }

    // TODO starknet_getClass
    // TODO starknet_getClassAt
    // TODO starknet_getClassHashAt
    // TODO starknet_getNonce
    // TODO starknet_getProof
    // TODO starknet_getStateRoot
    // TODO starknet_getStateUpdate
    // TODO starknet_getStorateAt
    
    // starknet_getTransactionByBlockIdAndIndex
    {
        async fn check_get_transaction(
            client: &JsonRpcClient<HttpTransport>,
            block_id: BlockId,
            transaction_index: usize,
            expected: &BlockWithTxs,
        ) {
            let transaction = client.get_transaction_by_block_id_and_index(block_id, transaction_index as u64).await.expect("Failed to retrieve a specific transaction by its block ID and index");

            // `starknet-rs` doesn't implement `PartialEq` on its DTOs, and transactions have many *variants which makes pure Rust comparison painful.
            // Just serialize these to compare them.
            assert_eq!(
                serde_json::to_string(&transaction).unwrap(),
                serde_json::to_string(
                    &expected.transactions[transaction_index]
                )
                .unwrap(),
            )
        }

        for transaction_index in 0..10 {
            check_get_transaction(&client, block_id, transaction_index, &block)
                .await;
        }

        check_get_transaction(&client, block_id, block.transactions.len() - 1, &block)
        .await;
    }

    // TODO starknet_getTransactionByHash
    // TODO starknet_getTransactionReceipt
    // TODO starknet_getTransactionStatus
    // TODO starknet_syncing
}

/* TODO
fn test_scope<T, F>(scope_name: &'static str, test: T) where T: FnOnce() -> F, F: Future<Output=()> {
    let result = std::panic::catch_unwind(|| {
        println!("hello!");
    });

    if let Err(err) = result {
        println!()
    }
}
 */