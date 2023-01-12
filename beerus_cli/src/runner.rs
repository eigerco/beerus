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
use helios::types::BlockTag;

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
            EthereumSubCommands::SendRawTransaction { bytes } => {
                ethereum::send_raw_transaction(beerus, bytes.to_string()).await
            }
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
            EthereumSubCommands::QueryBlockTxCountByNumber { block } => {
                ethereum::query_block_transaction_count_by_number(beerus, *block).await
            }
            EthereumSubCommands::QueryBlockTxCountByHash { hash } => {
                ethereum::query_block_transaction_count_by_hash(beerus, hash.to_string()).await
            }
            EthereumSubCommands::QueryTxByHash { hash } => {
                ethereum::query_transaction_by_hash(beerus, hash.to_string()).await
            }
            EthereumSubCommands::QueryGasPrice {} => ethereum::query_gas_price(beerus).await,
            EthereumSubCommands::QueryEstimateGas { params } => {
                ethereum::query_estimate_gas(beerus, params.to_owned()).await
            }
            EthereumSubCommands::QueryLogs {
                from_block,
                to_block,
                address,
                topics,
                blockhash: block_hash,
            } => {
                ethereum::query_logs(beerus, from_block, to_block, address, topics, block_hash)
                    .await
            }
            EthereumSubCommands::QueryBlockByHash { hash, full_tx } => {
                ethereum::query_block_by_hash(beerus, hash.to_string(), *full_tx).await
            }
            EthereumSubCommands::QueryPriorityFee {} => {
                ethereum::query_get_priority_fee(beerus).await
            }
            EthereumSubCommands::QueryBlockByNumber { block, full_tx } => {
                let block_json = serde_json::to_string(&block)?;
                let block_tag: BlockTag = serde_json::from_str(block_json.as_str())?;
                ethereum::query_block_by_number(beerus, block_tag, *full_tx).await
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
            StarkNetSubCommands::L1ToL2MessageNonce {} => {
                starknet::query_starknet_l1_to_l2_message_nonce(beerus).await
            }
            StarkNetSubCommands::QueryChainId {} => starknet::query_chain_id(beerus).await,
            StarkNetSubCommands::QueryBlockNumber {} => starknet::query_block_number(beerus).await,
            StarkNetSubCommands::QueryBlockHashAndNumber {} => {
                starknet::query_block_hash_and_number(beerus).await
            }
            StarkNetSubCommands::QueryGetClass {
                block_id_type,
                block_id,
                class_hash,
            } => {
                starknet::get_class(
                    beerus,
                    block_id_type.to_string(),
                    block_id.to_string(),
                    class_hash.to_string(),
                )
                .await
            }
            StarkNetSubCommands::QueryGetClassAt {
                block_id_type,
                block_id,
                contract_address,
            } => {
                starknet::get_class_at(
                    beerus,
                    block_id_type.to_string(),
                    block_id.to_string(),
                    contract_address.to_string(),
                )
                .await
            }
            StarkNetSubCommands::QueryGetBlockTransactionCount {
                block_id_type,
                block_id,
            } => {
                starknet::get_block_transaction_count(
                    beerus,
                    block_id_type.to_string(),
                    block_id.to_string(),
                )
                .await
            }
            StarkNetSubCommands::QueryGetStateUpdate {
                block_id_type,
                block_id,
            } => {
                starknet::get_state_update(beerus, block_id_type.to_string(), block_id.to_string())
                    .await
            }
        },
    }
}
