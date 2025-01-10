use eyre::{Context, OptionExt, Result};

use crate::{client::State, gen::Felt};

pub struct GatewayClient {
    url: String,
    client: reqwest::Client,
}

impl GatewayClient {
    pub fn new(url: &str) -> Result<Self> {
        if url.ends_with('/') {
            eyre::bail!("Gateway URL must not end with '/'.");
        }
        Ok(Self { url: url.to_owned(), client: reqwest::Client::new() })
    }

    pub async fn get_pubkey(&self, block_hash: &str) -> Result<String> {
        let url = format!(
            "{}/feeder_gateway/get_public_key?blockHash={}",
            self.url, block_hash
        );
        let hex: String = self
            .client
            .get(&url)
            .send()
            .await
            .context("failed to send gateway request")?
            .text()
            .await
            .context("failed to receive gateway response")?;
        Ok(hex)
    }

    pub async fn get_signature(
        &self,
        block_hash: &str,
    ) -> Result<(String, String)> {
        let url = format!(
            "{}/feeder_gateway/get_signature?blockHash={}",
            self.url, block_hash
        );
        let json: serde_json::Value = self
            .client
            .get(&url)
            .send()
            .await
            .context("failed to send gateway request")?
            .json()
            .await
            .context("failed to receive gateway response")?;

        let hash = json["block_hash"]
            .as_str()
            .ok_or_eyre("gateway: invalid block hash")?;
        if hash != block_hash {
            eyre::bail!("gateway: invalid block hash");
        }

        let signature = json["signature"]
            .as_array()
            .ok_or_eyre("gateway: invalid signature")?;
        if signature.len() != 2 {
            eyre::bail!("gateway: invalid signature");
        }

        let r = signature[0]
            .as_str()
            .map(ToOwned::to_owned)
            .ok_or_eyre("gateway: invalid signature")?;
        let s = signature[1]
            .as_str()
            .map(ToOwned::to_owned)
            .ok_or_eyre("gateway: invalid signature")?;
        Ok((r, s))
    }

    pub async fn get_state(&self) -> Result<State> {
        let url =
            format!("{}/feeder_gateway/get_block?blockNumber=latest", self.url);
        let json: serde_json::Value = self
            .client
            .get(&url)
            .send()
            .await
            .context("failed to send gateway request")?
            .json()
            .await
            .context("failed to receive gateway response")?;

        if json["status"].as_str() != Some("ACCEPTED_ON_L2") {
            eyre::bail!("gateway: invalid block status");
        }

        let block_number: u64 = json["block_number"]
            .as_u64()
            .ok_or_eyre("gateway: fetching block_number failed")?;
        let block_hash = json["block_hash"]
            .as_str()
            .map(ToOwned::to_owned)
            .ok_or_eyre("gateway: fetching block_hash failed")?;
        let root = json["state_root"]
            .as_str()
            .map(ToOwned::to_owned)
            .ok_or_eyre("gateway: fetching state_root failed")?;
        Ok(State {
            block_number,
            block_hash: Felt::try_new(&block_hash)?,
            root: Felt::try_new(&root)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{
        matchers::{method, path, query_param},
        Mock, MockServer, ResponseTemplate,
    };

    use super::*;

    #[tokio::test]
    async fn test_ok() -> Result<()> {
        const BLOCK_NUMBER: u64 = 1056427;
        const BLOCK_HASH: &str =
            "0x7c7b366f1b31a556ace49e1affe3b4ed3cfb5aa328b85307655ea70dadd0cc6";
        const STATE_ROOT: &str =
            "0x33d912445ba4f73ce6d910f3952e722aef1c55ee81278b3039b50243278f561";

        let mock = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/feeder_gateway/get_block"))
            .and(query_param("blockNumber", "latest"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({
                    "block_number": BLOCK_NUMBER,
                    "block_hash": BLOCK_HASH,
                    "state_root": STATE_ROOT,
                    "status": "ACCEPTED_ON_L2"
                }),
            ))
            .mount(&mock)
            .await;

        let gateway = GatewayClient::new(mock.uri().as_str())?;
        let state = gateway.get_state().await?;

        assert_eq!(state.root.as_ref(), STATE_ROOT);
        assert_eq!(state.block_number, BLOCK_NUMBER);
        assert_eq!(state.block_hash.as_ref(), BLOCK_HASH);
        Ok(())
    }
}
