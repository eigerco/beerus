use beerus_api::gen::Rpc;

mod common;

#[tokio::test]
#[allow(non_snake_case)]
async fn test_specVersion() -> Result<(), common::Error> {
    let Some(ctx) = common::ctx().await else {
        return Ok(());
    };
    assert_eq!(ctx.client.specVersion().await?, "0.6.0");
    Ok(())
}
