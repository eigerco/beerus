use alloy_primitives::U256;
use blockifier::state::state_api::{State as BlockifierState, StateReader};
use lru::LruCache;
use starknet_api::{core::ContractAddress, state::StorageKey};
use starknet_types_core::felt::Felt as StarkFelt;
use std::num::NonZeroUsize;
use std::sync::{LazyLock, Mutex};

use crate::gen;

mod storage {
    use super::*;

    type Key = (U256, U256, U256); // block hash + contract address + storage key
    type Value = StarkFelt;

    const SIZE: usize = 1024;

    static CACHE: LazyLock<Mutex<LruCache<Key, Value>>> = LazyLock::new(|| {
        Mutex::new(LruCache::new(NonZeroUsize::new(SIZE).unwrap()))
    });

    pub fn get(key: &Key) -> Option<Value> {
        let mut guard = CACHE.lock().expect("storage-cache-lock");
        guard.get(key).cloned()
    }

    pub fn set(key: Key, value: Value) -> Option<Value> {
        let mut guard = CACHE.lock().expect("storage-cache-lock");
        guard.put(key, value)
    }

    pub fn key(
        block_hash: &gen::Felt,
        contract_address: &ContractAddress,
        storage_key: &StorageKey,
    ) -> Key {
        (
            block_hash.as_ref().parse().unwrap(),
            U256::from_be_bytes(contract_address.0.key().to_bytes_be()),
            U256::from_be_bytes(storage_key.0.key().to_bytes_be()),
        )
    }
}

mod class_hash {
    use super::*;

    type Key = (U256, U256); // block hash + contract address
    type Value = starknet_api::core::ClassHash;

    const SIZE: usize = 256;

    static CACHE: LazyLock<Mutex<LruCache<Key, Value>>> = LazyLock::new(|| {
        Mutex::new(LruCache::new(NonZeroUsize::new(SIZE).unwrap()))
    });

    pub fn get(key: &Key) -> Option<Value> {
        let mut guard = CACHE.lock().expect("classhash-cache-lock");
        guard.get(key).cloned()
    }

    pub fn set(key: Key, value: Value) -> Option<Value> {
        let mut guard = CACHE.lock().expect("classhash-cache-lock");
        guard.put(key, value)
    }

    pub fn key(
        block_hash: &gen::Felt,
        contract_address: &ContractAddress,
    ) -> Key {
        (
            block_hash.as_ref().parse().unwrap(),
            U256::from_be_bytes(contract_address.0.key().to_bytes_be()),
        )
    }
}

mod contract_class {
    use super::*;

    type Key = (U256, U256); // block hash + class hash
    type Value = blockifier::execution::contract_class::ContractClass;

    const SIZE: usize = 256;

    static CACHE: LazyLock<Mutex<LruCache<Key, Value>>> = LazyLock::new(|| {
        Mutex::new(LruCache::new(NonZeroUsize::new(SIZE).unwrap()))
    });

    pub fn get(key: &Key) -> Option<Value> {
        let mut guard = CACHE.lock().expect("contractclass-cache-lock");
        guard.get(key).cloned()
    }

    pub fn set(key: Key, value: Value) -> Option<Value> {
        let mut guard = CACHE.lock().expect("contractclass-cache-lock");
        guard.put(key, value)
    }

    pub fn key(
        block_hash: &gen::Felt,
        class_hash: &starknet_api::core::ClassHash,
    ) -> Key {
        (
            block_hash.as_ref().parse().unwrap(),
            U256::from_be_bytes(class_hash.0.to_bytes_be()),
        )
    }
}

pub trait HasBlockHash {
    fn get_block_hash(&self) -> &gen::Felt;
}

pub struct CachedState<T: StateReader + BlockifierState + HasBlockHash> {
    inner: T,
}

