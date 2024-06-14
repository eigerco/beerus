use std::{sync::Arc, time::Duration};

use beerus::config::Config;
use clap::Parser;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let config = get_config(Args::parse())?;
    config.validate_params().await?;

    let beerus = beerus::client::Client::new(&config).await?;
    beerus.start().await?;

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
}

fn get_config(args: Args) -> eyre::Result<Config> {
    Ok(if let Some(path) = args.config.as_ref() {
        Config::from_file(path)?
    } else {
        Config::from_env()
    })
}
