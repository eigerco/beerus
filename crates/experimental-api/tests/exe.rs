use beerus_experimental_api::{
    exe::exec,
    gen::{client::blocking::Client, FunctionCall},
};

#[allow(dead_code)]
mod common;

#[test]
fn test_exec() -> Result<(), common::Error> {
    let Ok(url) = std::env::var("BEERUS_EXPERIMENTAL_TEST_STARKNET_URL") else {
        return Ok(());
    };
    let client = Client::new(&url);

    // TX: 0xcbb2b87d5378e682d650e0e7d36679b4557ba2bfa9d4e285b7168c04376b21
    let json = serde_json::json!({
      "calldata": [
        "0x2",
        "0x57c4b510d66eb1188a7173f31cccee47b9736d40185da8144377b896d5ff3",
        "0x2f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
        "0x0",
        "0x1",
        "0x57c4b510d66eb1188a7173f31cccee47b9736d40185da8144377b896d5ff3",
        "0x2f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a72734550354",
        "0x1",
        "0x1",
        "0x2",
        "0x0",
        "0x1"
      ],
      "contract_address": "0x13e3ca9a377084c37dc7eacbd1d9f8c3e3733935bcbad887c32a0e213cd6fe0",
      "entry_point_selector": "0x162da33a4585851fe8d3af3c2a9c60b557814e221e0d4f30ff0b2189d9c7775"
    });
    let call: FunctionCall = serde_json::from_value(json)?;

    let call_info = exec(&client, call)?;
    assert!(call_info.execution.retdata.0.is_empty());

    Ok(())
}
