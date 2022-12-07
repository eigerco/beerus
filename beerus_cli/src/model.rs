use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Main struct for the Beerus CLI args.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Set a custom config file
    #[arg(short, long, value_name = "FILE", global = true)]
    pub config: Option<PathBuf>,
    /// List of supported commands.
    #[command(subcommand)]
    pub command: Commands,
}

/// List of supported commands.
#[derive(Subcommand)]
pub enum Commands {
    /// Ethereum related subcommands
    #[command(about = "Ethereum related subcommands")]
    Ethereum(EthereumCommands),
    /// StarkNet related subcommands
    #[command(name = "starknet", about = "StarkNet related subcommands")]
    StarkNet(StarkNetCommands),
}

/// Ethereum related commands.
#[derive(Parser, Debug)]
pub struct EthereumCommands {
    /// Ethereum related subcommands.
    #[command(subcommand)]
    pub command: EthereumSubCommands,
}

/// Ethereum related subcommands.
#[derive(Subcommand, Debug)]
pub enum EthereumSubCommands {
    /// Query the balance of an Ethereum address.
    QueryBalance {
        /// The address to query the balance of
        #[arg(short, long, value_name = "ADDRESS")]
        address: String,
    },
}

/// StarkNet related commands.
#[derive(Parser, Debug)]
pub struct StarkNetCommands {
    /// StarkNet related subcommands.
    #[command(subcommand)]
    pub command: StarkNetSubCommands,
}

/// StarkNet related subcommands.
#[derive(Subcommand, Debug)]
pub enum StarkNetSubCommands {
    /// Query the state root of StarkNet.
    QueryStateRoot {},
    /// Query a StarkNet contract view.
    QueryContract {
        /// The address of the contract to query
        #[arg(short, long, value_name = "ADDRESS")]
        address: String,
        /// The selector of the function to call
        #[arg(short, long, value_name = "SELECTOR")]
        selector: String,
        /// The calldata of the function to call
        #[arg(long, value_name = "CALLDATA", use_value_delimiter = true)]
        calldata: Vec<String>,
    },
    QueryGetStorageAt {
        /// The address of the contract to query
        #[arg(short, long, value_name = "ADDRESS")]
        address: String,
        /// The slot of the storage to query
        #[arg(short, long, value_name = "KEY")]
        key: String,
    },
}
