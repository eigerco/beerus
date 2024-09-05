use beerus::gen::{
    Address, BroadcastedDeclareTxnV3, BroadcastedDeclareTxnV3Type,
    BroadcastedDeclareTxnV3Version, ContractClass,
    ContractClassEntryPointsByType, DaMode, Felt, ResourceBounds,
    ResourceBoundsMapping, SierraEntryPoint, U128, U64,
};

#[allow(dead_code)]
pub const COMPILED_ACCOUNT_CONTRACT: &str =
    include_str!("../clob/compiled_account_contract.txt");
#[allow(dead_code)]
pub const DECLARE_ACCOUNT: &str = include_str!("../clob/declare_account.txt");

#[allow(dead_code)]
pub fn dummy_transaction_v3() -> BroadcastedDeclareTxnV3 {
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
