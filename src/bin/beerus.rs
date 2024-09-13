use std::{sync::Arc, time::Duration};

use beerus::{client::Http, config::{check_data_dir, ServerConfig}};
use tokio::sync::RwLock;
use validator::Validate;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let path = std::env::args().nth(1);
    let config = get_config(path).await?;

    let http = Http::new();
    let beerus = beerus::client::Client::new(&config.client, http).await?;

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
                        tracing::error!(error=%e, "state update failed");
                    }
                }
            }
        });
    }

    let server = beerus::rpc::serve(
        &config.client.starknet_rpc,
        &config.rpc_addr,
        state,
    )
    .await?;

    tracing::info!(port = server.port(), "rpc server started");
    server.done().await;

    Ok(())
}

async fn get_config(path: Option<String>) -> eyre::Result<ServerConfig> {
    let config = if let Some(path) = path {
        ServerConfig::from_file(&path)?
    } else {
        ServerConfig::from_env()
    };
    config.validate()?;
    check_data_dir(&config.client.data_dir)?;
    Ok(config)
}
