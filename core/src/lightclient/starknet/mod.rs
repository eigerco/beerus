use eyre::Result;

use crate::config::Config;

pub struct StarkNetLightClient {}

impl StarkNetLightClient {
    pub fn new(_config: &Config) -> Result<Self> {
        Ok(Self {})
    }

    pub async fn start(&mut self) -> Result<()> {
        Ok(())
    }
}
