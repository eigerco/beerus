use std::{fs, io::Write, thread};

use anyhow::{anyhow, Error};
use regex::Regex;
use starkli::{
    account::{
        AccountConfig, AccountVariant, DeploymentStatus, OzAccountConfig,
        UndeployedStatus,
    },
    signer::AnySigner,
};
use starknet::{
    macros::felt,
    signers::{LocalWallet, Signer, SigningKey},
};

use super::scarb::Compiler;

#[allow(dead_code)]
pub struct Executor {
    accounts: Vec<String>,
}

#[allow(dead_code)]
impl Executor {
    pub fn new(num: usize) -> Result<Self, Error> {
        let template = "./tests/starknet/contract/account".to_string();
        let mut accounts = Vec::with_capacity(num);
        accounts.push(template);
        Ok(Self { accounts })
    }

    pub async fn deploy_accounts(
        &mut self,
        update_template: bool,
    ) -> Result<(), Error> {
        if update_template {
            self.update_account(&self.accounts[0])?;
        }
        self.prepare_contracts_environment()?;
        self.compile()?;
        // TODO
        // #804 starkli signer keystore new key.json - Storing somewhere or deleting?
        let key = SigningKey::from_random();
        let file = "key.json";
        let password = "password";
        let _ = key.save_as_keystore(file, password);
        // #804 starkli account oz init account.json - Storing somewhere or deleting?
        let signer = AnySigner::LocalWallet(LocalWallet::from_signing_key(key));
        let oz_account_class_hash = felt!("0x00e2eb8f5672af4e6a4e8a8f1b44989685e668489b0a25437733756c5a34a1d6");
        let salt = SigningKey::from_random().secret_scalar();
        let account_config = AccountConfig {
            version: 1,
            variant: AccountVariant::OpenZeppelin(OzAccountConfig {
                version: 1,
                public_key: signer.get_public_key().await.unwrap().scalar(),
                legacy: false,
            }),
            deployment: DeploymentStatus::Undeployed(UndeployedStatus {
                class_hash: oz_account_class_hash,
                salt,
                context: None,
            }),
        };
        let mut file = std::fs::File::create("account.json")?;
        serde_json::to_writer_pretty(&mut file, &account_config)?;
        file.write_all(b"\n")?;
        // #804 declare accounts
        // #804 #805 fund accounts from pre-funded account
        // #804 deploy accounts
        // #806 iterate through class hashes and call getClass to see if they are verified
        Ok(())
    }

    fn update_account(&self, path: &str) -> Result<(), Error> {
        let lib_path = path.to_owned() + "/src/lib.cairo";
        let account_old = fs::read_to_string(lib_path.clone())?;
        let re = Regex::new(r"self.id.write\((?<number>\d+)\);")?;

        let Some(val) = re.captures(&account_old) else {
            return Err(anyhow!("Could not find pattern in lib.cairo."));
        };
        let num_old =
            &val["number"].parse::<u64>().expect("Failed to read number");
        let num_new = num_old + 1;
        let account_new = account_old.replace(
            &format!("self.id.write({num_old})"),
            &format!("self.id.write({num_new})"),
        );
        fs::write(lib_path, account_new)?;

        Ok(())
    }

    fn prepare_contracts_environment(&mut self) -> Result<(), Error> {
        let capacity = self.accounts.capacity();
        let template = self.accounts[0].clone();
        let lib_path = "/src/lib.cairo";
        let toml_path = "/Scarb.toml";

        for i in 1..capacity {
            let account = template.clone() + &i.to_string();
            fs::create_dir(account.clone())?;
            fs::create_dir(account.clone() + "/src")?;
            fs::copy(
                self.accounts[i - 1].clone() + lib_path,
                account.clone() + lib_path,
            )?;
            fs::copy(
                template.clone() + toml_path,
                account.clone() + toml_path,
            )?;
            self.update_account(&account)?;
            self.accounts.push(account);
        }

        Ok(())
    }

    fn compile(&self) -> Result<(), Error> {
        let mut vec_of_threads = Vec::with_capacity(self.accounts.len());

        for account in self.accounts.iter() {
            let path = account.clone() + "/Scarb.toml";
            let compilation = thread::spawn(move || -> Result<(), Error> {
                let compiler = Compiler::new(&path)?;
                compiler.compile()
            });
            vec_of_threads.push(compilation);
        }
        for (i, thread) in vec_of_threads.into_iter().enumerate() {
            let compilation = thread.join();
            match compilation {
                Ok(val) => val?,
                Err(e) => {
                    return Err(anyhow!("Error during thread {i} execution. Original error message: {:#?}", e));
                }
            }
        }

        Ok(())
    }
}

impl Drop for Executor {
    fn drop(&mut self) {
        let dir = self.accounts[0].clone() + "/target";
        let scarb = self.accounts[0].clone() + "/Scarb.lock";
        if fs::exists(dir.clone()).expect("Failed to check template target") {
            fs::remove_dir_all(dir).expect("Failed to remove template target");
        };
        if fs::exists(scarb.clone()).expect("Failed to check template Scarb") {
            fs::remove_file(scarb).expect("Failed to remove template Scarb");
        }
        for i in 1..self.accounts.len() {
            let dir = self.accounts[i].clone();
            if fs::exists(dir.clone()).expect("Failed to check account dir") {
                fs::remove_dir_all(dir).expect("Failed to remove account dir");
            }
        }
        if fs::exists("key.json").expect("Failed to check key.json") {
            fs::remove_file("key.json").expect("Failed to remove key.json");
        }
        if fs::exists("account.json").expect("Failed to check account.json") {
            fs::remove_file("account.json")
                .expect("Failed to remove account.json");
        }
    }
}
