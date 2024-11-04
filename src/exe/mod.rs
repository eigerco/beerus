use std::{collections::HashSet, num::NonZeroU128, sync::Arc};

use blockifier::{
    blockifier::block::{BlockInfo, GasPrices},
    bouncer::BouncerConfig,
    context::{BlockContext, ChainInfo, FeeTokenAddresses, TransactionContext},
    execution::{
        call_info::CallInfo,
        common_hints::ExecutionMode,
        contract_class::ContractClass,
        entry_point::{CallEntryPoint, CallType, EntryPointExecutionContext},
    },
    state::{
        errors::StateError,
        state_api::{State as BlockifierState, StateReader, StateResult},
    },
    transaction::objects::{
        CommonAccountFields, DeprecatedTransactionInfo, TransactionInfo,
    },
    versioned_constants::VersionedConstants,
};
use starknet_api::{
    block::{BlockNumber as StarknetBlockNumber, BlockTimestamp},
    core::{
        ChainId as BlockifierChainId, ClassHash, CompiledClassHash,
        ContractAddress, EntryPointSelector, Nonce,
    },
    deprecated_contract_class::EntryPointType,
    hash::StarkHash,
    state::StorageKey as StarknetStorageKey,
    transaction::{
        Calldata, Fee, TransactionHash, TransactionSignature,
        TransactionVersion,
    },
};
use starknet_types_core::felt::Felt as StarkFelt;

use crate::{
    client::State,
    gen::{self, blocking::Rpc},
};

pub mod cache;
pub mod err;
pub mod map;

use err::Error;

pub fn call<T: gen::client::blocking::HttpClient>(
    client: gen::client::blocking::Client<T>,
    function_call: gen::FunctionCall,
    state: State,
) -> Result<CallInfo, Error> {
    let gen::FunctionCall { calldata, contract_address, entry_point_selector } =
        function_call;

    let calldata: Result<Vec<StarkFelt>, _> =
        calldata.into_iter().map(|felt| felt.try_into()).collect();

    let contract_address: StarkFelt = contract_address.0.try_into()?;

    let entry_point_selector: StarkFelt = entry_point_selector.try_into()?;

    let one = NonZeroU128::new(1)
        .ok_or_else(|| Error::Custom("NonZeroU128 is zero"))?;
    let block_info = BlockInfo {
        block_number: StarknetBlockNumber::default(),
        block_timestamp: BlockTimestamp::default(),
        sequencer_address: ContractAddress::default(),
        gas_prices: GasPrices {
            eth_l1_gas_price: one,
            strk_l1_gas_price: one,
            eth_l1_data_gas_price: one,
            strk_l1_data_gas_price: one,
        },
        use_kzg_da: false,
    };

    let chain_info = ChainInfo {
        chain_id: BlockifierChainId::Mainnet,
        fee_token_addresses: FeeTokenAddresses {
            strk_fee_token_address: ContractAddress::default(),
            eth_fee_token_address: ContractAddress::default(),
        },
    };

    let versioned_constants = VersionedConstants::latest_constants().to_owned();

    let bouncer_config = BouncerConfig::default();

    let block_context = BlockContext::new(
        block_info,
        chain_info,
        versioned_constants,
        bouncer_config,
    );

    let tx_info = TransactionInfo::Deprecated(DeprecatedTransactionInfo {
        common_fields: CommonAccountFields {
            transaction_hash: TransactionHash::default(),
            version: TransactionVersion(StarkFelt::ONE),
            signature: TransactionSignature(vec![
                StarkHash::ZERO,
                StarkHash::ZERO,
            ]),
            nonce: Nonce(StarkHash::ZERO),
            sender_address: ContractAddress::default(),
            only_query: true,
        },
        max_fee: Fee::default(),
    });

    let tx_context = Arc::new(TransactionContext { block_context, tx_info });
    let limit_steps_by_resources = false;
    let mut context = EntryPointExecutionContext::new(
        tx_context.clone(),
        ExecutionMode::Execute,
        limit_steps_by_resources,
    )?;

    let call_entry_point = CallEntryPoint {
        class_hash: None,
        code_address: None,
        entry_point_type: EntryPointType::External,
        entry_point_selector: EntryPointSelector(entry_point_selector),
        calldata: Calldata(Arc::new(calldata?)),
        storage_address: ContractAddress(contract_address.try_into()?),
        caller_address: ContractAddress::default(),
        call_type: CallType::Call,
        initial_gas: u64::MAX,
    };

    let state_proxy: StateProxy<T> = StateProxy { client, state };
    let mut state_proxy = cache::CachedState::new(state_proxy);

    let mut resources = Default::default();
    let call_info = call_entry_point.execute(
        &mut state_proxy,
        &mut resources,
        &mut context,
    )?;

    tracing::debug!(?call_info, "call completed");
    Ok(call_info)
}

