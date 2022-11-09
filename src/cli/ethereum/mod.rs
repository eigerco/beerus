use crate::{build_and_sync_helios_client, config::Config};
use ethers::{types::Address, utils};
use eyre::Result;
use helios::types::BlockTag;
use std::str::FromStr;

pub async fn query_balance(config: &Config, address: String) -> Result<()> {
    let client = build_and_sync_helios_client(config).await?;
    let addr = Address::from_str(&address)?;
    let block = BlockTag::Latest;
    let balance = client.get_balance(&addr, block).await?;
    let balance_in_eth = utils::format_units(balance, "ether")?;
    println!("{} ETH", balance_in_eth);
    Ok(())
}
