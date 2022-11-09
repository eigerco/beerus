pub mod cli;
use eyre::Result;
use helios::{
    client::{Client, ClientBuilder, FileDB},
    config::networks::Network,
};
use std::env;

pub async fn build_and_sync_helios_client() -> Result<Client<FileDB>> {
    let ethereum_execution_rpc_url = env::var("ETHEREUM_EXECUTION_RPC_URL")?;
    let ethereum_consensus_rpc_url = env::var("ETHEREUM_CONSENSUS_RPC_URL")?;
    let mut client = ClientBuilder::new()
        .network(Network::GOERLI)
        .consensus_rpc(&ethereum_consensus_rpc_url)
        .execution_rpc(&ethereum_execution_rpc_url)
        .build()?;
    client.start().await?;
    Ok(client)
}
