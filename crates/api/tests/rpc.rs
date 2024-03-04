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
