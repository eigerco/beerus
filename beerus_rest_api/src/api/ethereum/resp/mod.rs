use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct QueryBalanceResponse {
    pub address: String,
    pub balance: String,
    pub unit: String,
}
