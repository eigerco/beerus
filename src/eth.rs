use std::str::FromStr;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

use ethers::types::{Address, Bytes, SyncingStatus, H256};
use eyre::{Context, Result};
use helios::client::{Client, ClientBuilder};
use helios::config::checkpoints;
use helios::config::networks::Network;

#[cfg(target_arch = "wasm32")]
use helios::prelude::ConfigDB as DB;
#[cfg(not(target_arch = "wasm32"))]
use helios::prelude::FileDB as DB;
use helios::types::{BlockTag, CallOpts};

use crate::config::Config;

#[cfg(target_arch = "wasm32")]
mod setup {
    pub const MAINNET_CONSENSUS_RPC: &str =
        "http://127.0.0.1:3000/www.lightclientdata.org";
    pub const MAINNET_FALLBACK_RPC: &str =
        "http://127.0.0.1:3000/sync-mainnet.beaconcha.in";

    pub const SEPOLIA_CONSENSUS_RPC: &str =
        "http://127.0.0.1:3000/unstable.sepolia.beacon-api.nimbus.team";
    pub const SEPOLIA_FALLBACK_RPC: &str =
        "http://127.0.0.1:3000/sync-sepolia.beaconcha.in";
}

#[cfg(not(target_arch = "wasm32"))]
mod setup {
    pub const MAINNET_CONSENSUS_RPC: &str = "https://www.lightclientdata.org";
    pub const MAINNET_FALLBACK_RPC: &str = "https://sync-mainnet.beaconcha.in";

    pub const SEPOLIA_CONSENSUS_RPC: &str =
        "http://unstable.sepolia.beacon-api.nimbus.team";
    pub const SEPOLIA_FALLBACK_RPC: &str = "https://sync-sepolia.beaconcha.in";
}

const MAINNET_CC_ADDRESS: &str = "c662c410C0ECf747543f5bA90660f6ABeBD9C8c4";

const SEPOLIA_CC_ADDRESS: &str = "E2Bb56ee936fd6433DC0F6e7e3b8365C906AA057";

use setup::*;

pub type Helios = Client<DB>;

pub struct EthereumClient {
    pub helios: Client<DB>,
    starknet_core_contract_address: Address,
}

impl EthereumClient {
    pub async fn new(config: &Config, network: Network) -> Result<Self> {
        #[cfg(not(target_arch = "wasm32"))]
        let mut helios =
            get_client(&config.ethereum_rpc, network, &config.data_dir).await?;

        #[cfg(target_arch = "wasm32")]
        let mut helios = get_client(&config.ethereum_rpc, network).await?;

        helios.start().await.context("helios start")?;
        while let SyncingStatus::IsSyncing(sync) =
            helios.syncing().await.context("helios sync")?
        {
            tracing::info!(head=?sync.highest_block, "syncing");
            sleep(std::time::Duration::from_secs(1)).await;
        }

        Ok(Self {
            helios,
            starknet_core_contract_address: get_core_contract_address(
                &network,
            )?,
        })
    }

    pub async fn latest(&self) -> Result<(u64, H256)> {
        let block_number = self
            .helios
            .get_block_number()
            .await
            .context("helios:get_block_number")?
            .as_u64();
        let ret = self
            .helios
            .get_block_by_number(BlockTag::Number(block_number), false)
            .await
            .context("helios:get_block_by_number")?
            .map(|block| (block_number, block.hash))
            .ok_or_else(|| eyre::eyre!("Failed to fetch latest block"))?;
        Ok(ret)
    }

