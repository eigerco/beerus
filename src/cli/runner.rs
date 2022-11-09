use crate::config::Config;

use super::{
    ethereum,
    model::{Cli, Commands, EthereumSubCommands},
};
use clap::Parser;
use eyre::Result;

pub async fn run(config: &Config) -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Ethereum(ethereum_commands) => match &ethereum_commands.command {
            EthereumSubCommands::QueryBalance { address } => {
                ethereum::query_balance(config, address.to_string()).await
            }
        },
    }
}
