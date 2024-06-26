use std::str::FromStr;
use std::sync::Arc;

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
use tokio::sync::RwLock;

use crate::config::Config;

const MAINNET_CC_ADDRESS: &str = "c662c410C0ECf747543f5bA90660f6ABeBD9C8c4";
const MAINNET_CONSENSUS_RPC: &str = "http://127.0.0.1:3000/www.lightclientdata.org";
const MAINNET_FALLBACK_RPC: &str = "http://127.0.0.1:3000/sync-mainnet.beaconcha.in";

const SEPOLIA_CC_ADDRESS: &str = "E2Bb56ee936fd6433DC0F6e7e3b8365C906AA057";
const SEPOLIA_CONSENSUS_RPC: &str = "http://127.0.0.1:3000/unstable.sepolia.beacon-api.nimbus.team";
const SEPOLIA_FALLBACK_RPC: &str = "http://127.0.0.1:3000/sync-sepolia.beaconcha.in";

pub type Helios = Client<DB>;

pub struct EthereumClient {
    helios: Arc<RwLock<Client<DB>>>,
    starknet_core_contract_address: Address,
}

impl EthereumClient {
    pub async fn new(config: &Config) -> Result<Self> {

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"beerus: eth::new".into());

        let helios = get_client(config).await?;

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"beerus: eth::new done".into());

        Ok(Self {
            helios: Arc::new(RwLock::new(helios)),
            starknet_core_contract_address: get_core_contract_address(config)?,
        })
    }

    pub async fn start(&self) -> Result<()> {
        let mut helios = self.helios.write().await;
        helios.start().await.context("helios start")?;

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"beerus: helios started".into());

        let delay = std::time::Duration::from_secs(1);
        while let SyncingStatus::IsSyncing(sync) =
            helios.syncing().await.context("helios sync")?
        {
            #[cfg(target_arch = "wasm32")]
            {
                let js: wasm_bindgen::JsValue = sync.highest_block.low_u64().into();
                web_sys::console::log_2(&"beerus: helios syncing".into(), &js); 
            }

            tracing::info!(head=?sync.highest_block, "syncing");

            #[cfg(not(target_arch = "wasm32"))]
            tokio::time::sleep(delay).await;

            #[cfg(target_arch = "wasm32")]
            let _ = wasm_timer::Delay::new(delay).await;
        }

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"beerus: helios sync done".into());

        Ok(())
    }

    pub fn helios(&self) -> Arc<RwLock<Helios>> {
        self.helios.clone()
    }

    pub async fn latest(&self) -> Result<(u64, H256)> {
        let block_number = self
            .helios
            .read()
            .await
            .get_block_number()
            .await
            .context("helios:get_block_number")?
            .as_u64();
        let ret = self
            .helios
            .read()
            .await
            .get_block_by_number(BlockTag::Number(block_number), false)
            .await?
            .map(|block| (block_number, block.hash))
            .ok_or_else(|| eyre::eyre!("Failed to fetch latest block"))?;
        Ok(ret)
    }

    pub async fn starknet_state(&self) -> Result<(u64, H256, H256)> {
        let (number, _) = self.latest().await?;
        let tag = BlockTag::Number(number);

        let data = 0x35befa5du32.to_be_bytes(); // keccak("stateBlockNumber()")
        let block_number: [u8; 32] = self
            .call(&data, tag)
            .await
            .context("helios: state block number")?;
        let block_number: [u8; 8] = block_number[24..].try_into().unwrap();
        let block_number = u64::from_be_bytes(block_number);

        let data = 0x382d83e3u32.to_be_bytes(); // keccak("stateBlockHash()")
        let block_hash: H256 =
            self.call(&data, tag).await.context("helios: state block hash")?;

        let data = 0x9588eca2u32.to_be_bytes(); // keccak("stateRoot()")"
        let root: H256 =
            self.call(&data, tag).await.context("helios: state root")?;

        tracing::info!(block_number, ?block_hash, ?root, "starknet state");

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

        let ret = self
            .helios
            .read()
            .await
            .call(&opts, tag)
            .await
            .context("helios: call")?;

        if ret.len() != N {
            eyre::bail!("Expected {} bytes but got {}!", N, ret.len());
        }
        let sized: [u8; N] = ret.try_into().unwrap();
        Ok(sized.into())
    }
}

