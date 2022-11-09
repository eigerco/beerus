use beerus::cli::runner;

use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    runner::run().await
}
