// These tests need Beerus to run in the background, hence why they're hidden behind the following feature.
#![cfg(feature = "integration-tests")]

use beerus_core::config::DEFAULT_PORT;
use reqwest::Url;
use starknet::{
    core::types::{
        BlockId, BlockTag, BlockWithTxHashes, BlockWithTxs, DeclareTransaction,
        DeployAccountTransaction, FieldElement, InvokeTransaction,
        MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs,
        MaybePendingTransactionReceipt, Transaction, TransactionReceipt,
    },
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
};

fn rpc_client() -> JsonRpcClient<HttpTransport> {
    let rpc_url: Url = format!("http://localhost:{}", DEFAULT_PORT)
        .parse()
        .expect("Invalid RPC URL");
    JsonRpcClient::new(HttpTransport::new(rpc_url))
}

struct TestContext<T> {
    client: JsonRpcClient<HttpTransport>,
    block: BlockWithTxs,
    block_id: BlockId,
    extracted_value: T,
}

/// Creates a `TestContext` with the latest block.
async fn latest_block_context() -> TestContext<()> {
    context(|_block| Some(())).await
}

/// Creates a `TestContext` with the latest block with at least N transactions.
async fn n_txs_context(min_tx_number: usize) -> TestContext<()> {
    context(|block| {
        if block.transactions.len() >= min_tx_number {
            Some(())
        } else {
            None
        }
    })
    .await
}

/// Instantiate a client and look for a block matching the `extractor`, going from latest
///  to older.
///
/// # Parameters
///
/// * `extractor`: a closure returning `Some(T)` in the case of a match. The value returned
///  by the extractor will end up in the `TestContext`'s `extracted_value` field.
async fn context<F: Fn(&BlockWithTxs) -> Option<T>, T>(
    extractor: F,
) -> TestContext<T> {
    let client = rpc_client();

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

    if let Some(extracted_value) = extractor(&block) {
        return TestContext {
            client,
            block_id: BlockId::Number(block.block_number),
            block,
            extracted_value,
        };
    }

    // The latest block doesn't match the criterion.
    // Go to the lower blocks one by one until a match is found.

    let mut limit = 1_000;
    let mut block_number = block.block_number;

    while limit > 0 {
        limit -= 1;
        block_number -= 1;

        if block_number == 0 {
            panic!("Reached the genesis block, still no suitable block found");
        }

        let block = match client
            .get_block_with_txs(BlockId::Number(block_number))
            .await
            .expect("Block retrieval failed")
        {
            MaybePendingBlockWithTxs::Block(block) => block,
            MaybePendingBlockWithTxs::PendingBlock(_) => {
                continue;
            }
        };

        if let Some(extracted_value) = extractor(&block) {
            return TestContext {
                client,
                block_id: BlockId::Number(block.block_number),
                block,
                extracted_value,
            };
        }
    }

    panic!("No suitable block found")
}

// starknet_blockNumber is already tested in the creation of the test context.

#[tokio::test]
async fn test_chain_id() {
    let client = rpc_client();

    client.chain_id().await.expect("Failed to retrieve the chain ID");
}

#[tokio::test]
async fn test_get_block_transaction_count() {
    let TestContext { client, block, block_id, extracted_value: () } =
        latest_block_context().await;

    let tx_count = client
        .get_block_transaction_count(block_id)
        .await
        .expect("Failed to retrieve the transaction count");
    assert_eq!(tx_count, block.transactions.len() as u64);
}

// starknet_getBlockWithTxs is already tested in the creation of the test context.

