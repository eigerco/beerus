use beerus_api::gen::{
    Address, BlockId, BlockNumber, BlockTag, BroadcastedInvokeTxn, BroadcastedTxn, Felt, FunctionCall, GetBlockWithTxHashesResult, InvokeTxn, InvokeTxnV1, InvokeTxnV1Version, PriceUnit, Rpc
};

mod common;

#[tokio::test]
#[allow(non_snake_case)]
async fn test_specVersion() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let ret = ctx.client.specVersion().await?;
    assert_eq!(ret, "0.6.0");
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_chainId() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let ret = ctx.client.chainId().await?;
    assert_eq!(ret.as_ref(), "0x534e5f4d41494e");
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_blockHashAndNumber() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let ret = ctx.client.blockHashAndNumber().await?;
    assert!(*ret.block_number.as_ref() > 600612);
    assert!(!ret.block_hash.0.as_ref().is_empty());
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_blockNumber() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let ret = ctx.client.blockNumber().await?;
    assert!(*ret.as_ref() > 600612);
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_call() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let request = FunctionCall {
        calldata: Vec::default(),
        contract_address: Address(Felt::try_new(
            "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )?),
        entry_point_selector: Felt::try_new(
            "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60",
        )?,
    };

    let block_id =
        BlockId::BlockNumber { block_number: BlockNumber::try_new(33482)? };

    let ret = ctx.client.call(request, block_id).await?;
    assert_eq!(ret.len(), 1);
    assert_eq!(ret[0].as_ref(), "0x4574686572");
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_estimateFee() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let calldata = vec![
        "0x2",
        "0x57c4b510d66eb1188a7173f31cccee47b9736d40185da8144377b896d5ff3",
        "0x2f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
        "0x0",
        "0x1",
        "0x57c4b510d66eb1188a7173f31cccee47b9736d40185da8144377b896d5ff3",
        "0x2f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
        "0x1",
        "0x1",
        "0x2",
        "0x0",
        "0x1",
    ];
    let calldata: Result<Vec<Felt>, _> =
        calldata.into_iter().map(|felt| Felt::try_new(felt)).collect();

    let signature = vec![
        "0x42527ffe9912b338983cbed67e139cfcc26a4d8cf1d1c2a85e4125fdf5f59ed",
        "0x636147d06fefd02ed37984b752556d4b9aefdac1a50b3df0528ec7c201ad84b",
    ];
    let signature: Result<Vec<Felt>, _> =
        signature.into_iter().map(|felt| Felt::try_new(felt)).collect();

    let request = vec![
        BroadcastedTxn::BroadcastedInvokeTxn(
            BroadcastedInvokeTxn(
                InvokeTxn::InvokeTxnV1(
                    InvokeTxnV1 {
                        calldata: calldata?,
                        signature: signature?,
                        sender_address: Address(Felt::try_new("0x13e3ca9a377084c37dc7eacbd1d9f8c3e3733935bcbad887c32a0e213cd6fe0")?), 
                        max_fee: Felt::try_new("0x28ed6103d0000")?, 
                        nonce: Felt::try_new("0x1")?,
                        version: InvokeTxnV1Version::V0x1,
                        r#type: beerus_api::gen::InvokeTxnV1Type::Invoke, 
                    }
                )
            )
        )
    ];

    let simulation_flags = vec![];

    let block_id =
        BlockId::BlockNumber { block_number: BlockNumber::try_new(59999)? };

    let ret =
        ctx.client.estimateFee(request, simulation_flags, block_id).await?;
    assert_eq!(ret.len(), 1);
    assert_eq!(ret[0].overall_fee.as_ref(), "0x1abd7b153e472");
    assert_eq!(ret[0].gas_price.as_ref(), "0x67edb4f57");
    assert_eq!(ret[0].gas_consumed.as_ref(), "0x41de");
    assert!(matches!(ret[0].unit, PriceUnit::Wei));
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getBlockTransactionCount() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let block_id = BlockId::BlockTag(BlockTag::Latest);

    let ret = ctx.client.getBlockTransactionCount(block_id).await?;
    assert!(*ret.as_ref() > 0);
    Ok(())
}

#[tokio::test]
#[allow(non_snake_case)]
async fn test_getBlockWithTxHashes() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let block_id = BlockId::BlockTag(BlockTag::Latest);

    let ret = ctx.client.getBlockWithTxHashes(block_id).await?;
    assert!(matches!(ret, GetBlockWithTxHashesResult::BlockWithTxHashes(_)));
    let GetBlockWithTxHashesResult::BlockWithTxHashes(ret) = ret;
    assert!(ret.block_body_with_tx_hashes.transactions.len() > 0);
    Ok(())
}

/*
#[tokio::test]
#[allow(non_snake_case)]
async fn test_?() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };

    let ret = ctx.client.?().await?;
    println!("{ret:#?}");

    assert_eq!(ret, ?);
    Ok(())
}
*/

// TODO: getBlockWithTxs
// TODO: getClass
// TODO: getClassAt
// TODO: getClassHashAt
// TODO: getNonce
// TODO: getProof
// TODO: getStorageAt
// TODO: getTransactionByBlockIdAndIndex
// TODO: getTransactionByHash
// TODO: getTransactionReceipt
// TODO: getTransactionStatus
// TODO: syncing