    pub async fn starknet_state(&self) -> Result<(u64, H256, H256)> {
        let (number, _) = self.latest().await?;
        let tag = BlockTag::Number(number);

        #[cfg(target_arch = "wasm32")]
        let now = Instant::now();

        let data = 0x35befa5du32.to_be_bytes(); // keccak("stateBlockNumber()")
        let block_number: [u8; 32] = self
            .call(&data, tag)
            .await
            .context("helios: state block number")?;
        let block_number: [u8; 8] = block_number[24..].try_into().unwrap();
        let block_number = u64::from_be_bytes(block_number);

        #[cfg(target_arch = "wasm32")]
        log_latency("stateBlockNumber", &now);

        #[cfg(target_arch = "wasm32")]
        let now = Instant::now();

        let data = 0x382d83e3u32.to_be_bytes(); // keccak("stateBlockHash()")
        let block_hash: H256 =
            self.call(&data, tag).await.context("helios: state block hash")?;

        #[cfg(target_arch = "wasm32")]
        log_latency("stateBlockHash", &now);

        #[cfg(target_arch = "wasm32")]
        let now = Instant::now();

        let data = 0x9588eca2u32.to_be_bytes(); // keccak("stateRoot()")"
        let root: H256 =
            self.call(&data, tag).await.context("helios: state root")?;

        #[cfg(target_arch = "wasm32")]
        log_latency("stateRoot", &now);

        tracing::debug!(block_number, ?block_hash, ?root, "starknet state");

        Ok((block_number, block_hash, root))
    }

    async fn call<const N: usize, T: From<[u8; N]>>(
        &self,
        data: &[u8],
        tag: BlockTag,
    ) -> Result<T> {
        let opts = CallOpts {
            from: None,
            to: Some(self.starknet_core_contract_address),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(Bytes::from(data.to_vec())),
        };

        let ret = self.helios.call(&opts, tag).await.context("helios: call")?;

        if ret.len() != N {
            eyre::bail!("Expected {} bytes but got {}!", N, ret.len());
        }
        let sized: [u8; N] = ret.try_into().unwrap();
        Ok(sized.into())
    }
}

#[cfg(target_arch = "wasm32")]
fn log_latency(method: &str, instant: &Instant) {
    let millis = instant.elapsed().as_millis();
    let message = format!("call to {method} completed in {millis} ms");
    web_sys::console::log_1(&message.into());
}

async fn get_client(
    rpc: &str,
    network: Network,
    #[cfg(not(target_arch = "wasm32"))] data_dir: &str,
) -> Result<Client<DB>> {
    let consensus_rpc =
        get_consensus_rpc(&network).context("consensus rpc url")?;
    let fallback_rpc =
        get_fallback_address(&network).context("fallback rpc url")?;
    let checkpoint = get_checkpoint(&network).await.context("checkpoint")?;

    let builder = ClientBuilder::new()
        .network(network)
        .consensus_rpc(consensus_rpc)
        .execution_rpc(rpc)
        .checkpoint(&checkpoint)
        .load_external_fallback()
        .fallback(fallback_rpc);

    #[cfg(not(target_arch = "wasm32"))]
    let builder = builder.data_dir(data_dir.into());

    builder.build()
}

fn get_core_contract_address(network: &Network) -> Result<Address> {
    match network {
        Network::MAINNET => Ok(Address::from_str(MAINNET_CC_ADDRESS)?),
        Network::SEPOLIA => Ok(Address::from_str(SEPOLIA_CC_ADDRESS)?),
        network => eyre::bail!("unsupported network: {network:?}"),
    }
}

fn get_consensus_rpc(network: &Network) -> Result<&str> {
    match network {
        Network::MAINNET => Ok(MAINNET_CONSENSUS_RPC),
        Network::SEPOLIA => Ok(SEPOLIA_CONSENSUS_RPC),
        network => eyre::bail!("unsupported network: {network:?}"),
    }
}

fn get_fallback_address(network: &Network) -> Result<&str> {
    match network {
        Network::MAINNET => Ok(MAINNET_FALLBACK_RPC),
        Network::SEPOLIA => Ok(SEPOLIA_FALLBACK_RPC),
        network => eyre::bail!("unsupported network: {network:?}"),
    }
}

