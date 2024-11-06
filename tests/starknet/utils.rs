use std::fs;

use anyhow::Error;
use chrono;

const SOURCE_LIB: &str = "./tests/starknet/contract/account/src/lib.cairo";
const SOURCE_TOML: &str = "./tests/starknet/contract/account/Scarb.toml";

#[allow(dead_code)]
pub struct AccountEnvironment {
    pub folder: String,
    pub toml: String,
    pub id: String,
}

#[allow(dead_code)]
pub fn prepare_account() -> Result<AccountEnvironment, Error> {
    let now = chrono::offset::Local::now();
    let id = now.format("%Y%m%y%H%M%S").to_string();
    let target = "./target/account-".to_string() + &id + "/";
    let target_lib = target.clone() + "src/lib.cairo";
    let target_toml = target.clone() + "Scarb.toml";
    let target_src = target.clone() + "src";

    fs::create_dir(target.clone())?;
    fs::create_dir(target_src)?;
    fs::copy(SOURCE_LIB, target_lib.clone())?;
    fs::copy(SOURCE_TOML, target_toml.clone())?;

    let account_template = fs::read_to_string(target_lib.clone())?;
    let account_new = account_template.replace("<ID>", &id);
    fs::write(target_lib, account_new)?;

    Ok(AccountEnvironment { folder: target, toml: target_toml, id })
}
