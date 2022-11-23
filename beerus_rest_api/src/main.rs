use beerus_rest_api::api::ethereum;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hakai!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, ethereum::endpoints::query_balance,])
}
