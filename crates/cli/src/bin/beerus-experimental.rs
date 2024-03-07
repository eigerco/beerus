use beerus_cli::{get_config, Args};

use clap::Parser;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();
    let config = get_config(Args::parse())?;

    #[cfg(feature = "experimental")]
    {
        let server = beerus_experimental_api::rpc::serve(
            &config.starknet_rpc,
            &config.rpc_addr,
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
