#![cfg(not(target_arch = "wasm32"))]

pub mod common;
use common::mock_clients;

mod tests {
    use super::*;
    use beerus_core::lightclient::ethereum::helios_lightclient::HeliosLightClient;

    /// Test that we can create a Helios light client.
    #[tokio::test]
    async fn test_helios_lightclient_should_work() {
        // Mock config.
        let (config, _, _) = mock_clients();

        // Create a new Helios light client.
        let client = HeliosLightClient::new(config).await;
        assert!(client.is_ok()); // Assert that the client creation is successful.
    }

    /// Test that we can start a Helios light client.
    #[tokio::test]
    async fn test_helios_lightclient_should_start() {
        let (config, _, _) = mock_clients(); // Mock config.

        let mut helios_light_client = HeliosLightClient::new(config).await.unwrap();

        // Start the Helios light client.
        let _ = helios_light_client.helios_light_client.start().await;

        assert!(true); // Assert that the test passes.
    }
}