#[tokio::test]
async fn test_get_block_with_tx_hashes() {
    let txs = 10;
    let TestContext { client, block, block_id, extracted_value: () } =
        n_txs_context(txs).await;

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

#[tokio::test]
async fn test_get_class() {
    let TestContext { client, block: _, block_id, extracted_value } =
        context(|block| {
            block.transactions.iter().find_map(
                |transaction| match transaction {
                    Transaction::Declare(DeclareTransaction::V0(declare)) => {
                        Some(declare.class_hash)
                    }
                    Transaction::Declare(DeclareTransaction::V1(declare)) => {
                        Some(declare.class_hash)
                    }
                    Transaction::Declare(DeclareTransaction::V2(declare)) => {
                        Some(declare.class_hash)
                    }
                    Transaction::Declare(DeclareTransaction::V3(declare)) => {
                        Some(declare.class_hash)
                    }
                    _ => None,
                },
            )
        })
        .await;

    client.get_class(block_id, extracted_value).await.expect("getClass failed");
}

#[tokio::test]
async fn test_get_class_at() {
    let TestContext {
        client,
        block: _,
        block_id,
        extracted_value: deploy_tx_hash,
    } = context(|block| {
        block.transactions.iter().find_map(|transaction| match transaction {
            Transaction::DeployAccount(DeployAccountTransaction::V3(
                deploy,
            )) => Some(deploy.transaction_hash),
            _ => None,
        })
    })
    .await;

    let receipt = match client
        .get_transaction_receipt(deploy_tx_hash)
        .await
        .expect("the transaction to have a matching receipt")
    {
        MaybePendingTransactionReceipt::Receipt(
            TransactionReceipt::DeployAccount(receipt),
        ) => receipt,
        _ => panic!("Expected a valid receipt, got a pending one"),
    };

    client
        .get_class_at(block_id, receipt.contract_address)
        .await
        .expect("getClass failed");
}

#[tokio::test]
async fn test_get_class_hash_at() {
    let TestContext {
        client,
        block: _,
        block_id,
        extracted_value: deploy_tx_hash,
    } = context(|block| {
        block.transactions.iter().find_map(|transaction| match transaction {
            Transaction::DeployAccount(DeployAccountTransaction::V3(
                deploy,
            )) => Some(deploy.transaction_hash),
            _ => None,
        })
    })
    .await;

    let receipt = match client
        .get_transaction_receipt(deploy_tx_hash)
        .await
        .expect("the transaction to have a matching receipt")
    {
        MaybePendingTransactionReceipt::Receipt(
            TransactionReceipt::DeployAccount(receipt),
        ) => receipt,
        _ => panic!("Expected a valid receipt, got a pending one"),
    };

    client
        .get_class_hash_at(block_id, receipt.contract_address)
        .await
        .expect("getClass failed");
}

#[tokio::test]
async fn test_get_nonce() {
    let TestContext {
        client,
        block: _,
        block_id,
        extracted_value: (original_nonce, sender),
    } = context(|block| {
        // Browse the transaction in reverse order to make sure we got the latest nonce.
        block.transactions.iter().rev().find_map(
            |transaction| match transaction {
                Transaction::Invoke(InvokeTransaction::V1(invoke)) => {
                    Some((invoke.nonce, invoke.sender_address))
                }
                Transaction::Invoke(InvokeTransaction::V3(invoke)) => {
                    Some((invoke.nonce, invoke.sender_address))
                }
                _ => None,
            },
        )
    })
    .await;
    let original_nonce = truncate_felt_to_u128(&original_nonce);

    let incremented_nonce =
        client.get_nonce(block_id, sender).await.expect("getNonce failed");
    let incremented_nonce = truncate_felt_to_u128(&incremented_nonce);

    assert_eq!(original_nonce + 1, incremented_nonce);
}

#[tokio::test]
async fn test_get_transaction_by_block_id_and_index() {
    let txs = 10;
    let TestContext { client, block, block_id, extracted_value: () } =
        n_txs_context(txs).await;

    async fn check_transaction(
        client: &JsonRpcClient<HttpTransport>,
        block_id: BlockId,
        transaction_index: usize,
        expected: &BlockWithTxs,
    ) {
        let transaction = client.get_transaction_by_block_id_and_index(block_id, transaction_index as u64).await.expect("Failed to retrieve a specific transaction by its block ID and index");

        // `starknet-rs` doesn't implement `PartialEq` on its DTOs, and transactions have many *variants which makes pure Rust comparison painful.
        // Just serialize these to compare them.
        pretty_assertions::assert_eq!(
            serde_json::to_value(&transaction).unwrap(),
            serde_json::to_value(&expected.transactions[transaction_index])
                .unwrap(),
        )
    }

    for transaction_index in 0..txs {
        check_transaction(&client, block_id, transaction_index, &block).await;
    }

    check_transaction(&client, block_id, block.transactions.len() - 1, &block)
        .await;
}

#[tokio::test]
async fn test_get_transaction_by_hash() {
    let txs = 10;
    let TestContext { client, block, block_id: _, extracted_value: () } =
        n_txs_context(txs).await;

    async fn check_transaction(
        client: &JsonRpcClient<HttpTransport>,
        expected: &Transaction,
    ) {
        let transaction = client.get_transaction_by_hash(expected.transaction_hash()).await.expect("Failed to retrieve a specific transaction by its block ID and index");

        // `starknet-rs` doesn't implement `PartialEq` on its DTOs, and transactions have many *variants which makes pure Rust comparison painful.
        // Just serialize these to compare them.
        pretty_assertions::assert_eq!(
            serde_json::to_value(&transaction).unwrap(),
            serde_json::to_value(&expected).unwrap(),
        )
    }

    for transaction_index in 0..txs {
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

#[tokio::test]
async fn test_get_transaction_status() {
    let txs = 10;
    let TestContext { client, block, block_id: _, extracted_value: () } =
        n_txs_context(txs).await;

    async fn check_transaction(
        client: &JsonRpcClient<HttpTransport>,
        transaction_hash: FieldElement,
    ) {
        let _status = client
            .get_transaction_status(transaction_hash)
            .await
            .expect("Failed to retrieve the transaction status");
    }

    for transaction_index in 0..txs {
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

/* TODO
   Add more test scenarios to cover the following methods:

   starknet_block_hash_and_number
   starknet_call
   starknet_estimateFee
   starknet_estimateFeeSingle
   starknet_getBalance
   starknet_getEvents
   starknet_getProof
   starknet_getStateRoot
   starknet_getStateUpdate
   starknet_getStorateAt
   starknet_getTransactionReceipt
   starknet_getTransactionStatus
   starknet_syncing
   pathfinder_getProof
*/

fn truncate_felt_to_u128(felt: &FieldElement) -> u128 {
    u128::from_be_bytes(felt.to_bytes_be()[16..].try_into().unwrap())
}
