use beerus_core::config::Config;
use beerus_rest_api::api::ethereum::{self, ethereum_api::EthereumAPI};
use log::info;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hakai!"
}

#[launch]
async fn rocket() -> _ {
    env_logger::init();
    info!("starting Beerus Rest API...");
    // Create config.
    let config = Config::new_from_env().unwrap();
    // Create a new Ethereum API handler and start syncing.
    info!("ethereum sync started...");
    let ethereum_api = EthereumAPI::new(&config).await.unwrap();
    info!("ethereum sync completed.");
    rocket::build()
        .manage(ethereum_api)
        .mount("/", routes![index, ethereum::endpoints::query_balance,])
}
