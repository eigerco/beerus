use std::str::FromStr;

use alloy_primitives::U256;
use anyhow::Result;
use katana_core::backend::config::{Environment, StarknetConfig};
use katana_core::constants::DEFAULT_SEQUENCER_ADDRESS;
#[allow(deprecated)]
use katana_core::sequencer::SequencerConfig;
use katana_node::{start, NodeHandle};
use katana_primitives::block::GasPrices;
use katana_primitives::chain::ChainId;
use katana_primitives::genesis::{
    allocation::DevAllocationsGenerator,
    constant::DEFAULT_PREFUNDED_ACCOUNT_BALANCE, Genesis,
};
use katana_rpc::config::ServerConfig;
use katana_rpc_api::ApiKind;
use starknet_types_core::felt::Felt;
use url::Url;

pub struct Katana {
    rpc_handle: NodeHandle,
}

impl Katana {
    #[allow(dead_code)]
    pub async fn init(url: &str) -> Result<Self> {
        let sequencer_config = get_sequencer_config();
        let server_config = get_server_config(url)?;
        let starknet_config = get_starknet_config();
        let (rpc_handle, _) =
            start(server_config, sequencer_config, starknet_config).await?;
        Ok(Katana { rpc_handle })
    }

    #[allow(dead_code)]
    pub fn port(&self) -> u16 {
        self.rpc_handle.addr.port()
    }

    #[allow(dead_code)]
    pub fn stop(&self) -> Result<()> {
        self.rpc_handle.handle.stop()?;
        Ok(())
    }
}

impl Drop for Katana {
    fn drop(&mut self) {
        let _ = self.rpc_handle.handle.stop();
    }
}

#[allow(deprecated)]
#[allow(dead_code)]
fn get_sequencer_config() -> SequencerConfig {
    SequencerConfig { block_time: None, no_mining: false }
}

#[allow(dead_code)]
fn get_server_config(url: &str) -> Result<ServerConfig> {
    let url = Url::parse(url)?;
    Ok(ServerConfig {
        apis: vec![
            ApiKind::Starknet,
            ApiKind::Katana,
            ApiKind::Torii,
            ApiKind::Saya,
        ],
        port: url.port().unwrap(),
        host: url.host().unwrap().to_string(),
        max_connections: 100,
        allowed_origins: None,
        metrics: None,
    })
}

#[allow(dead_code)]
fn get_starknet_config() -> StarknetConfig {
    let gas_prices = GasPrices { eth: 100000000000, strk: 100000000000 };
    let accounts = DevAllocationsGenerator::new(10)
        .with_seed(parse_seed("0"))
        .with_balance(U256::from(DEFAULT_PREFUNDED_ACCOUNT_BALANCE))
        .generate();
    let mut genesis = Genesis {
        gas_prices,
        sequencer_address: *DEFAULT_SEQUENCER_ADDRESS,
        ..Default::default()
    };
    genesis
        .extend_allocations(accounts.into_iter().map(|(k, v)| (k, v.into())));

    StarknetConfig {
        disable_fee: true,
        disable_validate: false,
        fork_rpc_url: None,
        fork_block_number: None,
        env: Environment {
            chain_id: ChainId::Id(Felt::from_str("0x4b4154414e41").unwrap()),
            invoke_max_steps: 1000000,
            validate_max_steps: 1000000,
        },
        db_dir: None,
        genesis,
    }
}

#[allow(dead_code)]
fn parse_seed(seed: &str) -> [u8; 32] {
    let seed = seed.as_bytes();
    let mut actual_seed = [0u8; 32];
    seed.iter().enumerate().for_each(|(i, b)| actual_seed[i] = *b);
    actual_seed
}
