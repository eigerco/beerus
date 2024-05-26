use beerus_experimental_api::{
    exe::call,
    gen::{client::blocking::Client, FunctionCall},
};

mod common;

use common::Error;

#[test]
fn test_call_deprecated_contract_class() -> Result<(), Error> {
    let client = client!();
    tracing_subscriber::fmt::try_init().ok();

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
    let function_call: FunctionCall = serde_json::from_value(json)?;

    let call_info = call(&client, function_call)?;
    println!("{call_info:#?}");
    assert!(call_info.execution.retdata.0.is_empty());

    Ok(())
}

#[test]
fn test_call_regular_contract_class() -> Result<(), Error> {
    let client = client!();
    tracing_subscriber::fmt::try_init().ok();

    let json = serde_json::json!({
      "calldata": [],
      "contract_address": "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
      "entry_point_selector": "0x361458367e696363fbcc70777d07ebbd2394e89fd0adcaf147faccd1d294d60"
    });
    let function_call: FunctionCall = serde_json::from_value(json)?;

    let call_info = call(&client, function_call)?;
    println!("{call_info:#?}");
    assert_eq!(call_info.execution.retdata.0.len(), 1);
    assert_eq!(call_info.execution.retdata.0[0], "0x4574686572".try_into()?);

    Ok(())
}
