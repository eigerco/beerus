use std::{collections::HashMap, sync::{LazyLock, RwLock}};
use starknet_api::{core::ContractAddress, state::StorageKey};
use starknet_types_core::felt::Felt as StarkFelt;
use ethers::types::U256;

use crate::gen;

type Key = (U256, U256, U256); // block hash + contract address + storage key
type Value = U256;

static CACHE: LazyLock<RwLock<HashMap<Key, Value>>> = LazyLock::new(|| {
    RwLock::new(HashMap::new())
});

fn get(key: &Key) -> Option<Value> {
    let guard = CACHE.read().expect("cache-rlock");
    guard.get(key).cloned()
}

fn set(key: Key, value: Value) -> Option<Value> {
    let mut guard = CACHE.write().expect("cache-wlock");
    guard.insert(key, value)
}

fn key(block_hash: &gen::Felt, contract_address: &ContractAddress, storage_key: &StorageKey) -> Key {
    (
        block_hash.as_ref().parse().unwrap(),
        contract_address.0.key().to_bytes_be().into(),
        storage_key.0.key().to_bytes_be().into(),
    )
}

pub trait StorageCache {
    fn lookup(&self, block_hash: &gen::Felt, contract_address: &ContractAddress, storage_key: &StorageKey) -> Option<StarkFelt>;
    fn insert(&self, block_hash: &gen::Felt, contract_address: &ContractAddress, storage_key: &StorageKey, val: &gen::Felt);
}

pub struct NaiveUnboundedCache;

impl StorageCache for NaiveUnboundedCache {
    fn lookup(&self, block_hash: &gen::Felt, contract_address: &ContractAddress, storage_key: &StorageKey) -> Option<StarkFelt> {
        get(&key(block_hash, contract_address, storage_key))
            .map(|value| StarkFelt::from_raw(value.0))
    }

    fn insert(&self, block_hash: &gen::Felt, contract_address: &ContractAddress, storage_key: &StorageKey, val: &gen::Felt) {
        let val = val.as_ref().parse().unwrap();
        set(key(block_hash, contract_address, storage_key), val);
    }
}