async fn get_client(config: &Config) -> Result<Client<DB>> {

    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&"beerus: eth::client".into());

    let consensus_rpc =
        get_consensus_rpc(config).context("consensus rpc url")?;
    let fallback_rpc =
        get_fallback_address(config).context("fallback rpc url")?;
    /*
    [Attempted fix of checkpoint resolution: query service via proxy]

    Helios: config/src/checkpoints.rs:construct_url needs patching as below.
    Commented lines need to be added without comments in order to make sure
    that requests from browser are actually served by the CORS proxy.

    pub fn construct_url(endpoint: &str) -> String {
        // let endpoint = endpoint.strip_prefix("https://").unwrap_or(endpoint);
        // let endpoint = format!("http://127.0.0.1:3000/{endpoint}");
        format!("{endpoint}/checkpointz/v1/beacon/slots")
    }

     */
    // TODO: FIXME: resolving checkpoint does not work in WebAssembly
    // (Checkpoint needs to be resolved before building the client)
    // let checkpoint = get_checkpoint(config).await.context("checkpoint")?;
    let checkpoint = "3f220067597bd88a10b373d9dc5ae1d03b1bc25930bbcc26f44bd19aa6b1bd09"; // mainnet
    // let checkpoint = "531710292412f11227529591ded4b0c4bfde4cf894fd9437458febbb39f9485b"; // sepolia
    eprintln!("checkpoint: {checkpoint}");

    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_2(&"beerus: eth::client checkpoint".into(), &checkpoint.clone().into());

    let builder = ClientBuilder::new()
        .network(config.network)
        .consensus_rpc(consensus_rpc)
        .execution_rpc(&config.eth_execution_rpc)
        .checkpoint(&checkpoint)
        .load_external_fallback()
        .fallback(fallback_rpc);

    #[cfg(not(target_arch = "wasm32"))]
    let builder = builder.data_dir(config.data_dir.clone());

    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&"beerus: eth::client done".into());

    builder.build()
}

fn get_core_contract_address(config: &Config) -> Result<Address> {
    match config.network {
        Network::MAINNET => Ok(Address::from_str(MAINNET_CC_ADDRESS)?),
        Network::SEPOLIA => Ok(Address::from_str(SEPOLIA_CC_ADDRESS)?),
        network => eyre::bail!("unsupported network: {network:?}"),
    }
}

fn get_consensus_rpc(config: &Config) -> Result<&str> {
    match config.network {
        Network::MAINNET => Ok(MAINNET_CONSENSUS_RPC),
        Network::SEPOLIA => Ok(SEPOLIA_CONSENSUS_RPC),
        network => eyre::bail!("unsupported network: {network:?}"),
    }
}

fn get_fallback_address(config: &Config) -> Result<&str> {
    match config.network {
        Network::MAINNET => Ok(MAINNET_FALLBACK_RPC),
        Network::SEPOLIA => Ok(SEPOLIA_FALLBACK_RPC),
        network => eyre::bail!("unsupported network: {network:?}"),
    }
}

#[allow(dead_code)] // unusable in wasm unfortunately
async fn get_checkpoint(config: &Config) -> Result<String> {
    if !matches!(config.network, Network::MAINNET | Network::SEPOLIA) {
        eyre::bail!("unsupported network: {:?}", config.network);
    }

    let cf = checkpoints::CheckpointFallback::new().build().await?;
    let checkpoint = cf.fetch_latest_checkpoint(&config.network).await?;
    Ok(format!("{checkpoint:x}"))
}
