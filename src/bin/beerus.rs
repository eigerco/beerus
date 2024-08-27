use std::{sync::Arc, time::Duration};

use beerus::config::{check_data_dir, Config};
use clap::Parser;
use tokio::sync::RwLock;
use validator::Validate;

const RPC_SPEC_VERSION: &str = "0.7.1";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let config = get_config(&args).await?;

    let beerus = beerus::client::Client::new(&config).await?;
    beerus.start().await?;

    let rpc_spec_version = beerus.spec_version().await?;
    if rpc_spec_version != RPC_SPEC_VERSION {
        eyre::bail!("RPC spec version mismatch: expected {RPC_SPEC_VERSION} but got {rpc_spec_version}");
    }

    let state = beerus.get_state().await?;
    tracing::info!(?state, "initialized");

    let state = Arc::new(RwLock::new(state));

    {
        let state = state.clone();
        let period = Duration::from_secs(config.poll_secs);
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(period);
            loop {
                tick.tick().await;
                match beerus.get_state().await {
                    Ok(update) => {
                        *state.write().await = update;
                        tracing::info!(?state, "updated");
                    }
                    Err(e) => {
                        tracing::error!(error=?e, "state update failed");
                    }
                }
            }
        });
    }

    let server =
        beerus::rpc::serve(&config.starknet_rpc, &config.rpc_addr, state)
            .await?;

    tracing::info!(port = server.port(), "rpc server started");
    server.done().await;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[clap(short = 'c', long)]
    config: Option<String>,
    #[clap(short, long, default_value_t = false)]
    skip_chain_id_validation: bool,
}

async fn get_config(args: &Args) -> eyre::Result<Config> {
    let config = if let Some(path) = args.config.as_ref() {
        Config::from_file(path)?
    } else {
        Config::from_env()
    };
    config.validate()?;
    if args.skip_chain_id_validation {
        tracing::warn!("Skipping chain id validation");
    } else {
        config.validate_chain_id().await?;
    }
    check_data_dir(&config.data_dir)?;
    Ok(config)
}
