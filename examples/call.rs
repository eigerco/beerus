use beerus::client::{Client, Http};
use beerus::config::Config;
use beerus::gen::{Address, Felt, FunctionCall};
use eyre::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let api_key = std::env::var("ALCHEMY_API_KEY")
        .context("ALCHEMY_API_KEY is missing")?;

    let config = Config {
        starknet_rpc: format!(
            "https://starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0_7/{api_key}"
        ),
        gateway_url: None,
        data_dir: "tmp".to_owned(),
    };

    let http = Http::new();
    let beerus = Client::new(&config, http).await?;

    let calldata = FunctionCall {
        contract_address: Address(Felt::try_new(
            "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )?),
        entry_point_selector: Felt::try_new(
            "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60",
        )?,
        calldata: vec![],
    };

    let state = beerus.get_state().await?;
    let res = beerus.execute(calldata, state)?;
    println!("{:#?}", res);

    Ok(())
}
