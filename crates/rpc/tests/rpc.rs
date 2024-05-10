// These tests need Beerus to run in the background, hence why they're hidden behind the following feature.
#![cfg(feature = "integration-tests")]

use reqwest::Url;
use starknet::{
    core::types::{
        BlockId, BlockWithTxHashes, BlockWithTxs, DeclareTransaction,
        DeployAccountTransaction, FieldElement, InvokeTransaction,
        MaybePendingBlockWithTxHashes, MaybePendingTransactionReceipt,
        Transaction, TransactionReceipt,
    },
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
};

const TEST_RPC_URL: &str = "http://localhost:3030";

fn rpc_client() -> JsonRpcClient<HttpTransport> {
    let url: Url = TEST_RPC_URL.try_into().expect("Invalid RPC URL");
    JsonRpcClient::new(HttpTransport::new(url))
}

mod blocks {
    use futures::{stream, Stream, StreamExt};
    use starknet::{
        core::types::{BlockId, BlockWithTxs, MaybePendingBlockWithTxs},
        providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
    };

    use super::rpc_client;

    struct State {
        client: JsonRpcClient<HttpTransport>,
        latest: Option<u64>,
    }

    async fn next(state: State) -> Option<(BlockWithTxs, State)> {
        let latest = if let Some(latest) = state.latest.as_ref() {
            *latest
        } else {
            match state.client.block_number().await {
                Ok(latest) => latest,
                _ => {
                    return None;
                }
            }
        };

        let block = match state
            .client
            .get_block_with_txs(BlockId::Number(latest))
            .await
        {
            Ok(MaybePendingBlockWithTxs::Block(block)) => block,
            _ => {
                return None;
            }
        };

        if latest == 0 {
            return None;
        }

        let state = State { latest: Some(latest - 1), ..state };
        Some((block, state))
    }

    fn stream() -> impl Stream<Item = BlockWithTxs> {
        let state = State { client: rpc_client(), latest: None };
        stream::unfold(state, next)
    }

    pub async fn head() -> Option<BlockWithTxs> {
        let stream = stream();
        pin_utils::pin_mut!(stream);
        stream.next().await
    }

    pub async fn find(
        predicate: impl Fn(&BlockWithTxs) -> bool,
        limit: usize,
    ) -> Option<BlockWithTxs> {
        let stream = stream().take(limit).filter(|block| {
            let found = predicate(block);
            std::future::ready(found)
        });
        pin_utils::pin_mut!(stream);
        stream.next().await
    }

    pub async fn map<T>(
        predicate: impl Fn(&BlockWithTxs) -> Option<T>,
        limit: usize,
    ) -> Option<(BlockWithTxs, T)> {
        let stream = stream().take(limit).filter_map(|block| async {
            predicate(&block).map(|result| (block, result))
        });
        pin_utils::pin_mut!(stream);
        stream.next().await
    }
}

mod fixtures {
    use super::*;

    pub async fn latest_block(
    ) -> (JsonRpcClient<HttpTransport>, BlockWithTxs, BlockId) {
        let block = blocks::head().await.expect("failed to pull latest block");
        let block_id = BlockId::Number(block.block_number);
        (rpc_client(), block, block_id)
    }

    pub async fn block_with_min_ten_txs(
    ) -> (JsonRpcClient<HttpTransport>, BlockWithTxs, BlockId) {
        let block = blocks::find(|block| block.transactions.len() >= 10, 1000)
            .await
            .expect("failed to pull block with necessary tx count");
        let block_id = BlockId::Number(block.block_number);
        (rpc_client(), block, block_id)
    }
}

#[tokio::test]
async fn test_chain_id() {
    let client = rpc_client();

    client.chain_id().await.expect("chainId failed");
}

#[tokio::test]
async fn test_get_block_transaction_count() {
    let (client, block, block_id) = fixtures::latest_block().await;

    let tx_count = client
        .get_block_transaction_count(block_id)
        .await
        .expect("getBlockTransactionCount failed");
    assert_eq!(tx_count, block.transactions.len() as u64);
}

#[tokio::test]
async fn test_get_block_with_tx_hashes() {
    let (client, block, block_id) = fixtures::block_with_min_ten_txs().await;

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
    } = match client
        .get_block_with_tx_hashes(block_id)
        .await
        .expect("getBlockWithTxHashes failed")
    {
        MaybePendingBlockWithTxHashes::Block(with_tx_hashes) => with_tx_hashes,
        MaybePendingBlockWithTxHashes::PendingBlock(_) => {
            panic!("getBlockWithTxHashes returned a pending block")
        }
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
    let (block, tx_hash) = blocks::map(
        |block| {
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
        },
        1000,
    )
    .await
    .expect("failed to find tx hash");

    let client = rpc_client();
    let block_id = BlockId::Number(block.block_number);
    client.get_class(block_id, tx_hash).await.expect("getClass failed");
}

