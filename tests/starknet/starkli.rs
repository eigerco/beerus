use std::{fs, io::Write};

use anyhow::{anyhow, Error};
use clap::Parser;
use starkli::{
    account::{
        AccountConfig, AccountVariant, DeploymentStatus, OzAccountConfig,
        UndeployedStatus,
    },
    signer::AnySigner,
    utils::{Cli, Subcommands},
};
use starknet::{
    core::types::contract::SierraClass,
    signers::{LocalWallet, Signer, SigningKey},
};
use starknet_crypto::Felt;

#[allow(dead_code)]
pub struct Starkli {
    pub rpc: String,
    account_folder: String,
    prefunded_account: PreFundedAccount,
    persist_logger: bool,
}

#[allow(dead_code)]
pub enum PreFundedAccount {
    Katana,
    Sepolia,
}

const ACCOUNT: &str = "account.json";
const COMPILED_ACCOUNT: &str = "target/dev/account_Account.contract_class.json";
const KEY: &str = "key.json";
const PASSWORD: &str = "password";

#[allow(dead_code)]
impl Starkli {
    pub fn new(
        rpc: &str,
        account_folder: &str,
        prefunded_account: PreFundedAccount,
    ) -> Self {
        Self {
            rpc: rpc.into(),
            account_folder: account_folder.into(),
            prefunded_account,
            persist_logger: false,
        }
    }

    pub fn create_keystore(&self) -> Result<SigningKey, Error> {
        let key = SigningKey::from_random();
        let key_file = self.account_folder.clone() + KEY;
        key.save_as_keystore(key_file, PASSWORD)?;
        Ok(key)
    }

    pub fn extract_class_hash(&self) -> Result<Felt, Error> {
        let compiled = self.account_folder.clone() + COMPILED_ACCOUNT;
        let class = serde_json::from_reader::<_, SierraClass>(
            std::fs::File::open(compiled)?,
        )?;
        Ok(class.class_hash()?)
    }

    pub async fn create_account(
        &self,
        key: SigningKey,
        class_hash: Felt,
    ) -> Result<Felt, Error> {
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
                class_hash,
                salt,
                context: None,
            }),
        };
        let target_deployment_address =
            account_config.deploy_account_address()?;
        let mut file =
            std::fs::File::create(self.account_folder.clone() + ACCOUNT)?;
        serde_json::to_writer_pretty(&mut file, &account_config)?;
        file.write_all(b"\n")?;
        Ok(target_deployment_address)
    }

    pub async fn declare_account(&mut self) -> Result<(), Error> {
        let compiled_contract = self.account_folder.clone() + COMPILED_ACCOUNT;
        let rpc = self.rpc.clone();
        let mut input = vec![
            "starkli".to_string(),
            "declare".to_string(),
            compiled_contract,
            "--compiler-version".to_string(),
            "2.8.2".to_string(),
            "--rpc".to_string(),
            rpc,
        ];
        self.setup_prefunded_account(&mut input).await?;
        self.run_command(input).await
    }

    pub async fn invoke_eth_transfer(
        &mut self,
        to_address: Felt,
        amount: u64,
    ) -> Result<(), Error> {
        let address = format!("{:#064x}", to_address);
        let amount = format!("u256:{amount}");
        let rpc = self.rpc.clone();
        let mut input = vec![
            "starkli".to_string(),
            "invoke".to_string(),
            "eth".to_string(),
            "transfer".to_string(),
            address,
            amount,
            "--rpc".to_string(),
            rpc,
        ];
        self.setup_prefunded_account(&mut input).await?;
        self.run_command(input).await
    }

    pub async fn deploy_account(&mut self) -> Result<(), Error> {
        let account = self.account_folder.clone() + "account.json";
        let key = self.account_folder.clone() + "key.json";
        let rpc = self.rpc.clone();
        let input = vec![
            "starkli".to_string(),
            "account".to_string(),
            "deploy".to_string(),
            account,
            "--rpc".to_string(),
            rpc,
            "--keystore".to_string(),
            key,
            "--keystore-password".to_string(),
            "password".to_string(),
            "--skip-manual-confirmation".to_string(),
        ];
        self.run_command(input).await
    }

    async fn setup_prefunded_account(
        &mut self,
        input: &mut Vec<String>,
    ) -> Result<(), Error> {
        match self.prefunded_account {
            PreFundedAccount::Katana => {
                input.append(&mut vec![
                    "--account".to_string(),
                    "katana-0".to_string(),
                ]);
            }
            PreFundedAccount::Sepolia => {
                let account = self.get_deployer_account().await?;
                let private_key = std::env::var("DEPLOYER_PRIVATE_KEY")?;
                input.append(&mut vec![
                    "--account".to_string(),
                    account,
                    "--private-key".to_string(),
                    private_key,
                ]);
            }
        }
        Ok(())
    }

    async fn get_deployer_account(&mut self) -> Result<String, Error> {
        let account_address = std::env::var("DEPLOYER_ACCOUNT_ADDRESS")?;
        let account = self.account_folder.clone() + "account_deployer.json";
        if fs::exists(account.clone())? {
            return Ok(account);
        }
        let input = vec![
            "starkli".to_string(),
            "account".to_string(),
            "fetch".to_string(),
            account_address,
            "--output".to_string(),
            account.clone(),
            "--rpc".to_string(),
            self.rpc.clone(),
        ];
        self.run_command(input).await?;
        Ok(account)
    }

    async fn run_command(
        &mut self,
        mut input: Vec<String>,
    ) -> Result<(), Error> {
        if !self.persist_logger {
            self.persist_logger = true;
        } else {
            input.push("--persist-logger".to_string());
        }
        starkli::utils::run_command(Cli::parse_from(input)).await
    }

    pub async fn call(
        &self,
        address: Felt,
        func: &str,
    ) -> Result<Vec<Felt>, Error> {
        let address = &format!("{:#064x}", address);
        let input = vec!["starkli", "call", address, func, "--rpc", &self.rpc];
        let cli = Cli::parse_from(input);
        let cmd = match cli.command {
            Some(command) => match command {
                Subcommands::Call(cmd) => cmd,
                _ => return Err(anyhow!("Wrong subcommand")),
            },
            None => return Err(anyhow!("Wrong command")),
        };
        cmd.call().await
    }
}
