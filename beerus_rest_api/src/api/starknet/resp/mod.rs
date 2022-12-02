use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct QueryStateRootResponse {
    pub state_root: String,
}
