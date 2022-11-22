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
    Ethereum(EthereumCommands),
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