impl<T: StateReader + BlockifierState + HasBlockHash> CachedState<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: StateReader + BlockifierState + HasBlockHash> StateReader
    for CachedState<T>
{
    fn get_storage_at(
        &self,
        contract_address: ContractAddress,
        storage_key: StorageKey,
    ) -> blockifier::state::state_api::StateResult<StarkFelt> {
        let block_hash = self.inner.get_block_hash();
        if let Some(ret) = storage::get(&storage::key(
            block_hash,
            &contract_address,
            &storage_key,
        )) {
            return Ok(ret);
        }
        let ret = self.inner.get_storage_at(contract_address, storage_key)?;
        storage::set(
            storage::key(block_hash, &contract_address, &storage_key),
            ret,
        );
        Ok(ret)
    }

    fn get_nonce_at(
        &self,
        contract_address: ContractAddress,
    ) -> blockifier::state::state_api::StateResult<starknet_api::core::Nonce>
    {
        self.inner.get_nonce_at(contract_address)
    }

    fn get_class_hash_at(
        &self,
        contract_address: ContractAddress,
    ) -> blockifier::state::state_api::StateResult<starknet_api::core::ClassHash>
    {
        let block_hash = self.inner.get_block_hash();
        if let Some(ret) =
            class_hash::get(&class_hash::key(block_hash, &contract_address))
        {
            return Ok(ret);
        }
        let ret = self.inner.get_class_hash_at(contract_address)?;
        class_hash::set(class_hash::key(block_hash, &contract_address), ret);
        Ok(ret)
    }

    fn get_compiled_contract_class(
        &self,
        class_hash: starknet_api::core::ClassHash,
    ) -> blockifier::state::state_api::StateResult<
        blockifier::execution::contract_class::ContractClass,
    > {
        let block_hash = self.inner.get_block_hash();
        if let Some(ret) =
            contract_class::get(&contract_class::key(block_hash, &class_hash))
        {
            return Ok(ret);
        }
        let ret = self.inner.get_compiled_contract_class(class_hash)?;
        contract_class::set(
            contract_class::key(block_hash, &class_hash),
            ret.clone(),
        );
        Ok(ret)
    }

    fn get_compiled_class_hash(
        &self,
        class_hash: starknet_api::core::ClassHash,
    ) -> blockifier::state::state_api::StateResult<
        starknet_api::core::CompiledClassHash,
    > {
        self.inner.get_compiled_class_hash(class_hash)
    }
}

impl<T: StateReader + BlockifierState + HasBlockHash> BlockifierState
    for CachedState<T>
{
    fn set_storage_at(
        &mut self,
        contract_address: ContractAddress,
        storage_key: StorageKey,
        value: StarkFelt,
    ) -> blockifier::state::state_api::StateResult<()> {
        self.inner.set_storage_at(contract_address, storage_key, value)
    }

    fn increment_nonce(
        &mut self,
        contract_address: ContractAddress,
    ) -> blockifier::state::state_api::StateResult<()> {
        self.inner.increment_nonce(contract_address)
    }

    fn set_class_hash_at(
        &mut self,
        contract_address: ContractAddress,
        class_hash: starknet_api::core::ClassHash,
    ) -> blockifier::state::state_api::StateResult<()> {
        self.inner.set_class_hash_at(contract_address, class_hash)
    }

    fn set_contract_class(
        &mut self,
        class_hash: starknet_api::core::ClassHash,
        contract_class: blockifier::execution::contract_class::ContractClass,
    ) -> blockifier::state::state_api::StateResult<()> {
        self.inner.set_contract_class(class_hash, contract_class)
    }

    fn set_compiled_class_hash(
        &mut self,
        class_hash: starknet_api::core::ClassHash,
        compiled_class_hash: starknet_api::core::CompiledClassHash,
    ) -> blockifier::state::state_api::StateResult<()> {
        self.inner.set_compiled_class_hash(class_hash, compiled_class_hash)
    }

    fn add_visited_pcs(
        &mut self,
        class_hash: starknet_api::core::ClassHash,
        pcs: &std::collections::HashSet<usize>,
    ) {
        self.inner.add_visited_pcs(class_hash, pcs);
    }
}