struct StateProxy<T: gen::client::blocking::HttpClient> {
    client: gen::client::blocking::Client<T>,
    state: State,
}

impl<T: gen::client::blocking::HttpClient> cache::HasBlockHash
    for StateProxy<T>
{
    fn get_block_hash(&self) -> &gen::Felt {
        &self.state.block_hash
    }
}

impl<T: gen::client::blocking::HttpClient> StateReader for StateProxy<T> {
    fn get_storage_at(
        &self,
        contract_address: ContractAddress,
        storage_key: StarknetStorageKey,
    ) -> StateResult<StarkFelt> {
        tracing::info!(?contract_address, ?storage_key, "get_storage_at");

        let felt: gen::Felt = contract_address.0.key().try_into()?;
        let address = gen::Address(felt);

        let key = gen::StorageKey::try_new(&storage_key.0.to_string())
            .map_err(Into::<Error>::into)?;

        let block_id = gen::BlockId::BlockHash {
            block_hash: gen::BlockHash(self.state.block_hash.clone()),
        };

        let ret = self
            .client
            .getStorageAt(address.clone(), key.clone(), block_id.clone())
            .map_err(Into::<Error>::into)?;
        tracing::info!(?address, ?key, value=?ret, "get_storage_at");

        if ret.as_ref() == "0x0" {
            tracing::info!("get_storage_at: skipping proof for zero value");
            return Ok(ret.try_into()?);
        }

        let proof = self
            .client
            .getProof(block_id, address.clone(), vec![key.clone()])
            .map_err(Into::<Error>::into)?;
        tracing::info!("get_storage_at: proof received");

        let global_root = self.state.root.clone();
        let value = ret.clone();
        proof.verify(global_root, address, key, value).map_err(|e| {
            StateError::StateReadError(format!(
                "Failed to verify merkle proof: {e:?}"
            ))
        })?;
        tracing::info!("get_storage_at: proof verified");

        Ok(ret.try_into()?)
    }

    fn get_nonce_at(
        &self,
        contract_address: ContractAddress,
    ) -> StateResult<Nonce> {
        tracing::info!(?contract_address, "get_nonce_at");

        let block_id = gen::BlockId::BlockHash {
            block_hash: gen::BlockHash(self.state.block_hash.clone()),
        };

        let felt: gen::Felt = contract_address.0.key().try_into()?;
        let contract_address = gen::Address(felt);

        let ret = self
            .client
            .getNonce(block_id, contract_address)
            .map_err(Into::<Error>::into)?;

        Ok(Nonce(ret.try_into()?))
    }

    fn get_class_hash_at(
        &self,
        contract_address: ContractAddress,
    ) -> StateResult<ClassHash> {
        tracing::info!(?contract_address, "get_class_hash_at");

        let block_id = gen::BlockId::BlockHash {
            block_hash: gen::BlockHash(self.state.block_hash.clone()),
        };

        let felt: gen::Felt = contract_address.0.key().try_into()?;
        let contract_address = gen::Address(felt);

        let ret = self
            .client
            .getClassHashAt(block_id, contract_address)
            .map_err(Into::<Error>::into)?;

        Ok(ClassHash(ret.try_into()?))
    }

