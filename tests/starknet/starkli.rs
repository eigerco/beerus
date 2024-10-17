use std::io::Write;

use anyhow::Error;
use starkli::{
    account::{
        AccountConfig, AccountVariant, DeploymentStatus, OzAccountConfig,
        UndeployedStatus,
    },
    signer::AnySigner,
};
use starknet::{
    core::types::contract::SierraClass,
    signers::{LocalWallet, Signer, SigningKey},
};
use starknet_crypto::Felt;

pub fn extract_class_hash() -> Result<(), Error> {
    let class = serde_json::from_reader::<_, SierraClass>(std::fs::File::open("./tests/starknet/contract/account/target/dev/account_Account.contract_class.json")?)?;
    let class1 = serde_json::from_reader::<_, SierraClass>(std::fs::File::open("./tests/starknet/contract/account1/target/dev/account_Account.contract_class.json")?)?;
    let class2 = serde_json::from_reader::<_, SierraClass>(std::fs::File::open("./tests/starknet/contract/account2/target/dev/account_Account.contract_class.json")?)?;
    println!("Class hash: {:#?}", class.class_hash()?);
    println!("Class hash 1: {:#?}", class1.class_hash()?);
    println!("Class hash 2: {:#?}", class2.class_hash()?);
    Ok(())
}

pub fn create_keystore(
    file: &str,
    password: &str,
) -> Result<SigningKey, Error> {
    let key = SigningKey::from_random();
    key.save_as_keystore(file, password)?;
    Ok(key)
}

pub async fn create_account(
    key: SigningKey,
    class_hash: Felt,
    file: &str,
) -> Result<(), Error> {
    let signer = AnySigner::LocalWallet(LocalWallet::from_signing_key(key));
    let salt = SigningKey::from_random().secret_scalar();
    let account_config = AccountConfig {
        version: 1,
        variant: AccountVariant::OpenZeppelin(OzAccountConfig {
            version: 1,
            public_key: signer.get_public_key().await.unwrap().scalar(),
            legacy: false,
        }),
        deployment: DeploymentStatus::Undeployed(UndeployedStatus {
            class_hash,
            salt,
            context: None,
        }),
    };
    let mut file = std::fs::File::create(file)?;
    serde_json::to_writer_pretty(&mut file, &account_config)?;
    file.write_all(b"\n")?;

    Ok(())
}
