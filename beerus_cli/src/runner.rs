use crate::{model::StarkNetSubCommands, starknet};

use super::{
    ethereum,
    model::{Cli, Commands, EthereumSubCommands},
};
use beerus_core::config::Config;
use clap::Parser;
use eyre::Result;

/// Main entry point for the Beerus CLI.
pub async fn run(config: Config) -> Result<()> {
    // Parse the CLI arguments.
    let cli = Cli::parse();
    // Dispatch the CLI command.
    match &cli.command {
        Commands::Ethereum(ethereum_commands) => match &ethereum_commands.command {
            EthereumSubCommands::QueryBalance { address } => {
                ethereum::query_balance(config, address.to_string()).await
            }
        },
        Commands::StarkNet(starknet_commands) => match &starknet_commands.command {
            StarkNetSubCommands::QueryStateRoot {} => {
                starknet::query_starknet_state_root(config).await
            }
            StarkNetSubCommands::QueryContract {
                address,
                selector,
                calldata,
            } => {
                starknet::query_starknet_contract_view(
                    config,
                    address.to_string(),
                    selector.to_string(),
                    calldata.clone(),
                )
                .await
            }
        },
    }
}
