use beerus_rest_api::build_rocket_server;
use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> Rocket<Build> {
    build_rocket_server().await
}

#[cfg(test)]
mod test {
    use super::rocket;
    use beerus_rest_api::build_rocket_server;
    use rocket::{http::Status, local::asynchronous::Client};
    /// Test the `query_balance` endpoint.
    /// `/ethereum/balance/<address>`
    #[tokio::test]
    // For now we ignore this test because it requires to mock the Beerus light client.
    #[ignore]
    async fn given_normal_conditions_when_query_balance_then_ok() {
        let client = Client::tracked(build_rocket_server().await)
            .await
            .expect("valid rocket instance");
        let response = client
            .get(uri!(
                "/ethereum/balance/0xc24215226336d22238a20a72f8e489c005b44c4a"
            ))
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), "Hello, world!");
    }
}
