use beerus_rest_api::api::ethereum;
use log::info;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hakai!"
}

#[launch]
fn rocket() -> _ {
    env_logger::init();
    info!("Starting Beerus Rest API...");
    rocket::build().mount("/", routes![index, ethereum::endpoints::query_balance,])
}
