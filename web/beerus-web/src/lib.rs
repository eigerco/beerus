use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_compiler::project::{
    AllCratesConfig, ProjectConfig, ProjectConfigContent,
};
use cairo_lang_compiler::CompilerConfig;
use cairo_lang_filesystem::cfg::{Cfg, CfgSet};
use cairo_lang_filesystem::db::{
    CrateSettings, DependencySettings, Edition, ExperimentalFeaturesConfig,
};
use cairo_lang_filesystem::ids::{CrateLongId, Directory};
use cairo_lang_lowering::utils::InliningStrategy;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_starknet::compile::compile_prepared_db;
use cairo_lang_starknet::contract::{find_contracts, ContractDeclaration};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use semver::Version;
use smol_str::SmolStr;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

const MAINNET_ETHEREUM_CHAINID: &str = "0x1";
const SEPOLIA_ETHEREUM_CHAINID: &str = "0xaa36a7";

const MAINNET_STARKNET_CHAINID: &str = "0x534e5f4d41494e";
const SEPOLIA_STARKNET_CHAINID: &str = "0x534e5f5345504f4c4941";

pub mod dto {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct State {
        pub len: i64,
        pub hash: String,
        pub root: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Config {
        pub ethereum_url: String,
        pub starknet_url: String,
    }
}

#[wasm_bindgen]
pub fn declare() -> Result<JsValue, JsValue> {
    // Pre-declare preparation in JavaScript or separate
    // Read template account from default location
    // Copy it to another
    // Updating it's timestamp constant to make it different from others
    // Compile it by using scarb - NOT WASM COMPILABLE, calling
    // separate rpc?
    // -----------------
    // Declare API Args:
    // -----------------
    // COMPILED_CONTRACT.json
    // PREFUNDED_ACCOUNT.json
    // PREFUNDED_KEY.json
    //
    // Usage of starkli - NOT WASM COMPILABLE, calling separate rpc?
    // Starkli create keystore key.json - prompt password
    // Extract class hash from compiled contract
    // Starkli create account.json that needs key.json and class hash
    // -> Where to store it? Or just return it?
    // Starkli declare needs COMPILED_CONTRACT, PREFUNDED_ACCOUNT and
    // PREFUNDED_KEY - Prompt for PASSWORD will be necessary
    //
    // Return DEPLOYMENT_ADDRESS, ACCOUNT.json and KEY.json
    let db = generate_database()?;
    let contracts = generate_contract_declaration(&db);
    let mut compiler_config = CompilerConfig {
        // diagnostics_reporter: set to callback of print or print json in
        // scarb/src/compiler/helpers.rs
        // For simplicty, set to default
        replace_ids: true,
        inlining_strategy: InliningStrategy::Default,
        add_statements_functions: false,
        add_statements_code_locations: false,
        ..CompilerConfig::default()
    };

    compiler_config
        .diagnostics_reporter
        .ensure(&db)
        .map_err(|e| JsValue::from_str(&format!("Failed to ensure: {e}")))?;
    let classes = compile_prepared_db(
        &db,
        &contracts.iter().collect::<Vec<_>>(),
        compiler_config,
    )
    .map_err(|e| JsValue::from_str(&format!("Error while compiling: {e}")))?;
    Ok(JsValue::from_str(&format!("Size of classes: {}", classes.len())))
}

fn generate_database() -> Result<RootDatabase, JsValue> {
    // Differences from scarb binary
    // -----
    // in compiler/db.rs builder.with_plugin_suite() is called
    // but in fact, these plugins are empty
    let mut builder = RootDatabase::builder();
    builder.with_project_config(build_project_config());

    let mut cfg_set = CfgSet::new();
    cfg_set.insert(Cfg {
        key: "target".into(),
        value: Some("starknet-contract".into()),
    });
    builder.with_cfg(cfg_set);
    builder.with_inlining_strategy(InliningStrategy::Default);
    builder.with_plugin_suite(cairo_lang_starknet::starknet_plugin_suite());
    builder.build().map_err(|e| {
        JsValue::from_str(&format!("Error while building database: {e}"))
    })
}

fn generate_contract_declaration(
    db: &dyn SemanticGroup,
) -> Vec<ContractDeclaration> {
    let name = "account";
    let crate_ids = vec![db.intern_crate(CrateLongId::Real {
        name: name.into(),
        version: Some(Version::new(0, 1, 0)),
    })];
    find_contracts(db, &crate_ids)
}

fn build_project_config() -> ProjectConfig {
    let mut cfg = CfgSet::new();
    cfg.insert(Cfg {
        key: "target".into(),
        value: Some("starknet-contract".into()),
    });
    let mut deps_account = BTreeMap::new();
    deps_account.insert(
        "account".to_string(),
        DependencySettings { version: Some(Version::new(0, 1, 0)) },
    );
    deps_account
        .insert("core".to_string(), DependencySettings { version: None });
    let account = CrateSettings {
        edition: Edition::V2023_01,
        cfg_set: Some(cfg),
        version: Some(Version::new(0, 1, 0)),
        experimental_features: ExperimentalFeaturesConfig {
            negative_impls: false,
            coupons: false,
        },
        dependencies: deps_account,
    };

    let mut deps_core = BTreeMap::new();
    deps_core.insert("core".to_string(), DependencySettings { version: None });

    let core = CrateSettings {
        edition: Edition::V2024_07,
        cfg_set: None,
        version: Some(Version::new(2, 8, 2)),
        experimental_features: ExperimentalFeaturesConfig {
            negative_impls: true,
            coupons: true,
        },
        dependencies: deps_core,
    };

    let mut crates_config: OrderedHashMap<SmolStr, CrateSettings> =
        OrderedHashMap::default();
    crates_config.insert("account".into(), account);
    crates_config.insert("core".into(), core);

    let mut crate_roots: OrderedHashMap<SmolStr, PathBuf> =
        OrderedHashMap::default();
    crate_roots.insert(
        "account".into(),
        "/home/ivan/Development/rust_projects/beerus/target/account-20241124093852/src".into(),
    );

    let crates_config =
        AllCratesConfig { override_map: crates_config, ..Default::default() };
    let content = ProjectConfigContent { crate_roots, crates_config };

    ProjectConfig {
        base_path: "/home/ivan/Development/rust_projects/beerus/target/account-20241124093852"
            .into(),
        corelib: Some(Directory::Real(
            "/home/ivan/.cache/scarb/registry/std/323ea7e28/core/src".into(),
        )),
        content,
    }
}

#[wasm_bindgen]
pub fn estimate() -> JsValue {
    // Estimate API Args:
    // ------------------
    // ACCOUNT.json
    // KEY.json
    //
    // Starkli estimate fee on Beerus, because estimation is also
    // running on deployment and deployment is called with these args
    // Therefore, these args should be enough
    //
    // Return ESTIMATE_AMOUNT
    JsValue::from_str("Estimate fee is 10.")
}

#[wasm_bindgen]
pub fn transfer() -> JsValue {
    // Transfer API Args:
    // ------------------
    // DEPLOYMENT_ADDRESS
    // AMOUNT
    // PREFUNDED_ACCOUNT.json
    // PREFUNDED_KEY.json
    //
    // Starkli invoke eth transfer
    // Necessary to prompt password for PREFUNDED_KEY.json
    //
    // Return TRANSACTION_HASH
    JsValue::from_str("Successfully transfered 20!")
}

#[wasm_bindgen]
pub fn deploy() -> JsValue {
    // Deploy API Args:
    // ------------------
    // ACCOUNT.json
    // KEY.json
    //
    // Starkli deploy
    // Necessary to prompt password for KEY.json
    //
    // Return TRANSACTION_HASH
    JsValue::from_str("Successfully deployed!")
}

#[derive(Clone)]
pub struct Http(Rc<js_sys::Function>);

impl beerus::gen::client::blocking::HttpClient for Http {
    fn post(
        &self,
        url: &str,
        request: &iamgroot::jsonrpc::Request,
    ) -> std::result::Result<
        iamgroot::jsonrpc::Response,
        iamgroot::jsonrpc::Error,
    > {
        let json = serde_json::to_string(&request).map_err(|e| {
            iamgroot::jsonrpc::Error::new(
                32101,
                format!("request failed: {e:?}"),
            )
        })?;
        let result = self
            .0
            .as_ref()
            .call2(
                &JsValue::null(),
                &JsValue::from_str(url),
                &JsValue::from_str(&json),
            )
            .map_err(|e| {
                iamgroot::jsonrpc::Error::new(
                    32101,
                    format!("request failed: {e:?}"),
                )
            })?;
        let result = result.as_string().ok_or_else(|| {
            iamgroot::jsonrpc::Error::new(
                32101,
                format!("request failed: ¯\\_(ツ)_/¯"),
            )
        })?;
        let response = serde_json::from_str(&result).map_err(|e| {
            iamgroot::jsonrpc::Error::new(
                32101,
                format!("request failed: {e:?}"),
            )
        })?;
        Ok(response)
    }
}

#[async_trait::async_trait(?Send)]
impl beerus::gen::client::HttpClient for Http {
    async fn post(
        &self,
        url: &str,
        request: &iamgroot::jsonrpc::Request,
    ) -> std::result::Result<
        iamgroot::jsonrpc::Response,
        iamgroot::jsonrpc::Error,
    > {
        let client = reqwest::Client::new();
        let response = post(&client, url, &request).await?;
        Ok(response)
    }
}

async fn post<Q: serde::Serialize, R: serde::de::DeserializeOwned>(
    client: &reqwest::Client,
    url: &str,
    request: Q,
) -> std::result::Result<R, iamgroot::jsonrpc::Error> {
    let response = client
        .post(url)
        .json(&request)
        .send()
        .await
        .map_err(|e| {
            iamgroot::jsonrpc::Error::new(
                32101,
                format!("request failed: {e:?}"),
            )
        })?
        .json()
        .await
        .map_err(|e| {
            iamgroot::jsonrpc::Error::new(
                32102,
                format!("invalid response: {e:?}"),
            )
        })?;
    Ok(response)
}

async fn call(
    client: &reqwest::Client,
    url: &str,
    method: &str,
) -> Result<String, JsValue> {
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": [],
        "id": 0
    });
    let response: serde_json::Value = post(&client, url, &request)
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    response["result"].as_str().map(|result| result.to_owned()).ok_or_else(
        || JsValue::from_str(&format!("Result missing for '{method}'.")),
    )
}

