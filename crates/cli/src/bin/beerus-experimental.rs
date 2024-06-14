use beerus_cli::{get_config, Args};
use beerus_core::client::BeerusClient;

use clap::Parser;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();
    let config = get_config(Args::parse())?;
    config.validate_params().await?;

    let mut beerus = BeerusClient::new(&config).await?;
    beerus.start().await?;

    #[cfg(feature = "experimental")]
    {
        let server = beerus_experimental_api::rpc::serve(
            &config.starknet_rpc,
            &config.rpc_addr,
            beerus.node.clone(),
        )
        .await?;
        tracing::info!(port = server.port(), "experimental rpc server started");
        server.done().await;
    }

    #[cfg(not(feature = "experimental"))]
    {
        tracing::info!(?config);
        eprintln!("built without 'experimental' feature enabled");
    }

    Ok(())
}
