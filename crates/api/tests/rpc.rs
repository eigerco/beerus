use beerus_api::gen::{
    Address, BlockId, BlockNumber, BroadcastedInvokeTxn, BroadcastedTxn, Felt,
    FunctionCall, InvokeTxn, InvokeTxnV1, InvokeTxnV1Version, PriceUnit, Rpc,
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
