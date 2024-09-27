use beerus::gen::{
    Address, BroadcastedDeclareTxn, BroadcastedDeclareTxnV3,
    BroadcastedDeclareTxnV3Type, BroadcastedDeclareTxnV3Version,
    BroadcastedDeployAccountTxn, BroadcastedInvokeTxn, BroadcastedTxn,
    ContractClass, ContractClassEntryPointsByType, DaMode, DeployAccountTxn,
    DeployAccountTxnV3, DeployAccountTxnV3Type, DeployAccountTxnV3Version,
    Felt, InvokeTxn, InvokeTxnV3, InvokeTxnV3Type, InvokeTxnV3Version,
    ResourceBounds, ResourceBoundsMapping, SierraEntryPoint, U128, U64,
};

#[allow(dead_code)]
pub fn declare_transaction() -> BroadcastedDeclareTxn {
    BroadcastedDeclareTxn::BroadcastedDeclareTxnV3(
        dummy_declare_transaction_v3(),
    )
}

#[allow(dead_code)]
pub fn estimate_fee_transaction() -> BroadcastedTxn {
    BroadcastedTxn::BroadcastedDeclareTxn(
        BroadcastedDeclareTxn::BroadcastedDeclareTxnV3(
            dummy_declare_transaction_v3(),
        ),
    )
}

#[allow(dead_code)]
pub fn invoke_transaction() -> BroadcastedInvokeTxn {
    BroadcastedInvokeTxn(InvokeTxn::InvokeTxnV3(dummy_invoke_transaction_v3()))
}

#[allow(dead_code)]
pub fn deploy_transaction() -> BroadcastedDeployAccountTxn {
    BroadcastedDeployAccountTxn(DeployAccountTxn::DeployAccountTxnV3(
        dummy_deploy_transaction_v3(),
    ))
}

#[allow(dead_code)]
fn dummy_declare_transaction_v3() -> BroadcastedDeclareTxnV3 {
    BroadcastedDeclareTxnV3 {
        account_deployment_data: vec![Felt::try_new("0x0").unwrap()],
        compiled_class_hash: Felt::try_new("0x0").unwrap(),
        contract_class: ContractClass {
            sierra_program: vec![Felt::try_new("0x1").unwrap()],
            contract_class_version: "0.1.0".to_string(),
            entry_points_by_type: ContractClassEntryPointsByType {
                constructor: vec![SierraEntryPoint {
                    selector: Felt::try_new("0x2").unwrap(),
                    function_idx: 2,
                }],
                external: vec![
                    SierraEntryPoint {
                        selector: Felt::try_new("0x3").unwrap(),
                        function_idx: 3,
                    },
                    SierraEntryPoint {
                        selector: Felt::try_new("0x4").unwrap(),
                        function_idx: 4,
                    },
                ],
                l1_handler: vec![],
            },
            abi: Some("some_abi".to_string()),
        },
        fee_data_availability_mode: DaMode::L1,
        nonce: Felt::try_new("0x0").unwrap(),
        r#type: BroadcastedDeclareTxnV3Type::Declare,
        signature: vec![Felt::try_new("0x5").unwrap()],
        sender_address: Address(Felt::try_new("0x6").unwrap()),
        version:
            BroadcastedDeclareTxnV3Version::V0x100000000000000000000000000000003,
        nonce_data_availability_mode: DaMode::L1,
        paymaster_data: vec![Felt::try_new("0x7").unwrap()],
        resource_bounds: ResourceBoundsMapping {
            l1_gas: ResourceBounds {
                max_amount: U64::try_new("0x0").unwrap(),
                max_price_per_unit: U128::try_new("0x0").unwrap(),
            },
            l2_gas: ResourceBounds {
                max_amount: U64::try_new("0x0").unwrap(),
                max_price_per_unit: U128::try_new("0x0").unwrap(),
            },
        },
        tip: U64::try_new("0x0").unwrap(),
    }
}

#[allow(dead_code)]
fn dummy_invoke_transaction_v3() -> InvokeTxnV3 {
    InvokeTxnV3 {
        account_deployment_data: vec![Felt::try_new("0x0").unwrap()],
        calldata: vec![Felt::try_new("0x1").unwrap()],
        fee_data_availability_mode: DaMode::L1,
        nonce: Felt::try_new("0x2").unwrap(),
        nonce_data_availability_mode: DaMode::L1,
        paymaster_data: vec![Felt::try_new("0x1").unwrap()],
        r#type: InvokeTxnV3Type::Invoke,
        resource_bounds: ResourceBoundsMapping {
            l1_gas: ResourceBounds {
                max_amount: U64::try_new("0x0").unwrap(),
                max_price_per_unit: U128::try_new("0x0").unwrap(),
            },
            l2_gas: ResourceBounds {
                max_amount: U64::try_new("0x0").unwrap(),
                max_price_per_unit: U128::try_new("0x0").unwrap(),
            },
        },
        sender_address: Address(Felt::try_new("0x3").unwrap()),
        signature: vec![Felt::try_new("0x4").unwrap()],
        tip: U64::try_new("0x0").unwrap(),
        version: InvokeTxnV3Version::V0x3,
    }
}

#[allow(dead_code)]
fn dummy_deploy_transaction_v3() -> DeployAccountTxnV3 {
    DeployAccountTxnV3 {
        class_hash: Felt::try_new("0x0").unwrap(),
        constructor_calldata: vec![Felt::try_new("0x1").unwrap()],
        contract_address_salt: Felt::try_new("0x2").unwrap(),
        fee_data_availability_mode: DaMode::L1,
        nonce: Felt::try_new("0x3").unwrap(),
        nonce_data_availability_mode: DaMode::L1,
        paymaster_data: vec![Felt::try_new("0x4").unwrap()],
        r#type: DeployAccountTxnV3Type::DeployAccount,
        resource_bounds: ResourceBoundsMapping {
            l1_gas: ResourceBounds {
                max_amount: U64::try_new("0x0").unwrap(),
                max_price_per_unit: U128::try_new("0x0").unwrap(),
            },
            l2_gas: ResourceBounds {
                max_amount: U64::try_new("0x0").unwrap(),
                max_price_per_unit: U128::try_new("0x0").unwrap(),
            },
        },
        signature: vec![Felt::try_new("0x5").unwrap()],
        tip: U64::try_new("0x0").unwrap(),
        version: DeployAccountTxnV3Version::V0x3,
    }
}
