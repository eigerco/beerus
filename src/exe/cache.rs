use ethers::types::U256;
use lru::LruCache;
use starknet_api::{core::ContractAddress, state::StorageKey};
use starknet_types_core::felt::Felt as StarkFelt;
use std::num::NonZeroUsize;
use std::sync::{LazyLock, Mutex};

use crate::gen;

type Key = (U256, U256, U256); // block hash + contract address + storage key
type Value = U256;

const CACHE_SIZE: usize = 1024;

static CACHE: LazyLock<Mutex<LruCache<Key, Value>>> = LazyLock::new(|| {
    Mutex::new(LruCache::new(NonZeroUsize::new(CACHE_SIZE).unwrap()))
});

fn get(key: &Key) -> Option<Value> {
    let mut guard = CACHE.lock().expect("cache-lock");
    guard.get(key).cloned()
}

fn set(key: Key, value: Value) -> Option<Value> {
    let mut guard = CACHE.lock().expect("cache-lock");
    guard.put(key, value)
}

fn key(
    block_hash: &gen::Felt,
    contract_address: &ContractAddress,
    storage_key: &StorageKey,
) -> Key {
    (
        block_hash.as_ref().parse().unwrap(),
        contract_address.0.key().to_bytes_be().into(),
        storage_key.0.key().to_bytes_be().into(),
    )
}

pub trait StorageCache {
    fn lookup(
        &self,
        block_hash: &gen::Felt,
        contract_address: &ContractAddress,
        storage_key: &StorageKey,
    ) -> Option<StarkFelt>;
    fn insert(
        &self,
        block_hash: &gen::Felt,
        contract_address: &ContractAddress,
        storage_key: &StorageKey,
        val: &gen::Felt,
    );
}

pub struct Empty;

impl StorageCache for Empty {
    fn lookup(
        &self,
        _block_hash: &gen::Felt,
        _contract_address: &ContractAddress,
        _storage_key: &StorageKey,
    ) -> Option<StarkFelt> {
        None
    }

    fn insert(
        &self,
        _block_hash: &gen::Felt,
        _contract_address: &ContractAddress,
        _storage_key: &StorageKey,
        _val: &gen::Felt,
    ) {
    }
}

pub struct LRU;

impl StorageCache for LRU {
    fn lookup(
        &self,
        block_hash: &gen::Felt,
        contract_address: &ContractAddress,
        storage_key: &StorageKey,
    ) -> Option<StarkFelt> {
        get(&key(block_hash, contract_address, storage_key))
            .map(|value| StarkFelt::from_raw(value.0))
    }

    fn insert(
        &self,
        block_hash: &gen::Felt,
        contract_address: &ContractAddress,
        storage_key: &StorageKey,
        val: &gen::Felt,
    ) {
        let val = val.as_ref().parse().unwrap();
        set(key(block_hash, contract_address, storage_key), val);
    }
}
