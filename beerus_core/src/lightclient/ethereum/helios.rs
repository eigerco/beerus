use helios::client::{Client, ClientBuilder, FileDB};

use crate::config::Config;

use super::ethereum::EthereumLightClient;

/// Helios implementation of `EthereumLightClient`.
pub struct HeliosLightClient {
    /// The wrapped Helios client.
    pub helios_light_client: Client<FileDB>,
}

/// Implementation of `EthereumLightClient` for Helios.
impl EthereumLightClient for HeliosLightClient {
    async fn start(&mut self) -> eyre::Result<()> {
        // Start the Helios light client.
        self.helios_light_client.start().await
    }

    async fn call(
        &self,
        opts: &helios::types::CallOpts,
        block: helios::types::BlockTag,
    ) -> eyre::Result<Vec<u8>> {
        // Wrap the Helios call.
        self.helios_light_client.call(opts, block).await
    }

    async fn get_balance(
        &self,
        address: &ethers::types::Address,
        block: helios::types::BlockTag,
    ) -> eyre::Result<primitive_types::U256> {
        self.helios_light_client.get_balance(address, block).await
    }
}

/// HeliosLightClient non-trait functions.
impl HeliosLightClient {
    /// Create a new HeliosLightClient.
    pub fn new(config: &Config) -> eyre::Result<Self> {
        // Build the Helios wrapped light client.
        let helios_light_client = ClientBuilder::new()
            .network(config.ethereum_network()?)
            .consensus_rpc(&config.ethereum_consensus_rpc)
            .execution_rpc(&config.ethereum_execution_rpc)
            .checkpoint("c93123ff83f8bd1fdbe3a0dbd8cfa3b491a3eda66ecd49fa21c4fd82985ed73b")
            .build()?;
        Ok(Self {
            helios_light_client,
        })
    }
}
