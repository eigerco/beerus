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
            EthereumSubCommands::QueryBlockNumber {} => ethereum::query_block_number(beerus).await,
            EthereumSubCommands::QueryChainId {} => ethereum::query_chain_id(beerus).await,
            EthereumSubCommands::QueryCode { address } => {
                ethereum::query_code(beerus, address.to_owned()).await
            }
            EthereumSubCommands::QueryBlockTxCountBykNumber { block } => {
                ethereum::query_block_transaction_count_by_number(beerus, *block).await
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
            StarkNetSubCommands::QueryNonce { address } => {
                starknet::query_starknet_nonce(beerus, address.to_string()).await
            }
            StarkNetSubCommands::L1ToL2MessageCancellations { msg_hash } => {
                starknet::query_starknet_l1_to_l2_messages_cancellation_timestamp(
                    beerus,
                    msg_hash.to_string(),
                )
                .await
            }
            StarkNetSubCommands::L1ToL2Messages { msg_hash } => {
                starknet::query_starknet_l1_to_l2_messages(beerus, msg_hash.to_string()).await
            }
            StarkNetSubCommands::L2ToL1Messages { msg_hash } => {
                starknet::query_starknet_l2_to_l1_messages(beerus, msg_hash.to_string()).await
            }
            StarkNetSubCommands::QueryChainId {} => starknet::query_chain_id(beerus).await,
            StarkNetSubCommands::QueryBlockNumber {} => starknet::query_block_number(beerus).await,
            StarkNetSubCommands::QueryBlockHashAndNumber {} => {
                starknet::query_block_hash_and_number(beerus).await
            }
        },
    }
}
