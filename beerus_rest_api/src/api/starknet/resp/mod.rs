use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct QueryStateRootResponse {
    pub state_root: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct QueryContractViewResponse {
    pub result: Vec<String>,
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct QueryGetStorageAtResponse {
    pub result: String,
}
