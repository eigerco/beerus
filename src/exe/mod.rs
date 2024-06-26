use std::{collections::HashSet, num::NonZeroU128, sync::Arc};

use blockifier::{
    block::{BlockInfo, GasPrices},
    context::{BlockContext, ChainInfo, FeeTokenAddresses, TransactionContext},
    execution::{
        call_info::CallInfo,
        common_hints::ExecutionMode,
        contract_class::ContractClass,
        entry_point::{CallEntryPoint, CallType, EntryPointExecutionContext},
    },
    state::{
        cached_state::CommitmentStateDiff,
        errors::StateError,
        state_api::{State as BlockifierState, StateReader, StateResult},
    },
    transaction::objects::{
        CommonAccountFields, DeprecatedTransactionInfo, TransactionInfo,
    },
    versioned_constants::VersionedConstants,
};
use cairo_vm::vm::runners::cairo_runner::ExecutionResources;
use starknet_api::{
    block::{BlockNumber as StarknetBlockNumber, BlockTimestamp},
    core::{
        ChainId as BlockifierChainId, ClassHash, CompiledClassHash,
        ContractAddress, EntryPointSelector, Nonce,
    },
    deprecated_contract_class::EntryPointType,
    hash::{StarkFelt, StarkHash},
    state::StorageKey as StarknetStorageKey,
    transaction::{
        Calldata, Fee, TransactionHash, TransactionSignature,
        TransactionVersion,
    },
};

use crate::gen::{self, blocking::Rpc};

pub mod err;
pub mod map;

use err::Error;

