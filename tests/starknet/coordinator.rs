use std::fs;

use anyhow::Error;
use chrono;

#[allow(dead_code)]
pub enum TestMode {
    Katana,
    Sepolia,
}

#[allow(dead_code)]
pub struct Coordinator {
    id: String,
    mode: TestMode,
    source: String,
    target: String,
}

#[allow(dead_code)]
impl Coordinator {
    pub fn new(mode: TestMode) -> Self {
        let now = chrono::offset::Local::now();
        let id = now.format("%Y%m%y%H%M%S").to_string();
        let target = "./target/account-".to_string() + &id;
        Self {
            id,
            mode,
            source: "./tests/starknet/contract/account".to_string(),
            target,
        }
    }

    pub fn copy_template_to_target(&self) -> Result<(), Error> {
        fs::create_dir(&self.target)?;
        fs::create_dir(self.target_src())?;
        fs::copy(self.source_lib(), self.target_lib())?;
        fs::copy(self.source_scarb(), self.target_scarb())?;
        Ok(())
    }

    pub fn update_account(&self) -> Result<(), Error> {
        let account_template = fs::read_to_string(self.target_lib())?;
        let account_new = account_template.replace("<ID>", &self.id);
        fs::write(self.target_lib(), account_new)?;
        Ok(())
    }

    pub fn source_lib(&self) -> String {
        self.source.clone() + "/src/lib.cairo"
    }

    pub fn source_scarb(&self) -> String {
        self.source.clone() + "/Scarb.toml"
    }

    pub fn target_lib(&self) -> String {
        self.target.clone() + "/src/lib.cairo"
    }

    pub fn target_scarb(&self) -> String {
        self.target.clone() + "/Scarb.toml"
    }

    pub fn target_src(&self) -> String {
        self.target.clone() + "/src"
    }
}

impl Drop for Coordinator {
    fn drop(&mut self) {
        match self.mode {
            TestMode::Katana => {
                let target = self.target.clone();
                if fs::exists(&target).unwrap() {
                    fs::remove_dir_all(target).unwrap()
                }
            }
            TestMode::Sepolia => {}
        }
    }
}
