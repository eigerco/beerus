pub mod cli;
pub mod config;

use config::Config;
use eyre::Result;
use helios::client::{Client, ClientBuilder, FileDB};

pub async fn sync_ethereum_light_client(config: &Config) -> Result<Client<FileDB>> {
    let ethereum_network = config.ethereum_network()?;
    let mut client = ClientBuilder::new()
        .network(ethereum_network)
        .consensus_rpc(&config.ethereum_consensus_rpc)
        .execution_rpc(&config.ethereum_execution_rpc)
        .build()?;
    client.start().await?;
    Ok(client)
}