pub fn call(
    client: &gen::client::blocking::Client,
    function_call: gen::FunctionCall,
    state_root: gen::Felt,
) -> Result<CallInfo, Error> {
    let gen::FunctionCall { calldata, contract_address, entry_point_selector } =
        function_call;

    let calldata: Result<Vec<StarkFelt>, _> =
        calldata.into_iter().map(|felt| felt.try_into()).collect();

    let contract_address: StarkFelt = contract_address.0.try_into()?;

    let entry_point_selector: StarkFelt = entry_point_selector.try_into()?;

    let mut resources = ExecutionResources::default();

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
        chain_id: BlockifierChainId("00".to_owned()),
        fee_token_addresses: FeeTokenAddresses {
            strk_fee_token_address: ContractAddress::default(),
            eth_fee_token_address: ContractAddress::default(),
        },
    };

    let versioned_constants = VersionedConstants::latest_constants();

    let block_context = BlockContext::new_unchecked(
        &block_info,
        &chain_info,
        versioned_constants,
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

    let mut context = EntryPointExecutionContext::new(
        tx_context.clone(),
        ExecutionMode::Execute,
        /*limit_steps_by_resources=*/ false,
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

    let diff = CommitmentStateDiff {
        storage_updates: Default::default(),
        address_to_nonce: Default::default(),
        address_to_class_hash: Default::default(),
        class_hash_to_compiled_class_hash: Default::default(),
    };
    let mut proxy = StateProxy { client: client.to_owned(), diff, state_root };

    let call_info =
        call_entry_point.execute(&mut proxy, &mut resources, &mut context)?;

    tracing::debug!(?call_info, "call completed");
    Ok(call_info)
}

struct StateProxy {
    client: gen::client::blocking::Client,
    diff: CommitmentStateDiff,
    state_root: gen::Felt,
}

impl StateReader for StateProxy {
    fn get_storage_at(
        &mut self,
        contract_address: ContractAddress,
        key: StarknetStorageKey,
    ) -> StateResult<StarkFelt> {
        tracing::info!(?contract_address, ?key, "get_storage_at");

        let felt: gen::Felt = contract_address.0.key().try_into()?;
        let contract_address = gen::Address(felt);

        let key = gen::StorageKey::try_new(&key.0.to_string())
            .map_err(Into::<Error>::into)?;

        let block_id = gen::BlockId::BlockTag(gen::BlockTag::Latest);

        let ret = self
            .client
            .getStorageAt(
                contract_address.clone(),
                key.clone(),
                block_id.clone(),
            )
            .map_err(Into::<Error>::into)?;

        let proof = self
            .client
            .getProof(
                block_id.clone(),
                contract_address.clone(),
                vec![key.clone()],
            )
            .map_err(Into::<Error>::into)?;
        tracing::info!(?proof, "get_storage_at: proof received");

        // TODO: find more elegant way for this
        // workaround to skip proof validation for testing
        #[cfg(feature = "skip-zero-root-validation")]
        if self.state_root.as_ref() == "0x0" {
            return Ok(ret.try_into()?);
        }

        let global_root = self.state_root.clone();
        let value = ret.clone();
        proof.verify(global_root, contract_address, key, value).map_err(
            |_| StateError::StateReadError("Invalid merkle proof".to_owned()),
        )?;
        tracing::info!("get_storage_at: proof verified");

        Ok(ret.try_into()?)
    }

    fn get_nonce_at(
        &mut self,
        contract_address: ContractAddress,
    ) -> StateResult<Nonce> {
        tracing::info!(?contract_address, "get_nonce_at");

        let block_id = gen::BlockId::BlockTag(gen::BlockTag::Latest);

        let felt: gen::Felt = contract_address.0.key().try_into()?;
        let contract_address = gen::Address(felt);

        let ret = self
            .client
            .getNonce(block_id, contract_address)
            .map_err(Into::<Error>::into)?;

        Ok(Nonce(ret.try_into()?))
    }

    fn get_class_hash_at(
        &mut self,
        contract_address: ContractAddress,
    ) -> StateResult<ClassHash> {
        tracing::info!(?contract_address, "get_class_hash_at");

        let block_id = gen::BlockId::BlockTag(gen::BlockTag::Latest);

        let felt: gen::Felt = contract_address.0.key().try_into()?;
        let contract_address = gen::Address(felt);

        let ret = self
            .client
            .getClassHashAt(block_id, contract_address)
            .map_err(Into::<Error>::into)?;

        Ok(ClassHash(ret.try_into()?))
    }

    fn get_compiled_contract_class(
        &mut self,
        class_hash: ClassHash,
    ) -> StateResult<ContractClass> {
        tracing::info!(?class_hash, "get_compiled_contract_class");

        let block_id = gen::BlockId::BlockTag(gen::BlockTag::Latest);

        let class_hash: gen::Felt = class_hash.0.try_into()?;

        let ret = self
            .client
            .getClass(block_id, class_hash)
            .map_err(Into::<Error>::into)?;

        Ok(ret.try_into()?)
    }

    fn get_compiled_class_hash(
        &mut self,
        class_hash: ClassHash,
    ) -> StateResult<CompiledClassHash> {
        tracing::info!(?class_hash, "get_compiled_class_hash");
        Err(StateError::UndeclaredClassHash(class_hash))
    }
}

impl BlockifierState for StateProxy {
    fn set_storage_at(
        &mut self,
        contract_address: ContractAddress,
        key: StarknetStorageKey,
        value: StarkFelt,
    ) -> StateResult<()> {
        tracing::info!(?contract_address, ?key, ?value, "set_storage_at");
        self.diff
            .storage_updates
            .entry(contract_address)
            .or_default()
            .entry(key)
            .or_insert(value);
        Ok(())
    }

    fn increment_nonce(
        &mut self,
        contract_address: ContractAddress,
    ) -> StateResult<()> {
        tracing::info!(?contract_address, "increment_nonce");
        let nonce: &mut Nonce =
            self.diff.address_to_nonce.entry(contract_address).or_default();
        let value = *nonce;
        *nonce = value.try_increment()?;
        Ok(())
    }

    fn set_class_hash_at(
        &mut self,
        contract_address: ContractAddress,
        class_hash: ClassHash,
    ) -> StateResult<()> {
        tracing::info!(?contract_address, ?class_hash, "set_class_hash_at");
        *self.diff.address_to_class_hash.entry(contract_address).or_default() =
            class_hash;
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
        *self
            .diff
            .class_hash_to_compiled_class_hash
            .entry(class_hash)
            .or_default() = compiled_class_hash;
        Ok(())
    }

    fn to_state_diff(&mut self) -> CommitmentStateDiff {
        tracing::info!("to_state_diff");
        self.diff.clone()
    }

    fn add_visited_pcs(&mut self, class_hash: ClassHash, pcs: &HashSet<usize>) {
        tracing::info!(?class_hash, pcs.len = pcs.len(), "add_visited_pcs");
    }
}
