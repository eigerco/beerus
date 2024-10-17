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

pub fn create_keystore(
    file: &str,
    password: &str,
    paths: &Vec<String>,
) -> Result<Vec<SigningKey>, Error> {
    let mut keys = Vec::with_capacity(paths.len());
    for path in paths {
        let key = SigningKey::from_random();
        let key_file = path.clone() + "/" + file;
        key.save_as_keystore(key_file, password)?;
        keys.push(key);
    }
    Ok(keys)
}

pub fn extract_class_hash(paths: &Vec<String>) -> Result<Vec<Felt>, Error> {
    let mut class_hashes = Vec::with_capacity(paths.len());
    for path in paths {
        let compiled =
            path.clone() + "/target/dev/account_Account.contract_class.json";
        let class = serde_json::from_reader::<_, SierraClass>(
            std::fs::File::open(compiled)?,
        )?;
        class_hashes.push(class.class_hash()?);
    }
    Ok(class_hashes)
}

pub async fn create_account(
    keys: Vec<SigningKey>,
    class_hash: Vec<Felt>,
    path: &Vec<String>,
    file: &str,
) -> Result<(), Error> {
    for (index, key) in keys.into_iter().enumerate() {
        let signer = AnySigner::LocalWallet(LocalWallet::from_signing_key(key));
        let salt = SigningKey::from_random().secret_scalar();
        let account_config = AccountConfig {
            version: 1,
            variant: AccountVariant::OpenZeppelin(OzAccountConfig {
                version: 1,
                public_key: signer.get_public_key().await?.scalar(),
                legacy: false,
            }),
            deployment: DeploymentStatus::Undeployed(UndeployedStatus {
                class_hash: class_hash[index],
                salt,
                context: None,
            }),
        };
        let mut file = std::fs::File::create(path[index].clone() + "/" + file)?;
        serde_json::to_writer_pretty(&mut file, &account_config)?;
        file.write_all(b"\n")?;
    }
    Ok(())
}
