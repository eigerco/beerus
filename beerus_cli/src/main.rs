use beerus_cli::runner;
use beerus_core::config::Config;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let config = Config::new_from_env()?;
    runner::run(config).await
}