#[tokio::test]
async fn test_get_class_at() {
    let (block, tx_hash) = blocks::map(
        |block| {
            block.transactions.iter().find_map(
                |transaction| match transaction {
                    Transaction::DeployAccount(
                        DeployAccountTransaction::V3(deploy),
                    ) => Some(deploy.transaction_hash),
                    _ => None,
                },
            )
        },
        1000,
    )
    .await
    .expect("failed to find a deploy tx hash");
    let block_id = BlockId::Number(block.block_number);

    let client = rpc_client();
    let receipt = match client
        .get_transaction_receipt(tx_hash)
        .await
        .expect("getTransactionReceipt failed")
    {
        MaybePendingTransactionReceipt::Receipt(
            TransactionReceipt::DeployAccount(receipt),
        ) => receipt,
        _ => panic!("getTransactionReceipt returned a pending tx receipt"),
    };

    client
        .get_class_at(block_id, receipt.contract_address)
        .await
        .expect("getClass failed");
}

#[tokio::test]
async fn test_get_class_hash_at() {
    let (block, tx_hash) = blocks::map(
        |block| {
            block.transactions.iter().find_map(
                |transaction| match transaction {
                    Transaction::DeployAccount(
                        DeployAccountTransaction::V3(deploy),
                    ) => Some(deploy.transaction_hash),
                    _ => None,
                },
            )
        },
        1000,
    )
    .await
    .expect("failed to find a deploy tx hash");
    let block_id = BlockId::Number(block.block_number);

    let client = rpc_client();
    let receipt = match client
        .get_transaction_receipt(tx_hash)
        .await
        .expect("getTransactionReceipt failed")
    {
        MaybePendingTransactionReceipt::Receipt(
            TransactionReceipt::DeployAccount(receipt),
        ) => receipt,
        _ => panic!("getTransactionReceipt returned a pending tx receipt"),
    };

    client
        .get_class_hash_at(block_id, receipt.contract_address)
        .await
        .expect("getClass failed");
}

#[tokio::test]
async fn test_get_nonce() {
    let client = rpc_client();
    let (block, (nonce, sender)) = blocks::map(
        |block| {
            // Browse the transaction in reverse order to make sure we got the latest nonce.
            block.transactions.iter().rev().find_map(|transaction| {
                match transaction {
                    Transaction::Invoke(InvokeTransaction::V1(invoke)) => {
                        Some((invoke.nonce, invoke.sender_address))
                    }
                    Transaction::Invoke(InvokeTransaction::V3(invoke)) => {
                        Some((invoke.nonce, invoke.sender_address))
                    }
                    _ => None,
                }
            })
        },
        1000,
    )
    .await
    .expect("failed to find an invoke tx");
    let block_id = BlockId::Number(block.block_number);

    let nonce = truncate_felt_to_u128(&nonce);

    let updated_nonce =
        client.get_nonce(block_id, sender).await.expect("getNonce failed");
    let updated_nonce = truncate_felt_to_u128(&updated_nonce);

    assert_eq!(updated_nonce, nonce + 1);
}

#[tokio::test]
async fn test_get_transaction_by_block_id_and_index() {
    let (client, block, block_id) = fixtures::block_with_min_ten_txs().await;

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
            serde_json::to_value(transaction).unwrap(),
            serde_json::to_value(&expected.transactions[transaction_index])
                .unwrap(),
        )
    }

    for transaction_index in 0..block.transactions.len() {
        check_transaction(&client, block_id, transaction_index, &block).await;
    }

    check_transaction(&client, block_id, block.transactions.len() - 1, &block)
        .await;
}

#[tokio::test]
async fn test_get_transaction_by_hash() {
    let (client, block, _) = fixtures::block_with_min_ten_txs().await;

    async fn check_transaction(
        client: &JsonRpcClient<HttpTransport>,
        expected: &Transaction,
    ) {
        let transaction = client
            .get_transaction_by_hash(expected.transaction_hash())
            .await
            .expect("getTransactionByHash failed");

        // `starknet-rs` doesn't implement `PartialEq` on its DTOs, and transactions have many *variants which makes pure Rust comparison painful.
        // Just serialize these to compare them.
        pretty_assertions::assert_eq!(
            serde_json::to_value(transaction).unwrap(),
            serde_json::to_value(expected).unwrap(),
        )
    }

    for transaction_index in 0..block.transactions.len() {
        check_transaction(&client, &block.transactions[transaction_index])
            .await;
    }
}

#[tokio::test]
async fn test_get_transaction_status() {
    let (client, block, _) = fixtures::block_with_min_ten_txs().await;

    async fn check_transaction(
        client: &JsonRpcClient<HttpTransport>,
        transaction_hash: FieldElement,
    ) {
        let _status = client
            .get_transaction_status(transaction_hash)
            .await
            .expect("getTransactionStatus failed");
    }

    for transaction_index in 0..block.transactions.len() {
        check_transaction(
            &client,
            *block.transactions[transaction_index].transaction_hash(),
        )
        .await;
    }
}

fn truncate_felt_to_u128(felt: &FieldElement) -> u128 {
    u128::from_be_bytes(felt.to_bytes_be()[16..].try_into().unwrap())
}
