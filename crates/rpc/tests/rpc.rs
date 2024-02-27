/*
   TODO As expected, these states aren't as reliable as I hoped because to state changes.
   I see two possible ways to tackle this:
       * retries
       * maybe work on "mature" blocks, 100 below the latest to limit state changes?
*/

use reqwest::Url;
use starknet::{
    core::types::{
        BlockHashAndNumber, BlockId, BlockTag, BlockWithTxHashes, BlockWithTxs, BroadcastedInvokeTransactionV3, BroadcastedTransaction, DataAvailabilityMode, DeclareTransaction, DeployAccountTransaction, Event, EventFilter, FieldElement, FunctionCall, InvokeTransaction, MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs, MaybePendingTransactionReceipt, ResourceBounds, ResourceBoundsMapping, SimulationFlagForEstimateFee, Transaction, TransactionExecutionStatus, TransactionReceipt
    },
    providers::{
        jsonrpc::HttpTransport, JsonRpcClient, Provider, ProviderError,
    },
};

struct TestContext<T> {
    client: JsonRpcClient<HttpTransport>,
    block: BlockWithTxs,
    block_id: BlockId,
    extracted_value: T,
}

async fn latest_block_context() -> TestContext<()> {
    context(|_block| Some(())).await
}

// TODO Doc
async fn context<F: Fn(&BlockWithTxs) -> Option<T>, T>(
    extractor: F,
) -> TestContext<T> {
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

    // TODO Make it a criterion.
    assert!(
        block.transactions.len() >= 10,
        "This test requires blocks to have at least N transactions, got {}",
        block.transactions.len()
    );

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
            .expect("Failed to retrieve the latest block")
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

pub trait L2ClientExt {
    // TODO 550 Doc
    fn get_block_events(
        &self,
        block_id: BlockId,
    ) -> impl std::future::Future<Output = Result<Vec<Event>, ProviderError>> + Send;
}

impl L2ClientExt for JsonRpcClient<HttpTransport> {
    async fn get_block_events(
        &self,
        block_id: BlockId,
    ) -> Result<Vec<Event>, ProviderError> {
        let page_size = 1024;
        let mut events = vec![];
        let mut last_token = None;
        let mut more_to_fetch = true;

        while more_to_fetch {
            let events_page = self
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
                .await?;
            last_token = events_page.continuation_token;

            if last_token.is_none() {
                more_to_fetch = false;
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

        Ok(events)
    }
}

#[tokio::test]
async fn test_block_hash_and_number() {
    let TestContext { client, block, block_id: _, extracted_value: () } =
        latest_block_context().await;

    let BlockHashAndNumber { block_hash, block_number } = client
        .block_hash_and_number()
        .await
        .expect("Failed to retrieve the latest hash & block number");
    assert_eq!(block_number, block.block_number);
    assert_eq!(block_hash, block.block_hash);
}

#[tokio::test]
async fn test_block_number() {
    let TestContext { client, block, block_id: _, extracted_value: () } =
        latest_block_context().await;

    // TODO In the case the block just changed, retry
    let block_number = client
        .block_number()
        .await
        .expect("Failed to retrieve the latest block number");
    assert_eq!(block_number, block.block_number);
}

#[tokio::test]
async fn test_chain_id() {
    let TestContext { client, block: _, block_id: _, extracted_value: () } =
        latest_block_context().await;

    // TODO Assertion?
    client
        .chain_id()
        .await
        .expect("Failed to retrieve the chain ID");
}

#[tokio::test]
async fn test_estimate_fee() {
    let TestContext { client, block: _, block_id, extracted_value } =
        context(|block| {
            // Browse the transaction in reverse order to make sure we got the latest nonce.
            block.transactions.iter().rev().find_map(|transaction| {
                match transaction {
                    Transaction::Invoke(InvokeTransaction::V3(invoke)) => {
                        Some(invoke.clone())
                    }
                    // TODO Make the match exhaustive to make dep upgrades more reliable.
                    _ => None,
                }
            })
        })
        .await;
    let invoke = extracted_value;

    // TODO Criterion
    // TODO unwrap.
    assert_eq!(client.get_transaction_receipt(invoke.transaction_hash).await.unwrap().execution_result().status(), TransactionExecutionStatus::Succeeded);

    let incremented_nonce = {
        // TODO Ugly conversion
        let mut incremented_nonce = u128::from_be_bytes(
            invoke.nonce.to_bytes_be().split_at(16).1.try_into().unwrap(),
        );
        incremented_nonce += 1;

        FieldElement::from_byte_slice_be(&incremented_nonce.to_be_bytes())
            .unwrap()
    };

    let tx_to_estimate = BroadcastedInvokeTransactionV3 {
        sender_address: invoke.sender_address,
        signature: invoke.signature,
        nonce: incremented_nonce,
        is_query: true,
        resource_bounds: invoke.resource_bounds,
        tip: invoke.tip,
        paymaster_data: invoke.paymaster_data,
        account_deployment_data: invoke.account_deployment_data,
        nonce_data_availability_mode: invoke.nonce_data_availability_mode,
        fee_data_availability_mode: invoke.fee_data_availability_mode,
        calldata: invoke.calldata,
    };

    let estimation = client
        .estimate_fee(
            [BroadcastedTransaction::Invoke(
                starknet::core::types::BroadcastedInvokeTransaction::V3(
                    tx_to_estimate.clone(),
                ),
            )],
            &[SimulationFlagForEstimateFee::SkipValidate],
            block_id,
        )
        .await
        .expect("Fee estimation failed");

    assert_eq!(estimation.len(), 1);

    let single_estimation = client
        .estimate_fee_single(
            BroadcastedTransaction::Invoke(
                starknet::core::types::BroadcastedInvokeTransaction::V3(
                    tx_to_estimate,
                ),
            ),
            &[SimulationFlagForEstimateFee::SkipValidate],
            block_id,
        )
        .await
        .expect("Fee estimation failed");

    assert_eq!(estimation[0], single_estimation);
}

#[ignore = "tested with test_estimate_fee"]
#[test]
fn test_estimate_fee_single() {}

// TODO
#[ignore = "Incomplete, doesn't work yet"]
#[tokio::test]
async fn test_call() {
    let TestContext { client, block: _, block_id, extracted_value } =
    context(|block| {
        block.transactions.iter().find_map(|transaction| {
            match transaction {
                Transaction::DeployAccount(DeployAccountTransaction::V3(deploy)) => {
                    Some(deploy.clone())
                }
                // TODO Make the match exhaustive to make dep upgrades more reliable.
                _ => None,
            }
        })
    })
    .await;
    let deploy = extracted_value;

    let class = client
        .get_class(block_id, deploy.class_hash)
        .await
        .expect("getClass failed");

    let sierra = match class {
        starknet::core::types::ContractClass::Sierra(class) => class,
        // TODO
        starknet::core::types::ContractClass::Legacy(_) => panic!(),
    };

    let address = {
        let receipt = client.get_transaction_receipt(deploy.transaction_hash).await.expect("an existing receipt");
        
        // TODO Check the execution & finality status?
        match receipt {
            MaybePendingTransactionReceipt::Receipt(TransactionReceipt::DeployAccount(receipt)) => {
                receipt.contract_address
            },
            // TODO
            _ => panic!(),
        }
    };

    let entrypoint = sierra.entry_points_by_type.external.first().unwrap();
    
    // TODO Remove
    println!("{}", sierra.abi);
    // let abi: Vec<AbiEntry> = serde_json::from_str(&sierra.abi).unwrap();

    /* TODO
        The problem here is that we need to find a function we can call and valid parameters for that function.
        We could rely on the Argent & Braavos account contracts, maybe call `getBalance`. I just don't know how 
        to do that yet :)
    */
    client
        .call(
            FunctionCall {
                contract_address: address,
                entry_point_selector: entrypoint.selector,
                calldata: vec![],
            },
            block_id,
        )
        .await
        .unwrap();
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

#[tokio::test]
async fn test_get_block_with_tx_hashes() {
    let TestContext { client, block, block_id, extracted_value: () } =
        latest_block_context().await;

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
            block.transactions.iter().find_map(|transaction| {
                match transaction {
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
                    // TODO Make the match exhaustive to make dep upgrades more reliable.
                    _ => None,
                }
            })
        })
        .await;

    // TODO Assertions?
    client.get_class(block_id, extracted_value).await.expect("getClass failed");
}

#[tokio::test]
async fn test_get_class_at() {
    let TestContext { client, block: _, block_id, extracted_value } =
        context(|block| {
            block.transactions.iter().find_map(|transaction| {
                match transaction {
                    Transaction::DeployAccount(DeployAccountTransaction::V3(deploy)) => {
                        Some(deploy.transaction_hash)
                    }
                    // TODO Make the match exhaustive to make dep upgrades more reliable.
                    _ => None,
                }
            })
        })
        .await;
    let deploy_tx_hash = extracted_value;

    let receipt = match client.get_transaction_receipt(deploy_tx_hash).await.expect("the transaction to have a matching receipt") {
        MaybePendingTransactionReceipt::Receipt(TransactionReceipt::DeployAccount(receipt)) => receipt,
        // TODO
        _ => panic!()
    };

    client.get_class_at(block_id, receipt.contract_address).await.expect("getClass failed");
}

#[tokio::test]
async fn test_get_class_hash_at() {
    let TestContext { client, block: _, block_id, extracted_value } =
        context(|block| {
            block.transactions.iter().find_map(|transaction| {
                match transaction {
                    Transaction::DeployAccount(DeployAccountTransaction::V3(deploy)) => {
                        Some(deploy.transaction_hash)
                    }
                    // TODO Make the match exhaustive to make dep upgrades more reliable.
                    _ => None,
                }
            })
        })
        .await;
    let deploy_tx_hash = extracted_value;

    let receipt = match client.get_transaction_receipt(deploy_tx_hash).await.expect("the transaction to have a matching receipt") {
        MaybePendingTransactionReceipt::Receipt(TransactionReceipt::DeployAccount(receipt)) => receipt,
        // TODO
        _ => panic!()
    };

    client.get_class_hash_at(block_id, receipt.contract_address).await.expect("getClass failed");
}

#[ignore = "tested with test_get_transaction_receipt"]
#[test]
fn test_get_events() {}

#[tokio::test]
async fn test_get_nonce() {
    let TestContext { client, block: _, block_id, extracted_value } =
        context(|block| {
            // Browse the transaction in reverse order to make sure we got the latest nonce.
            block.transactions.iter().rev().find_map(|transaction| {
                match transaction {
                    Transaction::Invoke(InvokeTransaction::V1(invoke)) => {
                        Some((invoke.nonce, invoke.sender_address))
                    }
                    Transaction::Invoke(InvokeTransaction::V3(invoke)) => {
                        Some((invoke.nonce, invoke.sender_address))
                    }
                    // TODO Make the match exhaustive to make dep upgrades more reliable.
                    _ => None,
                }
            })
        })
        .await;
    let (original_nonce, sender) = extracted_value;
    // TODO Ugly conversion
    let original_nonce = u128::from_be_bytes(
        original_nonce.to_bytes_be().split_at(16).1.try_into().unwrap(),
    );

    let incremented_nonce =
        client.get_nonce(block_id, sender).await.expect("getNonce failed");
    // TODO Ugly conversion
    let incremented_nonce = u128::from_be_bytes(
        incremented_nonce.to_bytes_be().split_at(16).1.try_into().unwrap(),
    );

    assert_eq!(original_nonce + 1, incremented_nonce);
}

#[ignore = "this is still a draft"]
#[tokio::test]
async fn test_get_storage_at() {
    let TestContext { client, block: _, block_id, extracted_value } =
        context(|block| {
            // Browse the transaction in reverse order to make sure we got the latest nonce.
            block.transactions.iter().rev().find_map(|transaction| {
                match transaction {
                    Transaction::DeployAccount(DeployAccountTransaction::V3(deploy)) => {
                        Some(deploy.clone())
                    }
                    // TODO Make the match exhaustive to make dep upgrades more reliable.
                    _ => None,
                }
            })
        })
        .await;
    let deploy = extracted_value;
    
    let receipt = client.get_transaction_receipt(deploy.transaction_hash).await.expect("get_transaction_receipt failed");

    let deploy_receipt = match receipt {
        MaybePendingTransactionReceipt::Receipt(TransactionReceipt::DeployAccount(deploy_receipt)) => deploy_receipt,
        receipt => panic!("Unexpected receipt type: {:?}", receipt),
    };

    // TODO Event array size assertions
    client.get_storage_at(deploy_receipt.contract_address, deploy_receipt.events[0].keys[0], block_id).await.expect("getStorageAt failed");
}

#[tokio::test]
async fn test_get_transaction_by_block_id_and_index() {
    let TestContext { client, block, block_id, extracted_value: () } =
        latest_block_context().await;

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
            serde_json::to_string(&expected.transactions[transaction_index])
                .unwrap(),
        )
    }

    for transaction_index in 0..10 {
        check_transaction(&client, block_id, transaction_index, &block).await;
    }

    check_transaction(&client, block_id, block.transactions.len() - 1, &block)
        .await;
}

