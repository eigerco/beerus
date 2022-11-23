pub mod resp;

use std::str::FromStr;

use crate::api::ethereum::resp::QueryBalanceResponse;
use beerus_core::{
    config::Config,
    lightclient::beerus::{Beerus, BeerusLightClient},
};
use ethers::{types::Address, utils};
use helios::types::BlockTag;
use rocket::get;
use rocket::serde::json::Json;

#[get("/ethereum/balance/<address>")]
pub async fn query_balance(address: &str) -> Json<QueryBalanceResponse> {
    // TODO: proper error handling (remove all unwrap calls).
    let config = Config::new_from_env().unwrap();
    // Create a new Beerus light client.
    let mut beerus = BeerusLightClient::new(&config).unwrap();
    // Start the Beerus light client.
    beerus.start().await.unwrap();
    // Parse the Ethereum address.
    let addr = Address::from_str(&address).unwrap();

    // TODO: Make the block tag configurable.
    let block = BlockTag::Latest;
    // Query the balance of the Ethereum address.
    let balance = beerus
        .ethereum_lightclient
        .get_balance(&addr, block)
        .await
        .unwrap();
    // Format the balance in Ether.
    let balance_in_eth = utils::format_units(balance, "ether").unwrap();
    Json(QueryBalanceResponse {
        address: address.to_string(),
        balance: balance_in_eth,
        unit: "ETH".to_string(),
    })
}
