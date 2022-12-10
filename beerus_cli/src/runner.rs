use crate::{
    model::{CommandResponse, StarkNetSubCommands},
    starknet,
};

use super::{
    ethereum,
    model::{Cli, Commands, EthereumSubCommands},
};
use beerus_core::lightclient::beerus::BeerusLightClient;
use eyre::Result;

/// Main entry point for the Beerus CLI.
/// # Arguments
/// * `beerus` - The Beerus light client.
/// * `cli` - The CLI arguments.
/// # Returns
/// * `Result<CommandResponse>` - The result of the CLI command.
/// # Errors
/// * If the CLI command fails.
pub async fn run(beerus: BeerusLightClient, cli: Cli) -> Result<CommandResponse> {
    // Dispatch the CLI command.
    match &cli.command {
        // Ethereum commands.
        Commands::Ethereum(ethereum_commands) => match &ethereum_commands.command {
            EthereumSubCommands::QueryBalance { address } => {
                ethereum::query_balance(beerus, address.to_string()).await
            }
            EthereumSubCommands::QueryNonce { address } => {
                ethereum::query_nonce(beerus, address.to_string()).await
            }
        },
        // StarkNet commands.
        Commands::StarkNet(starknet_commands) => match &starknet_commands.command {
            StarkNetSubCommands::QueryStateRoot {} => {
                starknet::query_starknet_state_root(beerus).await
            }
            StarkNetSubCommands::QueryContract {
                address,
                selector,
                calldata,
            } => {
                starknet::query_starknet_contract_view(
                    beerus,
                    address.to_string(),
                    selector.to_string(),
                    calldata.clone(),
                )
                .await
            }
            StarkNetSubCommands::QueryGetStorageAt { address, key } => {
                starknet::query_starknet_get_storage_at(
                    beerus,
                    address.to_string(),
                    key.to_string(),
                )
                .await
            }
        },
    }
}
