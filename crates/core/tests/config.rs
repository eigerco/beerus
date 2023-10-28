use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

use beerus_core::config::{
    Config, DEFAULT_DATA_DIR, DEFAULT_FEE_TOKEN_ADDR, DEFAULT_POLL_SECS, MAINNET_CC_ADDRESS, MAINNET_CONSENSUS_RPC,
    MAINNET_FALLBACK_RPC, TESTNET_CC_ADDRESS, TESTNET_CONSENSUS_RPC, TESTNET_FALLBACK_RPC,
};
use ethers::abi::Address;
use helios::config::networks::Network;
use rstest::rstest;
use starknet_crypto::FieldElement;

#[rstest]
#[should_panic]
fn bad_toml() {
    let _conf = Config::from_file("common/data/bad.toml");
}

#[rstest]
fn goerli_json() {
    let conf = Config::from_file("tests/common/data/goerli.json");
    assert_eq!("https://eth-mainnet.g.alchemy.com/v2/TOKEN", conf.eth_execution_rpc);
    assert_eq!(Address::from_str(TESTNET_CC_ADDRESS).unwrap(), conf.get_core_contract_address());
    assert_eq!(TESTNET_CONSENSUS_RPC, conf.get_consensus_rpc());
    assert_eq!(TESTNET_FALLBACK_RPC, conf.get_fallback_address());
    assert_eq!(Network::GOERLI, conf.network);
    assert_eq!(PathBuf::from(DEFAULT_DATA_DIR), conf.data_dir);
    assert_eq!(DEFAULT_POLL_SECS, conf.poll_secs);
    assert_eq!(SocketAddr::from_str("127.0.0.1:3030").unwrap(), conf.rpc_addr);
    assert_eq!(FieldElement::from_hex_be(DEFAULT_FEE_TOKEN_ADDR).unwrap(), conf.fee_token_addr);
}

#[rstest]
fn mainnet_toml() {
    let conf = Config::from_file("tests/common/data/mainnet.toml");
    assert_eq!("https://eth-mainnet.g.alchemy.com/v2/XXXXX", conf.eth_execution_rpc);
    assert_eq!(Address::from_str(MAINNET_CC_ADDRESS).unwrap(), conf.get_core_contract_address());
    assert_eq!(MAINNET_CONSENSUS_RPC, conf.get_consensus_rpc());
    assert_eq!(MAINNET_FALLBACK_RPC, conf.get_fallback_address());
    assert_eq!(Network::MAINNET, conf.network);
    assert_eq!(PathBuf::from(DEFAULT_DATA_DIR), conf.data_dir);
    assert_eq!(10, conf.poll_secs);
    assert_eq!(SocketAddr::from_str("127.0.0.1:3030").unwrap(), conf.rpc_addr);
    assert_eq!(FieldElement::from_hex_be(DEFAULT_FEE_TOKEN_ADDR).unwrap(), conf.fee_token_addr);
}
