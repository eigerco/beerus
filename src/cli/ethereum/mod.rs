use crate::build_and_sync_helios_client;
use ethers::{types::Address, utils};
use eyre::Result;
use helios::types::BlockTag;
use std::str::FromStr;

pub async fn query_balance(address: String) -> Result<()> {
    println!("querying balance for address: {}", address);

    let client = build_and_sync_helios_client().await?;
    let head_block_num = client.get_block_number().await?;
    let addr = Address::from_str(&address)?;
    let block = BlockTag::Latest;
    let balance = client.get_balance(&addr, block).await?;

    println!("synced up to block: {}", head_block_num);
    println!("balance of address: {}", utils::format_ether(balance));
    Ok(())
}
