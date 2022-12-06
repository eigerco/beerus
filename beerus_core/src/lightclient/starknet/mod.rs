use async_trait::async_trait;
use eyre::Result;

use crate::config::Config;
#[async_trait]
pub trait StarkNetLightClient: Send + Sync {
    async fn start(&mut self) -> Result<()>;
}

pub struct StarkNetLightClientImpl {}

impl StarkNetLightClientImpl {
    pub fn new(_config: Config) -> Result<Self> {
        Ok(Self {})
    }
}

#[async_trait]
impl StarkNetLightClient for StarkNetLightClientImpl {
    async fn start(&mut self) -> Result<()> {
        Ok(())
    }
}