async fn check(config: &dto::Config) -> Result<(), JsValue> {
    let client = reqwest::Client::new();
    let ethereum_chain =
        call(&client, &config.ethereum_url, "eth_chainId").await?;
    let starknet_chain =
        call(&client, &config.starknet_url, "starknet_chainId").await?;
    match (ethereum_chain.as_str(), starknet_chain.as_str()) {
        (MAINNET_ETHEREUM_CHAINID, MAINNET_STARKNET_CHAINID) => Ok(()),
        (SEPOLIA_ETHEREUM_CHAINID, SEPOLIA_STARKNET_CHAINID) => Ok(()),
        _ => {
            Err(JsValue::from_str(&format!("Chain ID mismatch ethereum={ethereum_chain} starknet={starknet_chain}")))
        }
    }
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub struct Beerus {
    beerus: beerus::client::Client<Http>,
    state: Option<beerus::client::State>,
}

#[wasm_bindgen]
impl Beerus {
    #[wasm_bindgen(constructor)]
    pub async fn new(
        config_json: &str,
        f: js_sys::Function,
    ) -> Result<Beerus, JsValue> {
        let config: dto::Config =
            serde_json::from_str(config_json).map_err(|e| {
                JsValue::from_str(&format!(
                    "beerus: invalid config JSON: {e:?}"
                ))
            })?;
        check(&config).await.map_err(|e| {
            JsValue::from_str(&format!("beerus: invalid RPC config: {e:?}"))
        })?;
        let config = beerus::config::Config {
            ethereum_rpc: config.ethereum_url,
            starknet_rpc: config.starknet_url,
        };
        let beerus = beerus::client::Client::new(&config, Http(Rc::new(f)))
            .await
            .map_err(|e| {
                JsValue::from_str(&format!("failed to create client: {e:?}"))
            })?;
        web_sys::console::log_1(&"beerus: ready".into());
        Ok(Self { beerus, state: None })
    }

    #[wasm_bindgen]
    pub async fn get_state(&mut self) -> Result<JsValue, JsValue> {
        let state = self.beerus.get_state().await.map_err(|e| {
            JsValue::from_str(&format!("failed to get state: {e:?}"))
        })?;

        let ret = serde_json::to_string(&dto::State {
            len: state.block_number as i64,
            hash: state.block_hash.as_ref().to_owned(),
            root: state.root.as_ref().to_owned(),
        })
        .map_err(|e| {
            JsValue::from_str(&format!("failed to serialize state: {e:?}"))
        })?;
        let ret = JsValue::from_str(&ret);

        self.state = Some(state);
        Ok(ret)
    }

    #[wasm_bindgen]
    pub async fn execute(&mut self, request: &str) -> Result<JsValue, JsValue> {
        if self.state.is_none() {
            let _ = self.get_state().await?;
        }
        let state = self.state.clone().unwrap();

        let request: beerus::gen::FunctionCall = serde_json::from_str(request)
            .map_err(|e| {
                JsValue::from_str(&format!("failed to parse request: {e:?}"))
            })?;

        let result = self.beerus.execute(request, state).map_err(|e| {
            JsValue::from_str(&format!("failed to execute call: {e:?}"))
        })?;

        let result = serde_json::to_string(&result).map_err(|e| {
            JsValue::from_str(&format!(
                "failed to serialize call result: {e:?}"
            ))
        })?;
        Ok(JsValue::from_str(&result))
    }
}
