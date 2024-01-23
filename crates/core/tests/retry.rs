use async_std::future;
use beerus_core::client::wrapped_retry_inner;
use beerus_core::config::Config;
use beerus_core::CoreError;
use helios::client::{Client, Database, FileDB};
use rstest::rstest;

static mut N_RETRY: u8 = 0;

async fn never(_: &Client<impl Database>, _: ethabi::ethereum_types::Address) -> eyre::Result<u64, CoreError> {
    future::pending::<()>().await;
    Ok(0)
}

async fn code_error(_: &Client<impl Database>, _: ethabi::ethereum_types::Address) -> eyre::Result<u64, CoreError> {
    Err(CoreError::FetchL1Val(eyre::eyre!("code error")))
}

async fn fail_and_success(
    _: &Client<impl Database>,
    _: ethabi::ethereum_types::Address,
) -> eyre::Result<u64, CoreError> {
    unsafe {
        N_RETRY += 1;
        if N_RETRY == 1 { Ok(1) } else { Err(CoreError::FetchL1Val(eyre::eyre!("code error"))) }
    }
}

async fn success(_: &Client<impl Database>, _: ethabi::ethereum_types::Address) -> eyre::Result<u64, CoreError> {
    Ok(1)
}

#[rstest]
#[should_panic]
async fn fn_timeout() {
    let conf = Config::from_file("tests/common/data/mainnet.toml");

    #[cfg(not(target_arch = "wasm32"))]
    let l1_client: Client<FileDB> = conf.to_helios_client().await;
    #[cfg(target_arch = "wasm32")]
    let l1_client: Client<ConfigDB> = conf.to_helios_client().await;
    let core_contract_addr = conf.get_core_contract_address();
    wrapped_retry_inner(never, &l1_client, core_contract_addr, 1, 1, 0).await;
}

#[rstest]
#[should_panic]
async fn fn_code_error() {
    let conf = Config::from_file("tests/common/data/mainnet.toml");

    #[cfg(not(target_arch = "wasm32"))]
    let l1_client: Client<FileDB> = conf.to_helios_client().await;
    #[cfg(target_arch = "wasm32")]
    let l1_client: Client<ConfigDB> = conf.to_helios_client().await;
    let core_contract_addr = conf.get_core_contract_address();
    wrapped_retry_inner(code_error, &l1_client, core_contract_addr, 1, 1, 0).await;
}

#[rstest]
async fn fn_fail_and_success() {
    let conf = Config::from_file("tests/common/data/mainnet.toml");

    #[cfg(not(target_arch = "wasm32"))]
    let l1_client: Client<FileDB> = conf.to_helios_client().await;
    #[cfg(target_arch = "wasm32")]
    let l1_client: Client<ConfigDB> = conf.to_helios_client().await;
    unsafe {
        N_RETRY = 0;
    }
    let core_contract_addr = conf.get_core_contract_address();
    let ret = wrapped_retry_inner(fail_and_success, &l1_client, core_contract_addr, 1, 2, 0).await;
    assert_eq!(ret, 1);
}

#[rstest]
#[should_panic]
async fn fn_fail_and_fail() {
    let conf = Config::from_file("tests/common/data/mainnet.toml");

    #[cfg(not(target_arch = "wasm32"))]
    let l1_client: Client<FileDB> = conf.to_helios_client().await;
    #[cfg(target_arch = "wasm32")]
    let l1_client: Client<ConfigDB> = conf.to_helios_client().await;
    unsafe {
        N_RETRY = 2;
    }
    let core_contract_addr = conf.get_core_contract_address();
    wrapped_retry_inner(fail_and_success, &l1_client, core_contract_addr, 1, 2, 0).await;
}

#[rstest]
async fn fn_success() {
    let conf = Config::from_file("tests/common/data/mainnet.toml");

    #[cfg(not(target_arch = "wasm32"))]
    let l1_client: Client<FileDB> = conf.to_helios_client().await;
    #[cfg(target_arch = "wasm32")]
    let l1_client: Client<ConfigDB> = conf.to_helios_client().await;
    let core_contract_addr = conf.get_core_contract_address();
    let ret = wrapped_retry_inner(success, &l1_client, core_contract_addr, 1, 1, 0).await;
    assert_eq!(ret, 1);
}
