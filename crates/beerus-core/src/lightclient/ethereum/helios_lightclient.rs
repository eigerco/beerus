use crate::config::{Config, DEFAULT_STARKNET_CORE_CONTRACT_ADDRESS};
use async_trait::async_trait;
use ethers::types::{Address, BlockNumber, Filter, Log, Topic, Transaction, H256, U256};
use eyre::{eyre, Result};
use helios::{
    client::{Client, ClientBuilder, FileDB},
    types::{BlockTag, CallOpts, ExecutionBlock},
};
use std::{primitive::u64, str::FromStr};

use super::EthereumLightClient;

/// Helios implementation of `EthereumLightClient`.
pub struct HeliosLightClient {
    /// The wrapped Helios client.
    pub helios_light_client: Client<FileDB>,
}

/// Implementation of `EthereumLightClient` for Helios.
#[async_trait]
impl EthereumLightClient for HeliosLightClient {
    async fn start(&mut self) -> eyre::Result<()> {
        // Start the Helios light client.
        self.helios_light_client.start().await
    }

    async fn call(&self, opts: &CallOpts, block: BlockTag) -> eyre::Result<Vec<u8>> {
        // Wrap the Helios call.
        self.helios_light_client.call(opts, block).await
    }

    async fn send_raw_transaction(&self, bytes: &[u8]) -> eyre::Result<ethers::types::H256> {
        self.helios_light_client.send_raw_transaction(bytes).await
    }

    async fn get_balance(
        &self,
        address: &Address,
        block: BlockTag,
    ) -> eyre::Result<ethers::types::U256> {
        self.helios_light_client.get_balance(address, block).await
    }

    async fn get_nonce(&self, address: &Address, block: BlockTag) -> Result<u64> {
        self.helios_light_client.get_nonce(address, block).await
    }

    async fn get_block_number(&self) -> Result<u64> {
        self.helios_light_client.get_block_number().await
    }

    async fn chain_id(&self) -> u64 {
        self.helios_light_client.chain_id().await
    }

    async fn get_code(&self, address: &Address, block: BlockTag) -> Result<Vec<u8>> {
        self.helios_light_client.get_code(address, block).await
    }

    async fn get_transaction_count(&self, address: &Address, block: BlockTag) -> Result<u64> {
        // TODO: Rename after it has been renamed https://github.com/a16z/helios/pull/166#issuecomment-1379587761
        self.helios_light_client.get_nonce(address, block).await
    }

    async fn get_block_transaction_count_by_number(&self, block: BlockTag) -> Result<u64> {
        self.helios_light_client
            .get_block_transaction_count_by_number(block)
            .await
    }

    async fn get_block_transaction_count_by_hash(&self, hash: &[u8]) -> Result<u64> {
        let hash = hash.to_vec();
        self.helios_light_client
            .get_block_transaction_count_by_hash(&hash)
            .await
    }

    async fn get_transaction_by_hash(&self, tx_hash: &H256) -> Result<Option<Transaction>> {
        self.helios_light_client
            .get_transaction_by_hash(tx_hash)
            .await
    }
    async fn get_gas_price(&self) -> Result<U256> {
        self.helios_light_client.get_gas_price().await
    }

    async fn estimate_gas(&self, opts: &CallOpts) -> Result<u64> {
        self.helios_light_client.estimate_gas(opts).await
    }
    async fn get_block_by_hash(
        &self,
        hash: &[u8],
        full_tx: bool,
    ) -> eyre::Result<Option<ExecutionBlock>> {
        let hash: Vec<u8> = Vec::from(hash);
        self.helios_light_client
            .get_block_by_hash(&hash, full_tx)
            .await
    }
    async fn get_priority_fee(&self) -> Result<U256> {
        self.helios_light_client.get_priority_fee().await
    }

    async fn get_block_by_number(
        &self,
        block: BlockTag,
        full_tx: bool,
    ) -> eyre::Result<Option<ExecutionBlock>> {
        self.helios_light_client
            .get_block_by_number(block, full_tx)
            .await
    }

    async fn get_logs(
        &self,
        from_block: &Option<String>,
        to_block: &Option<String>,
        address: &Option<String>,
        topics: &Option<Vec<String>>,
        block_hash: &Option<String>,
    ) -> Result<Vec<Log>> {
        self.helios_light_client
            .get_logs(&build_logs_filter(
                from_block, to_block, address, topics, block_hash,
            )?)
            .await
    }

