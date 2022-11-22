use crate::config::Config;
use async_trait::async_trait;
use eyre::Result;
use helios::client::{Client, ClientBuilder, FileDB};

use super::starknet::StarkNetLightClient;

pub enum SyncStatus {
    NotSynced,
    Syncing,
    Synced,
}

#[async_trait]
pub trait Beerus {
    async fn start(&mut self) -> Result<()>;
    fn sync_status(&self) -> SyncStatus;
}

/// Beerus Light Client service.
pub struct BeerusLightClient {
    /// Ethereum light client.
    pub ethereum_lightclient: Client<FileDB>,
    /// StarkNet light client.
    pub starknet_lightclient: StarkNetLightClient,
}

impl BeerusLightClient {
    /// Create a new Beerus Light Client service.
    pub fn new(config: &Config) -> Result<Self> {
        let ethereum_network = config.ethereum_network()?;
        // Build the Ethereum light client.
        let ethereum_lightclient = ClientBuilder::new()
            .network(ethereum_network)
            .consensus_rpc(&config.ethereum_consensus_rpc)
            .execution_rpc(&config.ethereum_execution_rpc)
            .build()?;
        // Build the StarkNet light client.
        let starknet_lightclient = StarkNetLightClient::new(config)?;
        Ok(Self {
            ethereum_lightclient,
            starknet_lightclient,
        })
    }
}

#[async_trait]
impl Beerus for BeerusLightClient {
    /// Start Beerus light client and synchronize with Ethereum and StarkNet.
    async fn start(&mut self) -> Result<()> {
        // Start the Ethereum light client.
        self.ethereum_lightclient.start().await?;
        // Start the StarkNet light client.
        self.starknet_lightclient.start().await?;
        Ok(())
    }

    fn sync_status(&self) -> SyncStatus {
        todo!()
    }
}
