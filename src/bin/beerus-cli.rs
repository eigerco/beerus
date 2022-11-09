use std::{env, str::FromStr};

use ethers::{types::Address, utils};
use eyre::Result;
use helios::{client::ClientBuilder, config::networks::Network, types::BlockTag};

#[tokio::main]
async fn main() -> Result<()> {
    let ethereum_execution_rpc_url = env::var("ETHEREUM_EXECUTION_RPC_URL")?;
    let ethereum_consensus_rpc_url = env::var("ETHEREUM_CONSENSUS_RPC_URL")?;
    let mut helios_client = ClientBuilder::new()
        .network(Network::GOERLI)
        .consensus_rpc(&ethereum_consensus_rpc_url)
        .execution_rpc(&ethereum_execution_rpc_url)
        .build()?;

    helios_client.start().await?;

    let head_block_num = helios_client.get_block_number().await?;
    let addr = Address::from_str("0x00000000219ab540356cBB839Cbe05303d7705Fa")?;
    let block = BlockTag::Latest;
    let balance = helios_client.get_balance(&addr, block).await?;

    println!("synced up to block: {}", head_block_num);
    println!(
        "balance of deposit contract: {}",
        utils::format_ether(balance)
    );

    Ok(())
}
