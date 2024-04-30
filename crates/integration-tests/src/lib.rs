#[cfg(all(test, feature = "enabled"))]
mod suites;
#[cfg(all(test, feature = "enabled"))]
mod common;


#[cfg(not(all(test, feature = "enabled")))]
#[test]
/// A test stub just to warn when the integration tests aren't enabled
fn warning_integration_tests_arent_enabled() {}