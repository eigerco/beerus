/// Tools used by multiple test suites.

// TODO Doc
pub fn backend_url() -> String {
    static VAR_NAME: &str = "TEST_BACKEND_URL";
    match std::env::var(VAR_NAME).ok() {
        Some(url) => url,
        None => panic!("The `{VAR_NAME}` env var must be specified to run the tests")
    }
}