async fn get_checkpoint(network: &Network) -> Result<String> {
    if !matches!(network, Network::MAINNET | Network::SEPOLIA) {
        eyre::bail!("unsupported network: {:?}", network);
    }

    let cf = checkpoints::CheckpointFallback::new().build().await?;
    let checkpoint = cf.fetch_latest_checkpoint(network).await?;
    Ok(format!("{checkpoint:x}"))
}

async fn sleep(delay: std::time::Duration) {
    #[cfg(not(target_arch = "wasm32"))]
    tokio::time::sleep(delay).await;

    #[cfg(target_arch = "wasm32")]
    {
        let millis = delay.as_millis() as u32;
        gloo_timers::future::TimeoutFuture::new(millis).await;
    }
}


#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    #[test]
    fn test_get_core_contract_address() {
        assert_eq!(
            get_core_contract_address(&Network::MAINNET).unwrap(),
            Address::from_str(MAINNET_CC_ADDRESS).unwrap(),
        );

        assert_eq!(
            get_core_contract_address(&Network::SEPOLIA).unwrap(),
            Address::from_str(SEPOLIA_CC_ADDRESS).unwrap(),
        );

        assert_eq!(
            get_core_contract_address(&Network::GOERLI).unwrap_err().to_string(),
            "unsupported network: GOERLI".to_string(),
        );

        assert_eq!(
            get_core_contract_address(&Network::HOLESKY).unwrap_err().to_string(),
            "unsupported network: HOLESKY".to_string(),
        );
    }

    #[test]
    fn test_get_consensus_rpc() {
        assert_eq!(
            get_consensus_rpc(&Network::MAINNET).unwrap(),
            MAINNET_CONSENSUS_RPC,
        );

        assert_eq!(
            get_consensus_rpc(&Network::SEPOLIA).unwrap(),
            SEPOLIA_CONSENSUS_RPC,
        );

        assert_eq!(
            get_consensus_rpc(&Network::GOERLI).unwrap_err().to_string(),
            "unsupported network: GOERLI".to_string(),
        );

        assert_eq!(
            get_consensus_rpc(&Network::HOLESKY).unwrap_err().to_string(),
            "unsupported network: HOLESKY".to_string(),
        );
    }

    #[test]
    fn test_get_fallback_address() {
        assert_eq!(
            get_fallback_address(&Network::MAINNET).unwrap(),
            MAINNET_FALLBACK_RPC,
        );

        assert_eq!(
            get_fallback_address(&Network::SEPOLIA).unwrap(),
            SEPOLIA_FALLBACK_RPC,
        );

        assert_eq!(
            get_fallback_address(&Network::GOERLI).unwrap_err().to_string(),
            "unsupported network: GOERLI".to_string(),
        );

        assert_eq!(
            get_fallback_address(&Network::HOLESKY).unwrap_err().to_string(),
            "unsupported network: HOLESKY".to_string(),
        );
    }

    #[tokio::test]
    async fn test_get_checkpoint() {
        assert_eq!(
            get_checkpoint(&Network::GOERLI).await.unwrap_err().to_string(),
            "unsupported network: GOERLI".to_string(),
        );

        assert_eq!(
            get_checkpoint(&Network::HOLESKY).await.unwrap_err().to_string(),
            "unsupported network: HOLESKY".to_string(),
        );

        // TODO: it could utilise mock server
        get_checkpoint(&Network::MAINNET).await.unwrap();
        get_checkpoint(&Network::SEPOLIA).await.unwrap();
    }

    #[tokio::test]
    async fn test_sleep() {
        let start = std::time::Instant::now();
        sleep(Duration::from_millis(200)).await;
        assert!(start.elapsed().as_millis() >= 200);
    }

    #[tokio::test]
    async fn test_get_client() {
        let rpc = "https://rpc/v2";
        let data_dir = "tmp";

        assert_eq!(
            get_client(&rpc, Network::GOERLI, data_dir).await.err().unwrap().to_string(),
            "consensus rpc url",
        );

        // sucesfully build client
        get_client(&rpc, Network::SEPOLIA, data_dir).await.unwrap();

    }
}