    /// Get the StarkNet state root.
    async fn starknet_state_root(&self) -> Result<U256> {
        // Get the StarkNet core contract address.
        let starknet_core_contract_address =
            Address::from_str(DEFAULT_STARKNET_CORE_CONTRACT_ADDRESS)?;

        // Corresponds to the StarkNet core contract function `stateRoot`.
        // The function signature is `stateRoot() -> (uint256)`.
        // The function selector is `0x95d8ecA2`.
        let data = vec![0x95, 0x88, 0xec, 0xa2];

        // Build the call options.
        let call_opts = CallOpts {
            from: None,
            to: starknet_core_contract_address,
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let starknet_root = self.call(&call_opts, BlockTag::Latest).await?;

        // Convert the response bytes to a U256.
        let starknet_root = U256::from_big_endian(&starknet_root);

        Ok(starknet_root)
    }

    /// Get the StarkNet last proven block number.
    /// This function is used to get the last proven block number of the StarkNet network.
    ///
    /// # Returns
    /// `Ok(U256)` if the operation was successful.
    /// `Err(eyre::Report)` if the operation failed.
    async fn starknet_last_proven_block(&self) -> Result<U256> {
        // Get the StarkNet core contract address.
        let starknet_core_contract_address =
            Address::from_str(DEFAULT_STARKNET_CORE_CONTRACT_ADDRESS)?;

        let data = vec![53, 190, 250, 93];

        // Build the call options.
        let call_opts = CallOpts {
            from: None,
            to: starknet_core_contract_address,
            gas: None,
            gas_price: None,
            value: None,
            data: Some(data),
        };

        // Call the StarkNet core contract.
        let starknet_root = self.call(&call_opts, BlockTag::Latest).await?;

        // Convert the response bytes to a U256.
        let starknet_root = U256::from_big_endian(&starknet_root);

        Ok(starknet_root)
    }
}

/// HeliosLightClient non-trait functions.
impl HeliosLightClient {
    /// Create a new HeliosLightClient.
    pub async fn new(config: Config) -> eyre::Result<Self> {
        // Build the Helios wrapped light client.
        let helios_light_client: Client<FileDB> = ClientBuilder::new()
            .network(config.ethereum_network()?)
            .consensus_rpc(config.ethereum_consensus_rpc.as_str())
            .execution_rpc(config.ethereum_execution_rpc.as_str())
            .load_external_fallback()
            .data_dir(config.data_dir.unwrap())
            .build()?;

        Ok(Self {
            helios_light_client,
        })
    }
}

fn build_logs_filter(
    from_block: &Option<String>,
    to_block: &Option<String>,
    address: &Option<String>,
    topics: &Option<Vec<String>>,
    block_hash: &Option<String>,
) -> Result<Filter> {
    let mut filter = Filter::new();
    match (from_block, to_block, block_hash) {
        (Some(from), Some(to), None) => {
            let from_block = BlockNumber::from_str(from)
                .map_err(|err| eyre!("Non valid format for from_block: {}", err))?;
            let to_block = BlockNumber::from_str(to)
                .map_err(|err| eyre!("Non valid format for from_block: {}", err))?;
            filter = filter.select(from_block..to_block);
        }
        (Some(from), None, None) => {
            let from_block = BlockNumber::from_str(from)
                .map_err(|err| eyre!("Non valid format for from_block: {}", err))?;
            let to_block = BlockNumber::Latest;
            filter = filter.select(from_block..to_block);
        }
        (None, Some(to), None) => {
            let from_block = BlockNumber::Latest;
            let to_block = BlockNumber::from_str(to)
                .map_err(|err| eyre!("Non valid format for to_block: {}", err))?;
            filter = filter.select(from_block..to_block);
        }
        (None, None, Some(ref hash)) => {
            filter = filter.at_block_hash(H256::from_str(hash)?);
        }
        (None, None, _) => {
            let from_block = BlockNumber::Latest;
            let to_block = BlockNumber::Latest;
            filter = filter.select(from_block..to_block);
        }
        _ => {
            let error_msg = concat!(
                "Non valid combination of from_block, to_block and blockhash. ",
                "If you want to filter blocks, then ",
                "you can only use either from_block and to_block or blockhash, not both",
            );
            Err(eyre!(error_msg))?
        }
    }
    if let Some(address) = address {
        filter = filter.address(ethers::types::H160::from_str(address)?);
    }

    if let Some(topics) = topics {
        for (index, topic) in topics.iter().enumerate() {
            *(filter
                .topics
                .get_mut(index)
                .ok_or(eyre!("Too many topics, expected 4 at most"))?) =
                Some(Topic::from(H256::from_str(topic)?))
        }
    }
    Ok(filter)
}
