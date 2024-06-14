use std::path::PathBuf;

use beerus::config::Config;
use beerus::gen::{Address, BlockNumber, Felt, FunctionCall};
use beerus::{client::Client, gen::BlockId};
use eyre::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let api_key = std::env::var("ALCHEMY_API_KEY")
        .context("ALCHEMY_API_KEY is missing")?;

    let config = Config {
        network: helios::config::networks::Network::MAINNET,
        eth_execution_rpc: format!(
            "https://eth-mainnet.g.alchemy.com/v2/{api_key}"
        ),
        starknet_rpc: format!(
            "https://starknet-mainnet.g.alchemy.com/starknet/version/rpc/v0.6/{api_key}"
        ),
        data_dir: PathBuf::from("tmp"),
        poll_secs: 300,
        rpc_addr: ([127, 0, 0, 1], 3030).into(),
    };

    let beerus = Client::new(&config).await?;
    beerus.start().await?;

    let block_id =
        BlockId::BlockNumber { block_number: BlockNumber::try_new(33482)? };
    let calldata = FunctionCall {
        contract_address: Address(Felt::try_new(
            "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        )?),
        entry_point_selector: Felt::try_new(
            "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60",
        )?,
        calldata: vec![],
    };

    let res = beerus.call_starknet(calldata, block_id).await?;
    println!("{:#?}", res);

    Ok(())
}
