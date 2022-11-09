use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", global = true)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Ethereum related subcommands
    Ethereum(EthereumCommands),
}

#[derive(Parser, Debug)]
pub struct EthereumCommands {
    #[command(subcommand)]
    pub command: EthereumSubCommands,
}

#[derive(Subcommand, Debug)]
pub enum EthereumSubCommands {
    /// Ethereum related subcommands
    QueryBalance {
        /// The address to query the balance of
        #[arg(short, long, value_name = "ADDRESS")]
        address: String,
    },
}
