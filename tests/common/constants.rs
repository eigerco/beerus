use beerus::gen::{
    Address, BroadcastedDeclareTxnV2, BroadcastedDeclareTxnV2Type,
    BroadcastedDeclareTxnV2Version, ContractClass,
    ContractClassEntryPointsByType, Felt, SierraEntryPoint,
};

#[allow(dead_code)]
pub fn declare_transaction_v2() -> BroadcastedDeclareTxnV2 {
    BroadcastedDeclareTxnV2 {
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
        max_fee: Felt::try_new("0x0").unwrap(),
        nonce: Felt::try_new("0x0").unwrap(),
        r#type: BroadcastedDeclareTxnV2Type::Declare,
        signature: vec![Felt::try_new("0x5").unwrap()],
        sender_address: Address(Felt::try_new("0x6").unwrap()),
        version: BroadcastedDeclareTxnV2Version::V0x2,
    }
}
