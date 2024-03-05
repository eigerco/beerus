use reqwest::Url;
use starknet::{
    core::types::{
        BlockHashAndNumber, BlockId, BlockTag, BlockWithTxHashes, BlockWithTxs,
        EventFilter, FieldElement, MaybePendingBlockWithTxHashes,
        MaybePendingBlockWithTxs, MaybePendingTransactionReceipt, Transaction,
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

    // TODO constants
    assert!(
        block.transactions.len() >= 10,
        "This test requires blocks to have at least N transactions, got {}",
        block.transactions.len()
    );

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
            l1_gas_price,
            starknet_version,
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
        assert_eq!(l1_gas_price, block.l1_gas_price);
        assert_eq!(starknet_version, block.starknet_version);

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
    // starknet_getEvents is tested with starknet_getTransactionReceipt
    // TODO starknet_getNonce
    // TODO starknet_getProof
    // TODO starknet_getStateRoot
    // TODO starknet_getStateUpdate
    // TODO starknet_getStorateAt

    // starknet_getTransactionByBlockIdAndIndex
    {
        async fn check_transaction(
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
            check_transaction(&client, block_id, transaction_index, &block)
                .await;
        }

        check_transaction(
            &client,
            block_id,
            block.transactions.len() - 1,
            &block,
        )
        .await;
    }

    // starknet_getTransactionByHash
    {
        async fn check_transaction(
            client: &JsonRpcClient<HttpTransport>,
            expected: &Transaction,
        ) {
            let transaction = client.get_transaction_by_hash(expected.transaction_hash()).await.expect("Failed to retrieve a specific transaction by its block ID and index");

            // `starknet-rs` doesn't implement `PartialEq` on its DTOs, and transactions have many *variants which makes pure Rust comparison painful.
            // Just serialize these to compare them.
            assert_eq!(
                serde_json::to_string(&transaction).unwrap(),
                serde_json::to_string(&expected).unwrap(),
            )
        }

        for transaction_index in 0..10 {
            check_transaction(&client, &block.transactions[transaction_index])
                .await;
        }

        check_transaction(
            &client,
            block
                .transactions
                .last()
                .as_ref()
                .expect("We need a last transaction here"),
        )
        .await;
    }

    // TODO starknet_getTransactionReceipt
    {
        async fn check_transaction(
            client: &JsonRpcClient<HttpTransport>,
            block: &BlockWithTxs,
            expected: &Transaction,
        ) {
            let receipt = client.get_transaction_receipt(expected.transaction_hash()).await.expect("Failed to retrieve a specific transaction by its block ID and index");

            let receipt = match receipt {
                MaybePendingTransactionReceipt::Receipt(receipt) => receipt,
                MaybePendingTransactionReceipt::PendingReceipt(_) => {
                    panic!("Pending receipt")
                }
            };

            let events = match receipt {
                starknet::core::types::TransactionReceipt::Invoke(receipt) => {
                    receipt.events
                }
                starknet::core::types::TransactionReceipt::L1Handler(
                    receipt,
                ) => receipt.events,
                starknet::core::types::TransactionReceipt::Declare(receipt) => {
                    receipt.events
                }
                starknet::core::types::TransactionReceipt::Deploy(receipt) => {
                    receipt.events
                }
                starknet::core::types::TransactionReceipt::DeployAccount(
                    receipt,
                ) => receipt.events,
            };

            // TODO This appears to make the test fail, too restrictive. Is that an AND filter?
            // let _keys: Vec<Vec<FieldElement>> = events.iter().map(|event| event.keys.clone()).collect();
            let keys = None;

            let block_id = BlockId::Number(block.block_number);

            let chunk_size = 1024;
            let expected_events = client
                .get_events(
                    EventFilter {
                        from_block: Some(block_id),
                        to_block: Some(block_id),
                        address: None,
                        keys,
                    },
                    None,
                    chunk_size,
                )
                .await
                .expect("Failed to retrieve the events");

            // TODO Loop
            assert_ne!(
                expected_events.events.len() as u64,
                chunk_size,
                "Got as many events as requested, there may be more to pull"
            );

            assert!(expected_events.events.len() >= events.len(), "getEvents should have returned at least as many events. GetEvents returned {} while we initially had {}", expected_events.events.len(), events.len());

            
            for actual in &events {
                // Find a matching event from the expected events vec.
                // We can't invert that relationship because getEvents might have returned more than we need here.
                if !expected_events.events.iter().any(|expected| {
                    actual.data == expected.data
                        && actual.from_address == expected.from_address
                        && actual.keys == expected.keys
                }) {
                    panic!("No match found");
                }
            }

            // TODO transaction status comparison
            // TODO execution result comparison
        }

        for transaction_index in 0..10 {
            check_transaction(
                &client,
                &block,
                &block.transactions[transaction_index],
            )
            .await;
        }

        check_transaction(
            &client,
            &block,
            block
                .transactions
                .last()
                .as_ref()
                .expect("We need a last transaction here"),
        )
        .await;
    }

    // starknet_getTransactionStatus
    {
        async fn check_transaction(
            client: &JsonRpcClient<HttpTransport>,
            transaction_hash: FieldElement,
        ) {
            // TODO No further assertion to make?
            let _status = client.get_transaction_status(transaction_hash).await.expect("Failed to retrieve the transaction status");
        }

        for transaction_index in 0..10 {
            check_transaction(
                &client,
                *block.transactions[transaction_index].transaction_hash(),
            )
            .await;
        }

        check_transaction(
            &client,
            *block
                .transactions
                .last()
                .as_ref()
                .expect("We need a last transaction here")
                .transaction_hash(),
        )
        .await;
    }
    // TODO starknet_syncing

    // TODO Look for other methods, I don't think the list is exhaustive.
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
