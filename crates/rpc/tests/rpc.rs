mod common;

use beerus_rpc::api::BeerusRpcServer;
use common::helper::create_mock_broadcasted_transaction;
use common::{setup_beerus_rpc, MOCK_BLOCK_NUMBER};
use rstest::rstest;
use starknet::core::types::{
    BlockId, BlockStatus, BlockTag, BlockWithTxHashes, EventFilter, FeeEstimate, FieldElement, FunctionCall,
    InvokeTransaction, InvokeTransactionV1, MaybePendingBlockWithTxHashes, SyncStatusType, Transaction,
};
use starknet::macros::felt;

#[rstest]
async fn starknet_block_number_ok() {
    let beerus_rpc = setup_beerus_rpc().await;
    let block_number = beerus_rpc.block_number().await.unwrap();
    assert_eq!(block_number, MOCK_BLOCK_NUMBER);
}

#[rstest]
async fn starknet_block_transaction_count_ok() {
    let beerus_rpc = setup_beerus_rpc().await;

    let transaction_count = beerus_rpc.get_block_transaction_count(BlockId::Tag(BlockTag::Latest)).await.unwrap();

    assert_eq!(transaction_count, 90);

    let transaction_count = beerus_rpc.get_block_transaction_count(BlockId::Number(MOCK_BLOCK_NUMBER)).await.unwrap();

    assert_eq!(transaction_count, 90);
}

#[rstest]
#[ignore]
async fn test_get_events() {
    let beerus_rpc = setup_beerus_rpc().await;
    todo!();
}

#[rstest]
async fn test_get_estimate_fee() {
    let beerus_rpc = setup_beerus_rpc().await;

    // let block_hash = BlockId::Hash(
    //     FieldElement::from_hex_be(
    //         "0x0147c4b0f702079384e26d9d34a15e7758881e32b219fc68c076b09d0be13f8c",
    //     )
    //         .unwrap(),
    // );
    let block_hash = BlockId::Number(MOCK_BLOCK_NUMBER);
    let (broadcasted_transaction, _json) = create_mock_broadcasted_transaction();

    let expected = FeeEstimate { gas_consumed: 0x1de6, gas_price: 0x5df32828e, overall_fee: 0xaf8f402b6194 };

    let actual = beerus_rpc.estimate_fee_single(broadcasted_transaction.clone(), block_hash).await.unwrap();

    assert_eq!(expected.gas_consumed, actual.gas_consumed);
    assert_eq!(expected.gas_price, actual.gas_price);
    assert_eq!(expected.overall_fee, actual.overall_fee);

    let actual =
        beerus_rpc.estimate_fee_single(broadcasted_transaction, BlockId::Number(MOCK_BLOCK_NUMBER + 1)).await.unwrap();

    assert_eq!(expected.gas_consumed, actual.gas_consumed);
    assert_eq!(expected.gas_price, actual.gas_price);
    assert_eq!(expected.overall_fee, actual.overall_fee);
}

#[rstest]
async fn starknet_syncing_ok() {
    let beerus_rpc = setup_beerus_rpc().await;

    let sync_status = beerus_rpc.syncing().await.unwrap();

    let expected_current_block = felt!("0x7f65231188b64236c1142ae6a894e826583725bef6b9172f46b6ad5f9d87469");
    let expected_starting_block = felt!("0x54cfb11a0c61c26b2e84c6d085a8317e5a1a437fa092d59a97564936afe2438");

    match sync_status {
        SyncStatusType::Syncing(result) => {
            assert_eq!(result.current_block_hash, expected_current_block);
            assert_eq!(result.current_block_num, 27468);
            assert_eq!(result.highest_block_hash, expected_current_block);
            assert_eq!(result.highest_block_num, 27468);
            assert_eq!(result.starting_block_hash, expected_starting_block);
            assert_eq!(result.starting_block_num, 24317);
        }
        SyncStatusType::NotSyncing => panic!("Syncing status should be true"),
    }
}

#[ignore]
#[rstest]
async fn starknet_block_hash_and_number() {
    let beerus_rpc = setup_beerus_rpc().await;
    let result = beerus_rpc.block_hash_and_number().await.unwrap();
    assert_eq!(result.block_number, MOCK_BLOCK_NUMBER);
    assert_eq!(
        result.block_hash,
        FieldElement::from_hex_be("0x63813d0cd71bf351dfe3217f9d2dcd8871cf4d56c0ffe3563980b3d02b6898d").unwrap()
    );
}