#[tokio::test]
async fn test_get_transaction_by_hash() {
    let TestContext { client, block, block_id: _, extracted_value: () } =
        latest_block_context().await;

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

#[tokio::test]
async fn test_get_transaction_receipt() {
    let TestContext { client, block, block_id: _, extracted_value: () } =
        latest_block_context().await;

    async fn check_transaction(
        client: &JsonRpcClient<HttpTransport>,
        block: &BlockWithTxs,
        expected: &Transaction,
    ) {
        let receipt = client.get_transaction_receipt(expected.transaction_hash()).await.expect("Failed to retrieve a specific transaction by its block ID and index");

        let receipt = match receipt {
            MaybePendingTransactionReceipt::Receipt(receipt) => receipt,
            MaybePendingTransactionReceipt::PendingReceipt(_) => {
                // TODO Make it a criteria?
                panic!("Pending receipt")
            }
        };

        // TODO Check the execution & finality status. If the tx was reverted, no event was emitted and 
        // we can't go any further with the assertions.

        let events = match receipt {
            starknet::core::types::TransactionReceipt::Invoke(receipt) => {
                receipt.events
            }
            starknet::core::types::TransactionReceipt::L1Handler(receipt) => {
                receipt.events
            }
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

        let block_id = BlockId::Number(block.block_number);

        let expected_events = client
            .get_block_events(block_id)
            .await
            .expect("Failed to retrieve the events");

        assert!(expected_events.len() >= events.len(), "getEvents should have returned at least as many events. GetEvents returned {} while we initially had {}", expected_events.len(), events.len());

        for actual in &events {
            // Find a matching event from the expected events vec.
            // We can't invert that relationship because getEvents might have returned more than we need here.
            if !expected_events.iter().any(|expected| {
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

#[tokio::test]
async fn test_get_transaction_status() {
    let TestContext { client, block, block_id: _, extracted_value: () } =
        latest_block_context().await;

    async fn check_transaction(
        client: &JsonRpcClient<HttpTransport>,
        transaction_hash: FieldElement,
    ) {
        // TODO No further assertion to make?
        let _status = client
            .get_transaction_status(transaction_hash)
            .await
            .expect("Failed to retrieve the transaction status");
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

// TODO starknet_call
// TODO starknet_getBalance

// TODO starknet_getProof
// TODO starknet_getStateRoot

// TODO starknet_getStateUpdate
// TODO starknet_getStorateAt

// starknet_getTransactionStatus
// TODO starknet_syncing

// TODO Look for other methods, I don't think the list is exhaustive.
