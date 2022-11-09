use super::{
    ethereum,
    model::{Cli, Commands, EthereumSubCommands},
};
use clap::Parser;
use eyre::Result;

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Ethereum(ethereum_commands) => match &ethereum_commands.command {
            EthereumSubCommands::QueryBalance { address } => {
                ethereum::query_balance(address.to_string()).await
            }
        },
    }
}