    fn get_compiled_contract_class(
        &self,
        class_hash: ClassHash,
    ) -> StateResult<ContractClass> {
        tracing::info!(?class_hash, "get_compiled_contract_class");

        let block_id = gen::BlockId::BlockHash {
            block_hash: gen::BlockHash(self.state.block_hash.clone()),
        };

        let class_hash: gen::Felt = class_hash.0.try_into()?;

        let ret = self
            .client
            .getClass(block_id, class_hash)
            .map_err(Into::<Error>::into)?;

        Ok(ret.try_into()?)
    }

    fn get_compiled_class_hash(
        &self,
        class_hash: ClassHash,
    ) -> StateResult<CompiledClassHash> {
        tracing::info!(?class_hash, "get_compiled_class_hash");
        Err(StateError::UndeclaredClassHash(class_hash))
    }
}

impl<T: gen::client::blocking::HttpClient> BlockifierState for StateProxy<T> {
    fn set_storage_at(
        &mut self,
        contract_address: ContractAddress,
        key: StarknetStorageKey,
        value: StarkFelt,
    ) -> StateResult<()> {
        tracing::info!(?contract_address, ?key, ?value, "set_storage_at");
        Ok(())
    }

    fn increment_nonce(
        &mut self,
        contract_address: ContractAddress,
    ) -> StateResult<()> {
        tracing::info!(?contract_address, "increment_nonce");
        Ok(())
    }

    fn set_class_hash_at(
        &mut self,
        contract_address: ContractAddress,
        class_hash: ClassHash,
    ) -> StateResult<()> {
        tracing::info!(?contract_address, ?class_hash, "set_class_hash_at");
        Ok(())
    }

    fn set_contract_class(
        &mut self,
        class_hash: ClassHash,
        contract_class: ContractClass,
    ) -> StateResult<()> {
        tracing::info!(?class_hash, ?contract_class, "set_contract_class");
        Ok(())
    }

    fn set_compiled_class_hash(
        &mut self,
        class_hash: ClassHash,
        compiled_class_hash: CompiledClassHash,
    ) -> StateResult<()> {
        tracing::info!(
            ?class_hash,
            ?compiled_class_hash,
            "set_compiled_class_hash"
        );
        Ok(())
    }

    fn add_visited_pcs(&mut self, class_hash: ClassHash, pcs: &HashSet<usize>) {
        tracing::info!(?class_hash, pcs.len = pcs.len(), "add_visited_pcs");
    }
}

#[cfg(test)]
mod tests {

    use blockifier::execution::contract_class::ContractClassV1;
    use starknet_api::core::PatriciaKey;

    use super::*;
    struct MockHttpClient;

    impl crate::gen::client::blocking::HttpClient for MockHttpClient {
        fn post(
            &self,
            _url: &str,
            _request: &iamgroot::jsonrpc::Request,
        ) -> std::result::Result<iamgroot::jsonrpc::Response, iamgroot::jsonrpc::Error> {
            Err(iamgroot::jsonrpc::Error {code: 0, message: "0".into()})
        }
    }

    #[test]
    fn test_luke() {
        let mock = MockHttpClient{};
        let mut proxy = StateProxy {
            client: gen::client::blocking::Client::new("test", mock),
            state: State {
                block_number: 0,
                block_hash: gen::Felt::try_new("0x0").unwrap(),
                root: gen::Felt::try_new("0x0").unwrap(),
            }
        };

        proxy.set_storage_at(
            ContractAddress(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
            StarknetStorageKey(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
            starknet_crypto::Felt::ZERO,
        ).unwrap();
        proxy.increment_nonce(
            ContractAddress(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
        ).unwrap();
        proxy.set_class_hash_at(
            ContractAddress(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
            ClassHash(starknet_crypto::Felt::ZERO),
        ).unwrap();
        proxy.set_contract_class(ClassHash(starknet_crypto::Felt::ZERO), ContractClass::V1(ContractClassV1::empty_for_testing())).unwrap();
        proxy.set_compiled_class_hash(ClassHash(starknet_crypto::Felt::ZERO), CompiledClassHash(starknet_crypto::Felt::ZERO)).unwrap();
        proxy.add_visited_pcs(ClassHash(starknet_crypto::Felt::ZERO), &HashSet::new());

    }
}