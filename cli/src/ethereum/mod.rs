use core::config::Config;
use core::sync_ethereum_light_client;
use ethers::{types::Address, utils};
use eyre::Result;
use helios::types::BlockTag;
use std::str::FromStr;

pub async fn query_balance(config: &Config, address: String) -> Result<()> {
    let client = sync_ethereum_light_client(config).await?;
    let addr = Address::from_str(&address)?;
    let block = BlockTag::Latest;
    let balance = client.get_balance(&addr, block).await?;
    let balance_in_eth = utils::format_units(balance, "ether")?;
    println!("{} eth", balance_in_eth);
    Ok(())
}
