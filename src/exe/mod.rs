use std::{collections::HashSet, num::NonZeroU128, sync::Arc};

use blockifier::{
    block::{BlockInfo, GasPrices},
    context::{BlockContext, ChainInfo, FeeTokenAddresses, TransactionContext},
    execution::{
        call_info::CallInfo,
        common_hints::ExecutionMode,
        contract_class::{ContractClass, ContractClassV0, ContractClassV1},
        entry_point::{CallEntryPoint, CallType, EntryPointExecutionContext},
        syscalls::hint_processor::SyscallHintProcessor,
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
use cairo_vm::{
    felt::Felt252,
    serde::deserialize_program::BuiltinName,
    types::relocatable::MaybeRelocatable,
    vm::{
        runners::cairo_runner::{CairoArg, CairoRunner, ExecutionResources},
        vm_core::VirtualMachine,
    },
};
use starknet_api::{
    block::{BlockNumber as StarknetBlockNumber, BlockTimestamp},
    core::{
        ChainId as BlockifierChainId, ClassHash, CompiledClassHash,
        ContractAddress, EntryPointSelector, Nonce,
    },
    deprecated_contract_class::EntryPointType,
    hash::{StarkFelt, StarkHash},
    stark_felt,
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
        function_call.clone();

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

    // let call_info =
    //     call_entry_point.execute(&mut proxy, &mut resources, &mut context)?;

    let contract_address: StarkFelt =
        function_call.contract_address.0.clone().try_into()?;
    let contract_address: ContractAddress = contract_address.try_into()?;
    let class_hash = proxy.get_class_hash_at(contract_address)?;
    let contract_class = proxy.get_compiled_contract_class(class_hash)?;

    let call_info = match contract_class {
        ContractClass::V0(contract_class) => call_contract_v0(
            &mut proxy,
            &mut resources,
            &mut context,
            call_entry_point.clone(),
            contract_class,
            class_hash,
        )?,
        ContractClass::V1(contract_class) => call_contract_v1(
            &mut proxy,
            &mut resources,
            &mut context,
            call_entry_point.clone(),
            contract_class,
            class_hash,
        )?,
    };

    tracing::debug!(?call_info, "call completed");
    Ok(call_info)
}

#[allow(unused_variables)]
fn call_contract_v0(
    proxy: &mut dyn blockifier::state::state_api::State,
    resources: &mut ExecutionResources,
    context: &mut EntryPointExecutionContext,
    call_entry_point: CallEntryPoint,
    contract_class: ContractClassV0,
    class_hash: ClassHash,
) -> Result<CallInfo, Error> {
    // TODO: impl
    unimplemented!()
}

fn call_contract_v1(
    proxy: &mut dyn blockifier::state::state_api::State,
    resources: &mut ExecutionResources,
    context: &mut EntryPointExecutionContext,
    call_entry_point: CallEntryPoint,
    contract_class: ContractClassV1,
    class_hash: ClassHash,
) -> Result<CallInfo, Error> {
    // CALL: prepare_program_extra_data
    let proof_mode = false;
    let mut runner =
        CairoRunner::new(&contract_class.program, "starknet", proof_mode)?;
    let trace_enabled = true;
    let mut vm = VirtualMachine::new(trace_enabled);

    let program_builtins = [
        BuiltinName::bitwise,
        BuiltinName::ec_op,
        BuiltinName::ecdsa,
        BuiltinName::output,
        BuiltinName::pedersen,
        BuiltinName::poseidon,
        BuiltinName::range_check,
        BuiltinName::segment_arena,
    ];
    runner.initialize_function_runner_cairo_1(&mut vm, &program_builtins)?;

    let entry_point = contract_class.get_entry_point(&call_entry_point)?;

    let start_ptr = vm.add_memory_segment();
    let data = vec![MaybeRelocatable::from(0); 20];
    vm.load_data(start_ptr, &data)?;

    // Put a pointer to the builtin cost segment at the end of the program (after the
    // additional `ret` statement).
    let mut ptr = (vm.get_pc() + contract_class.bytecode_length())?;
    vm.insert_value(
        ptr,
        Felt252::from_bytes_be(stark_felt!("0x208b7fff7fff7ffe").bytes()),
    )?;
    ptr += 1;
    vm.insert_value(ptr, start_ptr)?; // Push a pointer to the builtin cost segment.
    let program_extra_data_length = 2;

    // Instantiate syscall handler.
    let initial_syscall_ptr = vm.add_memory_segment();
    let mut syscall_handler = SyscallHintProcessor::new(
        proxy,
        resources,
        context,
        initial_syscall_ptr,
        call_entry_point,
        &contract_class.hints,
        Default::default(),
    );
    // END CALL: prepare_program_extra_data

    // CALL: prepare_call_arguments
    let call = &syscall_handler.call;
    let mut args: Vec<CairoArg> = vec![];
    for builtin_name in &entry_point.builtins {
        if let Some(builtin) = vm
            .get_builtin_runners()
            .iter()
            .find(|builtin| builtin.name() == builtin_name)
        {
            args.extend(
                builtin.initial_stack().into_iter().map(CairoArg::Single),
            );
            continue;
        }
        if builtin_name == "segment_arena_builtin" {
            let segment_arena = vm.add_memory_segment();

            // Write into segment_arena.
            let mut ptr = segment_arena;
            let info_segment = vm.add_memory_segment();
            vm.insert_value(ptr, info_segment)?;
            ptr += 1;
            let n_constructed = StarkFelt::default();
            vm.insert_value(
                ptr,
                Felt252::from_bytes_be(n_constructed.bytes()),
            )?;
            ptr += 1;
            let n_destructed = StarkFelt::default();
            vm.insert_value(ptr, Felt252::from_bytes_be(n_destructed.bytes()))?;

            args.push(CairoArg::Single(MaybeRelocatable::from(ptr)));
            continue;
        }
        return Err(Error::Custom("invalid builtin"));
    }
    // Push gas counter.
    args.push(CairoArg::Single(MaybeRelocatable::from(Felt252::from(
        call.initial_gas,
    ))));
    // Push syscall ptr.
    args.push(CairoArg::Single(MaybeRelocatable::from(initial_syscall_ptr)));

    // Prepare calldata arguments.
    let calldata = &call.calldata.0;
    let calldata: Vec<MaybeRelocatable> = calldata
        .iter()
        .map(|&arg| MaybeRelocatable::from(Felt252::from_bytes_be(arg.bytes())))
        .collect();

    let calldata_start_ptr = {
        let start_ptr = vm.add_memory_segment();
        vm.load_data(start_ptr, &calldata)?;
        start_ptr
    };
    let calldata_end_ptr =
        MaybeRelocatable::from((calldata_start_ptr + calldata.len())?);
    args.push(CairoArg::Single(MaybeRelocatable::from(calldata_start_ptr)));
    args.push(CairoArg::Single(calldata_end_ptr));
    // END CALL: prepare_call_arguments

    let n_total_args = args.len();
    // Fix the resources, in order to calculate the usage of this run at the end.
    let previous_resources = syscall_handler.resources.clone();
    // Execute.
    let bytecode_length = contract_class.bytecode_length();
    let program_segment_size = bytecode_length + program_extra_data_length;

    // CALL: run_entry_point
    let verify_secure = true;
    let args: Vec<&CairoArg> = args.iter().collect();
    runner.run_from_entrypoint(
        entry_point.pc(),
        &args,
        verify_secure,
        Some(program_segment_size),
        &mut vm,
        &mut syscall_handler,
    )?;
    // END CALL: run_entry_point

    // CALL: register_visited_pcs
    let mut class_visited_pcs = HashSet::new();
    vm.relocate_trace(&[1, 1 + program_segment_size])?;
    for trace_entry in vm.get_relocated_trace()? {
        let pc = trace_entry.pc;
        if pc < 1 {
            return Err(Error::Custom("invalid pc"));
        }
        let real_pc = pc - 1;
        // Jumping to a PC that is not inside the bytecode is possible. For example, to obtain
        // the builtin costs. Filter out these values.
        if real_pc < bytecode_length {
            class_visited_pcs.insert(real_pc);
        }
    }
    syscall_handler.state.add_visited_pcs(class_hash, &class_visited_pcs);
    // END CALL: register_visited_pcs

    // TODO: unwrap the call
    let call_info =
        blockifier::execution::entry_point_execution::finalize_execution(
            vm,
            runner,
            syscall_handler,
            previous_resources,
            n_total_args,
            program_extra_data_length,
        )?;
    if call_info.execution.failed {
        // error_data = call_info.execution.retdata.0,
        return Err(Error::Custom("execution failed"));
    }

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

        // TODO: find more elegant way for this
        // workaround to skip proof validation for testing
        #[cfg(feature = "skip-zero-root-validation")]
        if self.state_root.as_ref() == "0x0" {
            return Ok(ret.try_into()?);
        }

        let proof = self
            .client
            .getProof(
                block_id.clone(),
                contract_address.clone(),
                vec![key.clone()],
            )
            .map_err(Into::<Error>::into)?;
        tracing::info!(?proof, "get_storage_at: proof received");

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
