use std::sync::Arc;

use beerus_core::lightclient::{beerus::BeerusLightClient, ethereum::helios::HeliosLightClient};
use log::debug;

use eyre::Result;

use super::resp::QueryStateRootResponse;

/// StarkNet API endpoints handler.
pub struct StarkNetAPI<'a> {
    /// The Beerus light client.
    beerus: Arc<BeerusLightClient<'a>>,
}

impl<'a> StarkNetAPI<'a> {
    /// Create a new StarkNet API handler.
    pub fn new(beerus: Arc<BeerusLightClient<'a>>) -> Self {
        Self {
            beerus: beerus.clone(),
        }
    }

    /// Query the state root of StarkNet.
    pub async fn query_state_root(&self) -> Result<QueryStateRootResponse> {
        debug!("Querying StarkNet state root");
        let beerus = self.beerus.clone();
        // Call the StarkNet contract to get the state root.
        let state_root = beerus.starknet_state_root().await?;
        Ok(QueryStateRootResponse {
            state_root: state_root.to_string(),
        })
    }
}

unsafe impl Send for StarkNetAPI<'_> {}
unsafe impl Sync for StarkNetAPI<'_> {}
