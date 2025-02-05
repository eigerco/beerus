pub use gen::*;

// TODO: must be handled in iamgroot
#[allow(clippy::needless_return)]
// vvv GENERATED CODE BELOW vvv
#[allow(clippy::module_inception)]
#[allow(non_snake_case)]
#[allow(clippy::enum_variant_names)]
#[allow(clippy::large_enum_variant)]
pub mod gen {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use iamgroot::jsonrpc;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Address(pub Felt);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BinaryNode {
        pub binary: BinaryNodeBinary,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BinaryNodeBinary {
        pub left: Felt,
        pub right: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockBodyWithReceipts {
        pub transactions: Vec<TransactionAndReceipt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockBodyWithTxHashes {
        pub transactions: Vec<TxnHash>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockBodyWithTxs {
        pub transactions: Vec<TransactionsInBlock>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockHash(pub Felt);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockHeader {
        pub block_hash: BlockHash,
        pub block_number: BlockNumber,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub l1_da_mode: Option<BlockHeaderL1DaMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub l1_data_gas_price: Option<ResourcePrice>,
        pub l1_gas_price: ResourcePrice,
        pub new_root: Felt,
        pub parent_hash: BlockHash,
        pub sequencer_address: Felt,
        pub starknet_version: String,
        pub timestamp: BlockHeaderTimestamp,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BlockHeaderL1DaMode {
        #[serde(rename = "BLOB")]
        Blob,
        #[serde(rename = "CALLDATA")]
        Calldata,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "i64")]
    pub struct BlockHeaderTimestamp(i64);

    mod blockheadertimestamp {
        use super::jsonrpc;
        use super::BlockHeaderTimestamp;

        static MIN: i64 = 0;
        static MAX: i64 = 9223372036854775807;

        impl BlockHeaderTimestamp {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("BlockHeaderTimestamp value {value} must be > {MIN}"),
                });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("BlockHeaderTimestamp value {value} must be < {MAX}"),
                });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for BlockHeaderTimestamp {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for BlockHeaderTimestamp {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BlockId {
        BlockHash { block_hash: BlockHash },
        BlockNumber { block_number: BlockNumber },
        BlockTag(BlockTag),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "i64")]
    pub struct BlockNumber(i64);

    mod blocknumber {
        use super::jsonrpc;
        use super::BlockNumber;

        static MIN: i64 = 0;
        static MAX: i64 = 9223372036854775807;

        impl BlockNumber {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "BlockNumber value {value} must be > {MIN}"
                        ),
                    });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "BlockNumber value {value} must be < {MAX}"
                        ),
                    });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for BlockNumber {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for BlockNumber {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BlockStatus {
        #[serde(rename = "PENDING")]
        Pending,
        #[serde(rename = "ACCEPTED_ON_L2")]
        AcceptedOnL2,
        #[serde(rename = "ACCEPTED_ON_L1")]
        AcceptedOnL1,
        #[serde(rename = "REJECTED")]
        Rejected,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BlockTag {
        #[serde(rename = "latest")]
        Latest,
        #[serde(rename = "pending")]
        Pending,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockWithReceipts {
        pub status: BlockStatus,
        #[serde(flatten)]
        pub block_header: BlockHeader,
        #[serde(flatten)]
        pub block_body_with_receipts: BlockBodyWithReceipts,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockWithTxHashes {
        pub status: BlockStatus,
        #[serde(flatten)]
        pub block_header: BlockHeader,
        #[serde(flatten)]
        pub block_body_with_tx_hashes: BlockBodyWithTxHashes,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockWithTxs {
        pub status: BlockStatus,
        #[serde(flatten)]
        pub block_header: BlockHeader,
        #[serde(flatten)]
        pub block_body_with_txs: BlockBodyWithTxs,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BroadcastedDeclareTxn {
        BroadcastedDeclareTxnV1(BroadcastedDeclareTxnV1),
        BroadcastedDeclareTxnV2(BroadcastedDeclareTxnV2),
        BroadcastedDeclareTxnV3(BroadcastedDeclareTxnV3),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BroadcastedDeclareTxnV1 {
        pub contract_class: DeprecatedContractClass,
        pub max_fee: Felt,
        pub nonce: Felt,
        pub r#type: BroadcastedDeclareTxnV1Type,
        pub sender_address: Address,
        pub signature: Signature,
        pub version: BroadcastedDeclareTxnV1Version,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BroadcastedDeclareTxnV1Type {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BroadcastedDeclareTxnV1Version {
        #[serde(rename = "0x1")]
        V0x1,
        #[serde(rename = "0x100000000000000000000000000000001")]
        V0x100000000000000000000000000000001,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BroadcastedDeclareTxnV2 {
        pub compiled_class_hash: Felt,
        pub contract_class: ContractClass,
        pub max_fee: Felt,
        pub nonce: Felt,
        pub r#type: BroadcastedDeclareTxnV2Type,
        pub sender_address: Address,
        pub signature: Signature,
        pub version: BroadcastedDeclareTxnV2Version,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BroadcastedDeclareTxnV2Type {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BroadcastedDeclareTxnV2Version {
        #[serde(rename = "0x2")]
        V0x2,
        #[serde(rename = "0x100000000000000000000000000000002")]
        V0x100000000000000000000000000000002,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BroadcastedDeclareTxnV3 {
        pub account_deployment_data: Vec<Felt>,
        pub compiled_class_hash: Felt,
        pub contract_class: ContractClass,
        pub fee_data_availability_mode: DaMode,
        pub nonce: Felt,
        pub nonce_data_availability_mode: DaMode,
        pub paymaster_data: Vec<Felt>,
        pub r#type: BroadcastedDeclareTxnV3Type,
        pub resource_bounds: ResourceBoundsMapping,
        pub sender_address: Address,
        pub signature: Signature,
        pub tip: U64,
        pub version: BroadcastedDeclareTxnV3Version,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BroadcastedDeclareTxnV3Type {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum BroadcastedDeclareTxnV3Version {
        #[serde(rename = "0x3")]
        V0x3,
        #[serde(rename = "0x100000000000000000000000000000003")]
        V0x100000000000000000000000000000003,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BroadcastedDeployAccountTxn(pub DeployAccountTxn);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BroadcastedInvokeTxn(pub InvokeTxn);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum BroadcastedTxn {
        BroadcastedInvokeTxn(BroadcastedInvokeTxn),
        BroadcastedDeclareTxn(BroadcastedDeclareTxn),
        BroadcastedDeployAccountTxn(BroadcastedDeployAccountTxn),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum CallType {
        #[serde(rename = "LIBRARY_CALL")]
        LibraryCall,
        #[serde(rename = "CALL")]
        Call,
        #[serde(rename = "DELEGATE")]
        Delegate,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "String")]
    pub struct ChainId(String);

    mod chainid {
        use super::jsonrpc;
        use super::ChainId;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static CHAINID_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new("^0x[a-fA-F0-9]+$").expect("ChainId: valid regex")
        });

        impl ChainId {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if CHAINID_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "ChainId value does not match regex: {value}"
                        ),
                    })
                }
            }
        }

        impl TryFrom<String> for ChainId {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for ChainId {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CommonReceiptProperties {
        pub actual_fee: FeePayment,
        pub events: Vec<Event>,
        pub execution_resources: ExecutionResources,
        pub finality_status: TxnFinalityStatus,
        pub messages_sent: Vec<MsgToL1>,
        pub transaction_hash: TxnHash,
        #[serde(flatten)]
        pub result_common_receipt_properties: ResultCommonReceiptProperties,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ComputationResources {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub bitwise_builtin_applications: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub ec_op_builtin_applications: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub ecdsa_builtin_applications: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub keccak_builtin_applications: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub memory_holes: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub pedersen_builtin_applications: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub poseidon_builtin_applications: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub range_check_builtin_applications: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub segment_arena_builtin: Option<i64>,
        pub steps: i64,
    }

    type ContractAbi = Vec<ContractAbiEntry>;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum ContractAbiEntry {
        FunctionAbiEntry(FunctionAbiEntry),
        EventAbiEntry(EventAbiEntry),
        StructAbiEntry(StructAbiEntry),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ContractClass {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub abi: Option<String>,
        pub contract_class_version: String,
        pub entry_points_by_type: ContractClassEntryPointsByType,
        pub sierra_program: Vec<Felt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ContractClassEntryPointsByType {
        #[serde(rename = "CONSTRUCTOR")]
        pub constructor: Vec<SierraEntryPoint>,
        #[serde(rename = "EXTERNAL")]
        pub external: Vec<SierraEntryPoint>,
        #[serde(rename = "L1_HANDLER")]
        pub l1_handler: Vec<SierraEntryPoint>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ContractStorageDiffItem {
        pub address: Felt,
        pub storage_entries: Vec<StorageDiffItem>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DaMode {
        #[serde(rename = "L1")]
        L1,
        #[serde(rename = "L2")]
        L2,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum DeclareTxn {
        DeclareTxnV0(DeclareTxnV0),
        DeclareTxnV1(DeclareTxnV1),
        DeclareTxnV2(DeclareTxnV2),
        DeclareTxnV3(DeclareTxnV3),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclareTxnReceipt {
        pub r#type: DeclareTxnReceiptType,
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnReceiptType {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclareTxnTrace {
        pub execution_resources: ExecutionResources,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub fee_transfer_invocation: Option<FunctionInvocation>,
        pub r#type: DeclareTxnTraceType,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub state_diff: Option<StateDiff>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub validate_invocation: Option<FunctionInvocation>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnTraceType {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclareTxnV0 {
        pub class_hash: Felt,
        pub max_fee: Felt,
        pub r#type: DeclareTxnV0Type,
        pub sender_address: Address,
        pub signature: Signature,
        pub version: DeclareTxnV0Version,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV0Type {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV0Version {
        #[serde(rename = "0x0")]
        V0x0,
        #[serde(rename = "0x100000000000000000000000000000000")]
        V0x100000000000000000000000000000000,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclareTxnV1 {
        pub class_hash: Felt,
        pub max_fee: Felt,
        pub nonce: Felt,
        pub r#type: DeclareTxnV1Type,
        pub sender_address: Address,
        pub signature: Signature,
        pub version: DeclareTxnV1Version,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV1Type {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV1Version {
        #[serde(rename = "0x1")]
        V0x1,
        #[serde(rename = "0x100000000000000000000000000000001")]
        V0x100000000000000000000000000000001,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclareTxnV2 {
        pub class_hash: Felt,
        pub compiled_class_hash: Felt,
        pub max_fee: Felt,
        pub nonce: Felt,
        pub r#type: DeclareTxnV2Type,
        pub sender_address: Address,
        pub signature: Signature,
        pub version: DeclareTxnV2Version,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV2Type {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV2Version {
        #[serde(rename = "0x2")]
        V0x2,
        #[serde(rename = "0x100000000000000000000000000000002")]
        V0x100000000000000000000000000000002,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeclareTxnV3 {
        pub account_deployment_data: Vec<Felt>,
        pub class_hash: Felt,
        pub compiled_class_hash: Felt,
        pub fee_data_availability_mode: DaMode,
        pub nonce: Felt,
        pub nonce_data_availability_mode: DaMode,
        pub paymaster_data: Vec<Felt>,
        pub r#type: DeclareTxnV3Type,
        pub resource_bounds: ResourceBoundsMapping,
        pub sender_address: Address,
        pub signature: Signature,
        pub tip: U64,
        pub version: DeclareTxnV3Version,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV3Type {
        #[serde(rename = "DECLARE")]
        Declare,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeclareTxnV3Version {
        #[serde(rename = "0x3")]
        V0x3,
        #[serde(rename = "0x100000000000000000000000000000003")]
        V0x100000000000000000000000000000003,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum DeployAccountTxn {
        DeployAccountTxnV1(DeployAccountTxnV1),
        DeployAccountTxnV3(DeployAccountTxnV3),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployAccountTxnReceipt {
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
        pub contract_address: Felt,
        pub r#type: DeployAccountTxnReceiptType,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployAccountTxnReceiptType {
        #[serde(rename = "DEPLOY_ACCOUNT")]
        DeployAccount,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployAccountTxnTrace {
        pub constructor_invocation: FunctionInvocation,
        pub execution_resources: ExecutionResources,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub fee_transfer_invocation: Option<FunctionInvocation>,
        pub r#type: DeployAccountTxnTraceType,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub state_diff: Option<StateDiff>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub validate_invocation: Option<FunctionInvocation>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployAccountTxnTraceType {
        #[serde(rename = "DEPLOY_ACCOUNT")]
        DeployAccount,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployAccountTxnV1 {
        pub class_hash: Felt,
        pub constructor_calldata: Vec<Felt>,
        pub contract_address_salt: Felt,
        pub max_fee: Felt,
        pub nonce: Felt,
        pub r#type: DeployAccountTxnV1Type,
        pub signature: Signature,
        pub version: DeployAccountTxnV1Version,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployAccountTxnV1Type {
        #[serde(rename = "DEPLOY_ACCOUNT")]
        DeployAccount,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployAccountTxnV1Version {
        #[serde(rename = "0x1")]
        V0x1,
        #[serde(rename = "0x100000000000000000000000000000001")]
        V0x100000000000000000000000000000001,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployAccountTxnV3 {
        pub class_hash: Felt,
        pub constructor_calldata: Vec<Felt>,
        pub contract_address_salt: Felt,
        pub fee_data_availability_mode: DaMode,
        pub nonce: Felt,
        pub nonce_data_availability_mode: DaMode,
        pub paymaster_data: Vec<Felt>,
        pub r#type: DeployAccountTxnV3Type,
        pub resource_bounds: ResourceBoundsMapping,
        pub signature: Signature,
        pub tip: U64,
        pub version: DeployAccountTxnV3Version,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployAccountTxnV3Type {
        #[serde(rename = "DEPLOY_ACCOUNT")]
        DeployAccount,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployAccountTxnV3Version {
        #[serde(rename = "0x3")]
        V0x3,
        #[serde(rename = "0x100000000000000000000000000000003")]
        V0x100000000000000000000000000000003,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployTxn {
        pub class_hash: Felt,
        pub constructor_calldata: Vec<Felt>,
        pub contract_address_salt: Felt,
        pub r#type: DeployTxnType,
        pub version: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployTxnReceipt {
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
        pub contract_address: Felt,
        pub r#type: DeployTxnReceiptType,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployTxnReceiptType {
        #[serde(rename = "DEPLOY")]
        Deploy,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum DeployTxnType {
        #[serde(rename = "DEPLOY")]
        Deploy,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeployedContractItem {
        pub address: Felt,
        pub class_hash: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeprecatedCairoEntryPoint {
        pub offset: NumAsHex,
        pub selector: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeprecatedContractClass {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub abi: Option<ContractAbi>,
        pub entry_points_by_type: DeprecatedContractClassEntryPointsByType,
        pub program: DeprecatedContractClassProgram,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeprecatedContractClassEntryPointsByType {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "CONSTRUCTOR")]
        pub constructor: Option<Vec<DeprecatedCairoEntryPoint>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "EXTERNAL")]
        pub external: Option<Vec<DeprecatedCairoEntryPoint>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "L1_HANDLER")]
        pub l1_handler: Option<Vec<DeprecatedCairoEntryPoint>>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "String")]
    pub struct DeprecatedContractClassProgram(String);

    mod deprecatedcontractclassprogram {
        use super::jsonrpc;
        use super::DeprecatedContractClassProgram;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static DEPRECATEDCONTRACTCLASSPROGRAM_REGEX: Lazy<Regex> = Lazy::new(
            || {
                Regex::new("^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{3}=|[A-Za-z0-9+/]{2}==)?$").expect("DeprecatedContractClassProgram: valid regex")
            },
        );

        impl DeprecatedContractClassProgram {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if DEPRECATEDCONTRACTCLASSPROGRAM_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("DeprecatedContractClassProgram value does not match regex: {value}"),
                })
                }
            }
        }

        impl TryFrom<String> for DeprecatedContractClassProgram {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for DeprecatedContractClassProgram {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EdgeNode {
        pub edge: EdgeNodeEdge,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EdgeNodeEdge {
        pub child: Felt,
        pub path: EdgeNodePath,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EdgeNodePath {
        pub len: i64,
        pub value: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EmittedEvent {
        #[serde(flatten)]
        pub event: Event,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub block_hash: Option<BlockHash>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub block_number: Option<BlockNumber>,
        pub transaction_hash: TxnHash,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum EntryPointType {
        #[serde(rename = "EXTERNAL")]
        External,
        #[serde(rename = "L1_HANDLER")]
        L1Handler,
        #[serde(rename = "CONSTRUCTOR")]
        Constructor,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "String")]
    pub struct EthAddress(String);

    mod ethaddress {
        use super::jsonrpc;
        use super::EthAddress;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static ETHADDRESS_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new("^0x[a-fA-F0-9]{40}$").expect("EthAddress: valid regex")
        });

        impl EthAddress {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if ETHADDRESS_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "EthAddress value does not match regex: {value}"
                        ),
                    })
                }
            }
        }

        impl TryFrom<String> for EthAddress {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for EthAddress {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Event {
        pub from_address: Address,
        #[serde(flatten)]
        pub event_content: EventContent,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EventAbiEntry {
        pub data: Vec<TypedParameter>,
        pub keys: Vec<TypedParameter>,
        pub name: String,
        pub r#type: EventAbiType,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum EventAbiType {
        #[serde(rename = "event")]
        Event,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EventContent {
        pub data: Vec<Felt>,
        pub keys: Vec<Felt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EventFilter {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub address: Option<Address>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub from_block: Option<BlockId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub keys: Option<Vec<Vec<Felt>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub to_block: Option<BlockId>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EventsChunk {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub continuation_token: Option<String>,
        pub events: Vec<EmittedEvent>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ExecutionResources {
        #[serde(flatten)]
        pub computation_resources: ComputationResources,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub data_availability: Option<ExecutionResourcesDataAvailability>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ExecutionResourcesDataAvailability {
        pub l1_data_gas: i64,
        pub l1_gas: i64,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FeeEstimate {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub data_gas_consumed: Option<Felt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub data_gas_price: Option<Felt>,
        pub gas_consumed: Felt,
        pub gas_price: Felt,
        pub overall_fee: Felt,
        pub unit: PriceUnit,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FeePayment {
        pub amount: Felt,
        pub unit: PriceUnit,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "String")]
    pub struct Felt(String);

    mod felt {
        use super::jsonrpc;
        use super::Felt;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static FELT_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new("^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,62})$")
                .expect("Felt: valid regex")
        });

        // The RPC spec regex is not respected anywhere these days,
        // thus such un-elegant workaround is necessary ¯\_(ツ)_/¯
        fn fix(value: &str) -> String {
            let unprefixed = value.strip_prefix("0x").unwrap_or(value);
            if unprefixed.is_empty() {
                // '0x'
                "0x0".to_owned()
            } else if unprefixed == "0" {
                value.to_owned()
            } else if unprefixed.starts_with("0") {
                // '0x0...'
                let unzeroed = unprefixed.trim_start_matches('0');
                format!("0x{unzeroed}")
            } else {
                value.to_owned()
            }
        }

        impl Felt {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                let value = &fix(value);
                if FELT_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "Felt value does not match regex: {value}"
                        ),
                    })
                }
            }
        }

        impl TryFrom<String> for Felt {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for Felt {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FunctionAbiEntry {
        pub inputs: Vec<TypedParameter>,
        pub name: String,
        pub outputs: Vec<TypedParameter>,
        pub r#type: FunctionAbiType,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "stateMutability")]
        pub statemutability: Option<FunctionStateMutability>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum FunctionAbiType {
        #[serde(rename = "function")]
        Function,
        #[serde(rename = "l1_handler")]
        L1Handler,
        #[serde(rename = "constructor")]
        Constructor,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FunctionCall {
        pub calldata: Vec<Felt>,
        pub contract_address: Address,
        pub entry_point_selector: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FunctionInvocation {
        #[serde(flatten)]
        pub function_call: FunctionCall,
        pub call_type: CallType,
        pub caller_address: Felt,
        pub calls: Vec<NestedCall>,
        pub class_hash: Felt,
        pub entry_point_type: EntryPointType,
        pub events: Vec<OrderedEvent>,
        pub execution_resources: ComputationResources,
        pub messages: Vec<OrderedMessage>,
        pub result: Vec<Felt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum FunctionStateMutability {
        #[serde(rename = "view")]
        View,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum InvokeTxn {
        InvokeTxnV0(InvokeTxnV0),
        InvokeTxnV1(InvokeTxnV1),
        InvokeTxnV3(InvokeTxnV3),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InvokeTxnReceipt {
        pub r#type: InvokeTxnReceiptType,
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnReceiptType {
        #[serde(rename = "INVOKE")]
        Invoke,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InvokeTxnTrace {
        pub execute_invocation: InvokeTxnTraceExecuteInvocation,
        pub execution_resources: ExecutionResources,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub fee_transfer_invocation: Option<FunctionInvocation>,
        pub r#type: InvokeTxnTraceType,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub state_diff: Option<StateDiff>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub validate_invocation: Option<FunctionInvocation>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum InvokeTxnTraceExecuteInvocation {
        FunctionInvocation(FunctionInvocation),
        RevertReason { revert_reason: String },
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnTraceType {
        #[serde(rename = "INVOKE")]
        Invoke,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InvokeTxnV0 {
        pub calldata: Vec<Felt>,
        pub contract_address: Address,
        pub entry_point_selector: Felt,
        pub max_fee: Felt,
        pub r#type: InvokeTxnV0Type,
        pub signature: Signature,
        pub version: InvokeTxnV0Version,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnV0Type {
        #[serde(rename = "INVOKE")]
        Invoke,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnV0Version {
        #[serde(rename = "0x0")]
        V0x0,
        #[serde(rename = "0x100000000000000000000000000000000")]
        V0x100000000000000000000000000000000,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InvokeTxnV1 {
        pub calldata: Vec<Felt>,
        pub max_fee: Felt,
        pub nonce: Felt,
        pub r#type: InvokeTxnV1Type,
        pub sender_address: Address,
        pub signature: Signature,
        pub version: InvokeTxnV1Version,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnV1Type {
        #[serde(rename = "INVOKE")]
        Invoke,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnV1Version {
        #[serde(rename = "0x1")]
        V0x1,
        #[serde(rename = "0x100000000000000000000000000000001")]
        V0x100000000000000000000000000000001,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InvokeTxnV3 {
        pub account_deployment_data: Vec<Felt>,
        pub calldata: Vec<Felt>,
        pub fee_data_availability_mode: DaMode,
        pub nonce: Felt,
        pub nonce_data_availability_mode: DaMode,
        pub paymaster_data: Vec<Felt>,
        pub r#type: InvokeTxnV3Type,
        pub resource_bounds: ResourceBoundsMapping,
        pub sender_address: Address,
        pub signature: Signature,
        pub tip: U64,
        pub version: InvokeTxnV3Version,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnV3Type {
        #[serde(rename = "INVOKE")]
        Invoke,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InvokeTxnV3Version {
        #[serde(rename = "0x3")]
        V0x3,
        #[serde(rename = "0x100000000000000000000000000000003")]
        V0x100000000000000000000000000000003,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1HandlerTxn {
        pub nonce: NumAsHex,
        pub r#type: L1HandlerTxnType,
        pub version: L1HandlerTxnVersion,
        #[serde(flatten)]
        pub function_call: FunctionCall,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1HandlerTxnReceipt {
        pub message_hash: NumAsHex,
        pub r#type: L1HandlerTxnReceiptType,
        #[serde(flatten)]
        pub common_receipt_properties: CommonReceiptProperties,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum L1HandlerTxnReceiptType {
        #[serde(rename = "L1_HANDLER")]
        L1Handler,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct L1HandlerTxnTrace {
        pub execution_resources: ExecutionResources,
        pub function_invocation: FunctionInvocation,
        pub r#type: L1HandlerTxnTraceType,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub state_diff: Option<StateDiff>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum L1HandlerTxnTraceType {
        #[serde(rename = "L1_HANDLER")]
        L1Handler,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum L1HandlerTxnType {
        #[serde(rename = "L1_HANDLER")]
        L1Handler,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum L1HandlerTxnVersion {
        #[serde(rename = "0x0")]
        V0x0,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MsgFromL1 {
        pub entry_point_selector: Felt,
        pub from_address: EthAddress,
        pub payload: Vec<Felt>,
        pub to_address: Address,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MsgToL1 {
        pub from_address: Felt,
        pub payload: Vec<Felt>,
        pub to_address: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NestedCall(pub FunctionInvocation);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NewClasses {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub class_hash: Option<Felt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub compiled_class_hash: Option<Felt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum Node {
        BinaryNode(BinaryNode),
        EdgeNode(EdgeNode),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct NonceUpdate {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub contract_address: Option<Address>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub nonce: Option<Felt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "String")]
    pub struct NumAsHex(String);

    mod numashex {
        use super::jsonrpc;
        use super::NumAsHex;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static NUMASHEX_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new("^0x[a-fA-F0-9]+$").expect("NumAsHex: valid regex")
        });

        impl NumAsHex {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if NUMASHEX_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "NumAsHex value does not match regex: {value}"
                        ),
                    })
                }
            }
        }

        impl TryFrom<String> for NumAsHex {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for NumAsHex {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct OrderedEvent {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub order: Option<i64>,
        #[serde(flatten)]
        pub event: Event,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct OrderedMessage {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub order: Option<i64>,
        #[serde(flatten)]
        pub msg_to_l1: MsgToL1,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingBlockHeader {
        pub l1_da_mode: PendingBlockHeaderL1DaMode,
        pub l1_data_gas_price: ResourcePrice,
        pub l1_gas_price: ResourcePrice,
        pub parent_hash: BlockHash,
        pub sequencer_address: Felt,
        pub starknet_version: String,
        pub timestamp: PendingBlockHeaderTimestamp,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum PendingBlockHeaderL1DaMode {
        #[serde(rename = "BLOB")]
        Blob,
        #[serde(rename = "CALLDATA")]
        Calldata,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "i64")]
    pub struct PendingBlockHeaderTimestamp(i64);

    mod pendingblockheadertimestamp {
        use super::jsonrpc;
        use super::PendingBlockHeaderTimestamp;

        static MIN: i64 = 0;
        static MAX: i64 = 9223372036854775807;

        impl PendingBlockHeaderTimestamp {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("PendingBlockHeaderTimestamp value {value} must be > {MIN}"),
                });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("PendingBlockHeaderTimestamp value {value} must be < {MAX}"),
                });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for PendingBlockHeaderTimestamp {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for PendingBlockHeaderTimestamp {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingBlockWithReceipts {
        #[serde(flatten)]
        pub block_body_with_receipts: BlockBodyWithReceipts,
        #[serde(flatten)]
        pub pending_block_header: PendingBlockHeader,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingBlockWithTxHashes {
        #[serde(flatten)]
        pub block_body_with_tx_hashes: BlockBodyWithTxHashes,
        #[serde(flatten)]
        pub pending_block_header: PendingBlockHeader,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingBlockWithTxs {
        #[serde(flatten)]
        pub block_body_with_txs: BlockBodyWithTxs,
        #[serde(flatten)]
        pub pending_block_header: PendingBlockHeader,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PendingStateUpdate {
        pub old_root: Felt,
        pub state_diff: StateDiff,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum PriceUnit {
        #[serde(rename = "WEI")]
        Wei,
        #[serde(rename = "FRI")]
        Fri,
    }

    type Proof = Vec<Node>;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ReplacedClass {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub class_hash: Option<Felt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub contract_address: Option<Address>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ResourceBounds {
        pub max_amount: U64,
        pub max_price_per_unit: U128,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ResourceBoundsMapping {
        pub l1_gas: ResourceBounds,
        pub l2_gas: ResourceBounds,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ResourcePrice {
        pub price_in_fri: Felt,
        pub price_in_wei: Felt,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum ResultCommonReceiptProperties {
        SuccessfulCommonReceiptProperties(SuccessfulCommonReceiptProperties),
        RevertedCommonReceiptProperties(RevertedCommonReceiptProperties),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ResultPageRequest {
        pub chunk_size: ResultPageRequestChunkSize,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub continuation_token: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "i64")]
    pub struct ResultPageRequestChunkSize(i64);

    mod resultpagerequestchunksize {
        use super::jsonrpc;
        use super::ResultPageRequestChunkSize;

        static MIN: i64 = 1;
        static MAX: i64 = 9223372036854775807;

        impl ResultPageRequestChunkSize {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("ResultPageRequestChunkSize value {value} must be > {MIN}"),
                });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("ResultPageRequestChunkSize value {value} must be < {MAX}"),
                });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for ResultPageRequestChunkSize {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for ResultPageRequestChunkSize {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RevertedCommonReceiptProperties {
        pub execution_status: RevertedCommonReceiptPropertiesExecutionStatus,
        pub revert_reason: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum RevertedCommonReceiptPropertiesExecutionStatus {
        #[serde(rename = "REVERTED")]
        Reverted,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SierraEntryPoint {
        pub function_idx: i64,
        pub selector: Felt,
    }

    type Signature = Vec<Felt>;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum SimulationFlag {
        #[serde(rename = "SKIP_VALIDATE")]
        SkipValidate,
        #[serde(rename = "SKIP_FEE_CHARGE")]
        SkipFeeCharge,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum SimulationFlagForEstimateFee {
        #[serde(rename = "SKIP_VALIDATE")]
        SkipValidate,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StateDiff {
        pub declared_classes: Vec<NewClasses>,
        pub deployed_contracts: Vec<DeployedContractItem>,
        pub deprecated_declared_classes: Vec<Felt>,
        pub nonces: Vec<NonceUpdate>,
        pub replaced_classes: Vec<ReplacedClass>,
        pub storage_diffs: Vec<ContractStorageDiffItem>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StateUpdate {
        pub block_hash: BlockHash,
        pub new_root: Felt,
        pub old_root: Felt,
        pub state_diff: StateDiff,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StorageDiffItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub key: Option<Felt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub value: Option<Felt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "String")]
    pub struct StorageKey(String);

    mod storagekey {
        use super::jsonrpc;
        use super::StorageKey;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static STORAGEKEY_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new("^0x(0|[0-7]{1}[a-fA-F0-9]{0,62}$)")
                .expect("StorageKey: valid regex")
        });

        impl StorageKey {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if STORAGEKEY_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "StorageKey value does not match regex: {value}"
                        ),
                    })
                }
            }
        }

        impl TryFrom<String> for StorageKey {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for StorageKey {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StructAbiEntry {
        pub members: Vec<StructMember>,
        pub name: String,
        pub r#type: StructAbiType,
        pub size: StructAbiEntrySize,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "i64")]
    pub struct StructAbiEntrySize(i64);

    mod structabientrysize {
        use super::jsonrpc;
        use super::StructAbiEntrySize;

        static MIN: i64 = 1;
        static MAX: i64 = 9223372036854775807;

        impl StructAbiEntrySize {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "StructAbiEntrySize value {value} must be > {MIN}"
                        ),
                    });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "StructAbiEntrySize value {value} must be < {MAX}"
                        ),
                    });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for StructAbiEntrySize {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for StructAbiEntrySize {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum StructAbiType {
        #[serde(rename = "struct")]
        Struct,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StructMember {
        #[serde(flatten)]
        pub typed_parameter: TypedParameter,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub offset: Option<i64>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SuccessfulCommonReceiptProperties {
        pub execution_status: SuccessfulCommonReceiptPropertiesExecutionStatus,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum SuccessfulCommonReceiptPropertiesExecutionStatus {
        #[serde(rename = "SUCCEEDED")]
        Succeeded,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SyncStatus {
        pub current_block_hash: BlockHash,
        pub current_block_num: BlockNumber,
        pub highest_block_hash: BlockHash,
        pub highest_block_num: BlockNumber,
        pub starting_block_hash: BlockHash,
        pub starting_block_num: BlockNumber,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TransactionAndReceipt {
        pub receipt: TxnReceipt,
        pub transaction: Txn,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum TransactionTrace {
        InvokeTxnTrace(InvokeTxnTrace),
        DeclareTxnTrace(DeclareTxnTrace),
        DeployAccountTxnTrace(DeployAccountTxnTrace),
        L1HandlerTxnTrace(L1HandlerTxnTrace),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TransactionsInBlock {
        #[serde(flatten)]
        pub txn: Txn,
        pub transaction_hash: TxnHash,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum TxGatewayStatus {
        #[serde(rename = "NOT_RECEIVED")]
        NotReceived,
        #[serde(rename = "RECEIVED")]
        Received,
        #[serde(rename = "PENDING")]
        Pending,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "ACCEPTED_ON_L1")]
        AcceptedOnL1,
        #[serde(rename = "ACCEPTED_ON_L2")]
        AcceptedOnL2,
        #[serde(rename = "REVERTED")]
        Reverted,
        #[serde(rename = "ABORTED")]
        Aborted,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum Txn {
        InvokeTxn(InvokeTxn),
        L1HandlerTxn(L1HandlerTxn),
        DeclareTxn(DeclareTxn),
        DeployTxn(DeployTxn),
        DeployAccountTxn(DeployAccountTxn),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum TxnExecutionStatus {
        #[serde(rename = "SUCCEEDED")]
        Succeeded,
        #[serde(rename = "REVERTED")]
        Reverted,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum TxnFinalityStatus {
        #[serde(rename = "ACCEPTED_ON_L2")]
        AcceptedOnL2,
        #[serde(rename = "ACCEPTED_ON_L1")]
        AcceptedOnL1,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TxnHash(pub Felt);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum TxnReceipt {
        InvokeTxnReceipt(InvokeTxnReceipt),
        L1HandlerTxnReceipt(L1HandlerTxnReceipt),
        DeclareTxnReceipt(DeclareTxnReceipt),
        DeployTxnReceipt(DeployTxnReceipt),
        DeployAccountTxnReceipt(DeployAccountTxnReceipt),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TxnReceiptWithBlockInfo {
        #[serde(flatten)]
        pub txn_receipt: TxnReceipt,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub block_hash: Option<BlockHash>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub block_number: Option<BlockNumber>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum TxnStatus {
        #[serde(rename = "RECEIVED")]
        Received,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "ACCEPTED_ON_L2")]
        AcceptedOnL2,
        #[serde(rename = "ACCEPTED_ON_L1")]
        AcceptedOnL1,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum TxnType {
        #[serde(rename = "DECLARE")]
        Declare,
        #[serde(rename = "DEPLOY")]
        Deploy,
        #[serde(rename = "DEPLOY_ACCOUNT")]
        DeployAccount,
        #[serde(rename = "INVOKE")]
        Invoke,
        #[serde(rename = "L1_HANDLER")]
        L1Handler,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TypedParameter {
        pub name: String,
        pub r#type: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "String")]
    pub struct U128(String);

    mod u128 {
        use super::jsonrpc;
        use super::U128;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static U128_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new("^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,31})$")
                .expect("U128: valid regex")
        });

        impl U128 {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if U128_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "U128 value does not match regex: {value}"
                        ),
                    })
                }
            }
        }

        impl TryFrom<String> for U128 {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for U128 {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "String")]
    pub struct U64(String);

    mod u64 {
        use super::jsonrpc;
        use super::U64;
        use once_cell::sync::Lazy;
        use regex::Regex;

        static U64_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new("^0x(0|[a-fA-F1-9]{1}[a-fA-F0-9]{0,15})$")
                .expect("U64: valid regex")
        });

        impl U64 {
            pub fn try_new(value: &str) -> Result<Self, jsonrpc::Error> {
                if U64_REGEX.is_match(value) {
                    Ok(Self(value.to_string()))
                } else {
                    Err(jsonrpc::Error {
                        code: 1001,
                        message: format!(
                            "U64 value does not match regex: {value}"
                        ),
                    })
                }
            }
        }

        impl TryFrom<String> for U64 {
            type Error = String;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(&value).map_err(|e| e.message)
            }
        }

        impl AsRef<String> for U64 {
            fn as_ref(&self) -> &String {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetBlockWithTxHashesResult {
        BlockWithTxHashes(BlockWithTxHashes),
        PendingBlockWithTxHashes(PendingBlockWithTxHashes),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetBlockWithTxsResult {
        BlockWithTxs(BlockWithTxs),
        PendingBlockWithTxs(PendingBlockWithTxs),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetBlockWithReceiptsResult {
        BlockWithReceipts(BlockWithReceipts),
        PendingBlockWithReceipts(PendingBlockWithReceipts),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetStateUpdateResult {
        StateUpdate(StateUpdate),
        PendingStateUpdate(PendingStateUpdate),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetTransactionStatusResult {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub execution_status: Option<TxnExecutionStatus>,
        pub finality_status: TxnStatus,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetTransactionByHashResult {
        #[serde(flatten)]
        pub txn: Txn,
        pub transaction_hash: TxnHash,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetTransactionByBlockIdAndIndexResult {
        #[serde(flatten)]
        pub txn: Txn,
        pub transaction_hash: TxnHash,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "i64")]
    pub struct GetTransactionByBlockIdAndIndexIndex(i64);

    mod gettransactionbyblockidandindexindex {
        use super::jsonrpc;
        use super::GetTransactionByBlockIdAndIndexIndex;

        static MIN: i64 = 0;
        static MAX: i64 = 9223372036854775807;

        impl GetTransactionByBlockIdAndIndexIndex {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("GetTransactionByBlockIdAndIndexIndex value {value} must be > {MIN}"),
                });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("GetTransactionByBlockIdAndIndexIndex value {value} must be < {MAX}"),
                });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for GetTransactionByBlockIdAndIndexIndex {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for GetTransactionByBlockIdAndIndexIndex {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetClassResult {
        DeprecatedContractClass(DeprecatedContractClass),
        ContractClass(ContractClass),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum GetClassAtResult {
        DeprecatedContractClass(DeprecatedContractClass),
        ContractClass(ContractClass),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(try_from = "i64")]
    pub struct GetBlockTransactionCountResult(i64);

    mod getblocktransactioncountresult {
        use super::jsonrpc;
        use super::GetBlockTransactionCountResult;

        static MIN: i64 = 0;
        static MAX: i64 = 9223372036854775807;

        impl GetBlockTransactionCountResult {
            pub fn try_new(value: i64) -> Result<Self, jsonrpc::Error> {
                if value < MIN {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("GetBlockTransactionCountResult value {value} must be > {MIN}"),
                });
                }
                if value > MAX {
                    return Err(jsonrpc::Error {
                    code: 1001,
                    message: format!("GetBlockTransactionCountResult value {value} must be < {MAX}"),
                });
                }
                Ok(Self(value))
            }
        }

        impl TryFrom<i64> for GetBlockTransactionCountResult {
            type Error = String;
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                Self::try_new(value).map_err(|e| e.message)
            }
        }

        impl AsRef<i64> for GetBlockTransactionCountResult {
            fn as_ref(&self) -> &i64 {
                &self.0
            }
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockHashAndNumberResult {
        pub block_hash: BlockHash,
        pub block_number: BlockNumber,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum SyncingResult {
        False(bool),
        SyncStatus(SyncStatus),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetEventsFilter {
        #[serde(flatten)]
        pub event_filter: EventFilter,
        #[serde(flatten)]
        pub result_page_request: ResultPageRequest,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddInvokeTransactionResult {
        pub transaction_hash: TxnHash,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddDeclareTransactionResult {
        pub class_hash: Felt,
        pub transaction_hash: TxnHash,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddDeployAccountTransactionResult {
        pub contract_address: Felt,
        pub transaction_hash: TxnHash,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SimulatedTransaction {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub fee_estimation: Option<FeeEstimate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub transaction_trace: Option<TransactionTrace>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BlockTransactionTrace {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub trace_root: Option<TransactionTrace>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub transaction_hash: Option<Felt>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ContractData {
        pub class_hash: Felt,
        pub contract_state_hash_version: Felt,
        pub nonce: Felt,
        pub root: Felt,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub storage_proofs: Option<Vec<Proof>>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetProofResult {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub class_commitment: Option<Felt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub contract_data: Option<ContractData>,
        pub contract_proof: Proof,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub state_commitment: Option<Felt>,
    }

    pub mod error {
        pub const BLOCK_NOT_FOUND: Error = Error(24, "Block not found");
        pub const CLASS_ALREADY_DECLARED: Error =
            Error(51, "Class already declared");
        pub const CLASS_HASH_NOT_FOUND: Error =
            Error(28, "Class hash not found");
        pub const COMPILATION_FAILED: Error = Error(56, "Compilation failed");
        pub const COMPILED_CLASS_HASH_MISMATCH: Error = Error(60, "the compiled class hash did not match the one supplied in the transaction");
        pub const CONTRACT_CLASS_SIZE_IS_TOO_LARGE: Error =
            Error(57, "Contract class size it too large");
        pub const CONTRACT_ERROR: Error = Error(40, "Contract error");
        pub const CONTRACT_NOT_FOUND: Error = Error(20, "Contract not found");
        pub const DUPLICATE_TX: Error = Error(
            59,
            "A transaction with the same hash already exists in the mempool",
        );
        pub const FAILED_TO_RECEIVE_TXN: Error =
            Error(1, "Failed to write transaction");
        pub const INSUFFICIENT_ACCOUNT_BALANCE: Error = Error(
            54,
            "Account balance is smaller than the transaction's max_fee",
        );
        pub const INSUFFICIENT_MAX_FEE: Error = Error(53, "Max fee is smaller than the minimal transaction cost (validation plus fee transfer)");
        pub const INVALID_CONTINUATION_TOKEN: Error =
            Error(33, "The supplied continuation token is invalid or unknown");
        pub const INVALID_TRANSACTION_NONCE: Error =
            Error(52, "Invalid transaction nonce");
        pub const INVALID_TXN_INDEX: Error =
            Error(27, "Invalid transaction index in a block");
        pub const NON_ACCOUNT: Error =
            Error(58, "Sender address in not an account contract");
        pub const NO_BLOCKS: Error = Error(32, "There are no blocks");
        pub const NO_TRACE_AVAILABLE: Error =
            Error(10, "No trace available for transaction");
        pub const PAGE_SIZE_TOO_BIG: Error =
            Error(31, "Requested page size is too big");
        pub const PROOF_LIMIT_EXCEEDED: Error =
            Error(10000, "Too many storage keys requested");
        pub const TOO_MANY_KEYS_IN_FILTER: Error =
            Error(34, "Too many keys provided in a filter");
        pub const TRANSACTION_EXECUTION_ERROR: Error =
            Error(41, "Transaction execution error");
        pub const TXN_HASH_NOT_FOUND: Error =
            Error(29, "Transaction hash not found");
        pub const UNEXPECTED_ERROR: Error =
            Error(63, "An unexpected error occurred");
        pub const UNSUPPORTED_CONTRACT_CLASS_VERSION: Error =
            Error(62, "the contract class version is not supported");
        pub const UNSUPPORTED_TX_VERSION: Error =
            Error(61, "the transaction version is not supported");
        pub const VALIDATION_FAILURE: Error =
            Error(55, "Account validation failed");

        pub struct Error(i64, &'static str);

        impl From<Error> for iamgroot::jsonrpc::Error {
            fn from(Error(code, message): Error) -> Self {
                Self { code, message: message.to_string() }
            }
        }
    }

    #[allow(non_snake_case)]
    #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    pub trait Rpc {
        /// Returns merkle proofs of a contract's storage state
        async fn getProof(
            &self,
            block_id: BlockId,
            contract_address: Address,
            keys: Vec<StorageKey>,
        ) -> std::result::Result<GetProofResult, jsonrpc::Error>;

        /// Returns the status of a transaction
        async fn getTxStatus(
            &self,
            transaction_hash: TxnHash,
        ) -> std::result::Result<TxGatewayStatus, jsonrpc::Error>;

        /// The version of the pathfinder node hosting this API.
        async fn version(&self) -> std::result::Result<String, jsonrpc::Error>;

        /// Submit a new class declaration transaction
        async fn addDeclareTransaction(
            &self,
            declare_transaction: BroadcastedDeclareTxn,
        ) -> std::result::Result<AddDeclareTransactionResult, jsonrpc::Error>;

        /// Submit a new deploy account transaction
        async fn addDeployAccountTransaction(
            &self,
            deploy_account_transaction: BroadcastedDeployAccountTxn,
        ) -> std::result::Result<
            AddDeployAccountTransactionResult,
            jsonrpc::Error,
        >;

        /// Submit a new transaction to be added to the chain
        async fn addInvokeTransaction(
            &self,
            invoke_transaction: BroadcastedInvokeTxn,
        ) -> std::result::Result<AddInvokeTransactionResult, jsonrpc::Error>;

        /// Get the most recent accepted block hash and number
        async fn blockHashAndNumber(
            &self,
        ) -> std::result::Result<BlockHashAndNumberResult, jsonrpc::Error>;

        /// Get the most recent accepted block number
        async fn blockNumber(
            &self,
        ) -> std::result::Result<BlockNumber, jsonrpc::Error>;

        /// call a starknet function without creating a StarkNet transaction
        async fn call(
            &self,
            request: FunctionCall,
            block_id: BlockId,
        ) -> std::result::Result<Vec<Felt>, jsonrpc::Error>;

        /// Return the currently configured StarkNet chain id
        async fn chainId(&self)
            -> std::result::Result<ChainId, jsonrpc::Error>;

        /// estimate the fee for of StarkNet transactions
        async fn estimateFee(
            &self,
            request: Vec<BroadcastedTxn>,
            simulation_flags: Vec<SimulationFlagForEstimateFee>,
            block_id: BlockId,
        ) -> std::result::Result<Vec<FeeEstimate>, jsonrpc::Error>;

        /// estimate the L2 fee of a message sent on L1
        async fn estimateMessageFee(
            &self,
            message: MsgFromL1,
            block_id: BlockId,
        ) -> std::result::Result<FeeEstimate, jsonrpc::Error>;

        /// Get the number of transactions in a block given a block id
        async fn getBlockTransactionCount(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetBlockTransactionCountResult, jsonrpc::Error>;

        /// Get block information with full transactions and receipts given the block id
        async fn getBlockWithReceipts(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetBlockWithReceiptsResult, jsonrpc::Error>;

        /// Get block information with transaction hashes given the block id
        async fn getBlockWithTxHashes(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetBlockWithTxHashesResult, jsonrpc::Error>;

        /// Get block information with full transactions given the block id
        async fn getBlockWithTxs(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetBlockWithTxsResult, jsonrpc::Error>;

        /// Get the contract class definition in the given block associated with the given hash
        async fn getClass(
            &self,
            block_id: BlockId,
            class_hash: Felt,
        ) -> std::result::Result<GetClassResult, jsonrpc::Error>;

        /// Get the contract class definition in the given block at the given address
        async fn getClassAt(
            &self,
            block_id: BlockId,
            contract_address: Address,
        ) -> std::result::Result<GetClassAtResult, jsonrpc::Error>;

        /// Get the contract class hash in the given block for the contract deployed at the given address
        async fn getClassHashAt(
            &self,
            block_id: BlockId,
            contract_address: Address,
        ) -> std::result::Result<Felt, jsonrpc::Error>;

        /// Returns all events matching the given filter
        async fn getEvents(
            &self,
            filter: GetEventsFilter,
        ) -> std::result::Result<EventsChunk, jsonrpc::Error>;

        /// Get the nonce associated with the given address in the given block
        async fn getNonce(
            &self,
            block_id: BlockId,
            contract_address: Address,
        ) -> std::result::Result<Felt, jsonrpc::Error>;

        /// Get the information about the result of executing the requested block
        async fn getStateUpdate(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<GetStateUpdateResult, jsonrpc::Error>;

        /// Get the value of the storage at the given address and key
        async fn getStorageAt(
            &self,
            contract_address: Address,
            key: StorageKey,
            block_id: BlockId,
        ) -> std::result::Result<Felt, jsonrpc::Error>;

        /// Get the details of a transaction by a given block id and index
        async fn getTransactionByBlockIdAndIndex(
            &self,
            block_id: BlockId,
            index: GetTransactionByBlockIdAndIndexIndex,
        ) -> std::result::Result<
            GetTransactionByBlockIdAndIndexResult,
            jsonrpc::Error,
        >;

        /// Get the details and status of a submitted transaction
        async fn getTransactionByHash(
            &self,
            transaction_hash: TxnHash,
        ) -> std::result::Result<GetTransactionByHashResult, jsonrpc::Error>;

        /// Get the transaction receipt by the transaction hash
        async fn getTransactionReceipt(
            &self,
            transaction_hash: TxnHash,
        ) -> std::result::Result<TxnReceiptWithBlockInfo, jsonrpc::Error>;

        /// Gets the transaction status (possibly reflecting that the tx is still in the mempool, or dropped from it)
        async fn getTransactionStatus(
            &self,
            transaction_hash: TxnHash,
        ) -> std::result::Result<GetTransactionStatusResult, jsonrpc::Error>;

        /// Simulate a given sequence of transactions on the requested state, and generate the execution traces. Note that some of the transactions may revert, in which case no error is thrown, but revert details can be seen on the returned trace object. . Note that some of the transactions may revert, this will be reflected by the revert_error property in the trace. Other types of failures (e.g. unexpected error or failure in the validation phase) will result in TRANSACTION_EXECUTION_ERROR.
        async fn simulateTransactions(
            &self,
            block_id: BlockId,
            transactions: Vec<BroadcastedTxn>,
            simulation_flags: Vec<SimulationFlag>,
        ) -> std::result::Result<Vec<SimulatedTransaction>, jsonrpc::Error>;

        /// Returns the version of the Starknet JSON-RPC specification being used
        async fn specVersion(
            &self,
        ) -> std::result::Result<String, jsonrpc::Error>;

        /// Returns an object about the sync status, or false if the node is not synching
        async fn syncing(
            &self,
        ) -> std::result::Result<SyncingResult, jsonrpc::Error>;

        /// Retrieve traces for all transactions in the given block
        async fn traceBlockTransactions(
            &self,
            block_id: BlockId,
        ) -> std::result::Result<Vec<BlockTransactionTrace>, jsonrpc::Error>;

        /// For a given executed transaction, return the trace of its execution, including internal calls
        async fn traceTransaction(
            &self,
            transaction_hash: TxnHash,
        ) -> std::result::Result<TransactionTrace, jsonrpc::Error>;
    }

    async fn handle_getProof<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Address, Vec<StorageKey>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            contract_address: Address,
            keys: Vec<StorageKey>,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id, contract_address, keys) =
                            args_by_pos;
                        ArgByName { block_id, contract_address, keys }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { block_id, contract_address, keys } = args;

        match rpc.getProof(block_id, contract_address, keys).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_getTxStatus<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TxnHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: TxnHash,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(transaction_hash) = args_by_pos;
                        ArgByName { transaction_hash }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { transaction_hash } = args;

        match rpc.getTxStatus(transaction_hash).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_version<RPC: Rpc>(
        rpc: &RPC,
        _params: &Value,
    ) -> jsonrpc::Response {
        match rpc.version().await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_addDeclareTransaction<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BroadcastedDeclareTxn);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            declare_transaction: BroadcastedDeclareTxn,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(declare_transaction) = args_by_pos;
                        ArgByName { declare_transaction }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { declare_transaction } = args;

        match rpc.addDeclareTransaction(declare_transaction).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_addDeployAccountTransaction<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BroadcastedDeployAccountTxn);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            deploy_account_transaction: BroadcastedDeployAccountTxn,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(deploy_account_transaction) = args_by_pos;
                        ArgByName { deploy_account_transaction }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { deploy_account_transaction } = args;

        match rpc.addDeployAccountTransaction(deploy_account_transaction).await
        {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_addInvokeTransaction<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BroadcastedInvokeTxn);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            invoke_transaction: BroadcastedInvokeTxn,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(invoke_transaction) = args_by_pos;
                        ArgByName { invoke_transaction }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { invoke_transaction } = args;

        match rpc.addInvokeTransaction(invoke_transaction).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_blockHashAndNumber<RPC: Rpc>(
        rpc: &RPC,
        _params: &Value,
    ) -> jsonrpc::Response {
        match rpc.blockHashAndNumber().await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_blockNumber<RPC: Rpc>(
        rpc: &RPC,
        _params: &Value,
    ) -> jsonrpc::Response {
        match rpc.blockNumber().await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_call<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(FunctionCall, BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            request: FunctionCall,
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(request, block_id) = args_by_pos;
                        ArgByName { request, block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { request, block_id } = args;

        match rpc.call(request, block_id).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_chainId<RPC: Rpc>(
        rpc: &RPC,
        _params: &Value,
    ) -> jsonrpc::Response {
        match rpc.chainId().await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_estimateFee<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(
            Vec<BroadcastedTxn>,
            Vec<SimulationFlagForEstimateFee>,
            BlockId,
        );

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            request: Vec<BroadcastedTxn>,
            simulation_flags: Vec<SimulationFlagForEstimateFee>,
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(request, simulation_flags, block_id) =
                            args_by_pos;
                        ArgByName { request, simulation_flags, block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { request, simulation_flags, block_id } = args;

        match rpc.estimateFee(request, simulation_flags, block_id).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_estimateMessageFee<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(MsgFromL1, BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            message: MsgFromL1,
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(message, block_id) = args_by_pos;
                        ArgByName { message, block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { message, block_id } = args;

        match rpc.estimateMessageFee(message, block_id).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_getBlockTransactionCount<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id) = args_by_pos;
                        ArgByName { block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { block_id } = args;

        match rpc.getBlockTransactionCount(block_id).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_getBlockWithReceipts<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id) = args_by_pos;
                        ArgByName { block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { block_id } = args;

        match rpc.getBlockWithReceipts(block_id).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_getBlockWithTxHashes<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id) = args_by_pos;
                        ArgByName { block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { block_id } = args;

        match rpc.getBlockWithTxHashes(block_id).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_getBlockWithTxs<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id) = args_by_pos;
                        ArgByName { block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { block_id } = args;

        match rpc.getBlockWithTxs(block_id).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_getClass<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Felt);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            class_hash: Felt,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id, class_hash) = args_by_pos;
                        ArgByName { block_id, class_hash }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { block_id, class_hash } = args;

        match rpc.getClass(block_id, class_hash).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_getClassAt<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Address);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            contract_address: Address,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id, contract_address) = args_by_pos;
                        ArgByName { block_id, contract_address }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { block_id, contract_address } = args;

        match rpc.getClassAt(block_id, contract_address).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_getClassHashAt<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Address);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            contract_address: Address,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id, contract_address) = args_by_pos;
                        ArgByName { block_id, contract_address }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { block_id, contract_address } = args;

        match rpc.getClassHashAt(block_id, contract_address).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_getEvents<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(GetEventsFilter);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            filter: GetEventsFilter,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(filter) = args_by_pos;
                        ArgByName { filter }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { filter } = args;

        match rpc.getEvents(filter).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_getNonce<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Address);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            contract_address: Address,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id, contract_address) = args_by_pos;
                        ArgByName { block_id, contract_address }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { block_id, contract_address } = args;

        match rpc.getNonce(block_id, contract_address).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_getStateUpdate<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id) = args_by_pos;
                        ArgByName { block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { block_id } = args;

        match rpc.getStateUpdate(block_id).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_getStorageAt<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(Address, StorageKey, BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            contract_address: Address,
            key: StorageKey,
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(contract_address, key, block_id) =
                            args_by_pos;
                        ArgByName { contract_address, key, block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { contract_address, key, block_id } = args;

        match rpc.getStorageAt(contract_address, key, block_id).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_getTransactionByBlockIdAndIndex<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, GetTransactionByBlockIdAndIndexIndex);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            index: GetTransactionByBlockIdAndIndexIndex,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id, index) = args_by_pos;
                        ArgByName { block_id, index }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { block_id, index } = args;

        match rpc.getTransactionByBlockIdAndIndex(block_id, index).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_getTransactionByHash<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TxnHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: TxnHash,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(transaction_hash) = args_by_pos;
                        ArgByName { transaction_hash }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { transaction_hash } = args;

        match rpc.getTransactionByHash(transaction_hash).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_getTransactionReceipt<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TxnHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: TxnHash,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(transaction_hash) = args_by_pos;
                        ArgByName { transaction_hash }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { transaction_hash } = args;

        match rpc.getTransactionReceipt(transaction_hash).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_getTransactionStatus<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TxnHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: TxnHash,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(transaction_hash) = args_by_pos;
                        ArgByName { transaction_hash }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { transaction_hash } = args;

        match rpc.getTransactionStatus(transaction_hash).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_simulateTransactions<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId, Vec<BroadcastedTxn>, Vec<SimulationFlag>);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
            transactions: Vec<BroadcastedTxn>,
            simulation_flags: Vec<SimulationFlag>,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id, transactions, simulation_flags) =
                            args_by_pos;
                        ArgByName { block_id, transactions, simulation_flags }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { block_id, transactions, simulation_flags } = args;

        match rpc
            .simulateTransactions(block_id, transactions, simulation_flags)
            .await
        {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_specVersion<RPC: Rpc>(
        rpc: &RPC,
        _params: &Value,
    ) -> jsonrpc::Response {
        match rpc.specVersion().await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_syncing<RPC: Rpc>(
        rpc: &RPC,
        _params: &Value,
    ) -> jsonrpc::Response {
        match rpc.syncing().await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_traceBlockTransactions<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(BlockId);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            block_id: BlockId,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(block_id) = args_by_pos;
                        ArgByName { block_id }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { block_id } = args;

        match rpc.traceBlockTransactions(block_id).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    async fn handle_traceTransaction<RPC: Rpc>(
        rpc: &RPC,
        params: &Value,
    ) -> jsonrpc::Response {
        #[derive(Deserialize, Serialize)]
        struct ArgByPos(TxnHash);

        #[derive(Deserialize, Serialize)]
        struct ArgByName {
            transaction_hash: TxnHash,
        }

        let args =
            serde_json::from_value::<ArgByName>(params.clone()).or_else(|_| {
                serde_json::from_value::<ArgByPos>(params.clone()).map(
                    |args_by_pos| {
                        let ArgByPos(transaction_hash) = args_by_pos;
                        ArgByName { transaction_hash }
                    },
                )
            });

        let args: ArgByName = match args {
            Ok(args) => args,
            Err(error) => {
                tracing::debug!(?error, "failed to parse request params");
                return jsonrpc::Response::error(-32602, "Invalid params");
            }
        };

        let ArgByName { transaction_hash } = args;

        match rpc.traceTransaction(transaction_hash).await {
            Ok(ret) => match serde_json::to_value(ret) {
                Ok(ret) => jsonrpc::Response::result(ret),
                Err(error) => {
                    tracing::debug!(?error, "failed to parse response object");
                    jsonrpc::Response::error(-32603, "Internal error")
                }
            },
            Err(e) => jsonrpc::Response::error(e.code, &e.message),
        }
    }

    pub async fn handle<RPC: Rpc>(
        rpc: &RPC,
        req: &jsonrpc::Request,
    ) -> jsonrpc::Response {
        let params = &req.params.clone().unwrap_or_default();

        let response = match req.method.as_str() {
            "pathfinder_getProof" => handle_getProof(rpc, params).await,
            "pathfinder_getTxStatus" => handle_getTxStatus(rpc, params).await,
            "pathfinder_version" => handle_version(rpc, params).await,
            "starknet_addDeclareTransaction" => {
                handle_addDeclareTransaction(rpc, params).await
            }
            "starknet_addDeployAccountTransaction" => {
                handle_addDeployAccountTransaction(rpc, params).await
            }
            "starknet_addInvokeTransaction" => {
                handle_addInvokeTransaction(rpc, params).await
            }
            "starknet_blockHashAndNumber" => {
                handle_blockHashAndNumber(rpc, params).await
            }
            "starknet_blockNumber" => handle_blockNumber(rpc, params).await,
            "starknet_call" => handle_call(rpc, params).await,
            "starknet_chainId" => handle_chainId(rpc, params).await,
            "starknet_estimateFee" => handle_estimateFee(rpc, params).await,
            "starknet_estimateMessageFee" => {
                handle_estimateMessageFee(rpc, params).await
            }
            "starknet_getBlockTransactionCount" => {
                handle_getBlockTransactionCount(rpc, params).await
            }
            "starknet_getBlockWithReceipts" => {
                handle_getBlockWithReceipts(rpc, params).await
            }
            "starknet_getBlockWithTxHashes" => {
                handle_getBlockWithTxHashes(rpc, params).await
            }
            "starknet_getBlockWithTxs" => {
                handle_getBlockWithTxs(rpc, params).await
            }
            "starknet_getClass" => handle_getClass(rpc, params).await,
            "starknet_getClassAt" => handle_getClassAt(rpc, params).await,
            "starknet_getClassHashAt" => {
                handle_getClassHashAt(rpc, params).await
            }
            "starknet_getEvents" => handle_getEvents(rpc, params).await,
            "starknet_getNonce" => handle_getNonce(rpc, params).await,
            "starknet_getStateUpdate" => {
                handle_getStateUpdate(rpc, params).await
            }
            "starknet_getStorageAt" => handle_getStorageAt(rpc, params).await,
            "starknet_getTransactionByBlockIdAndIndex" => {
                handle_getTransactionByBlockIdAndIndex(rpc, params).await
            }
            "starknet_getTransactionByHash" => {
                handle_getTransactionByHash(rpc, params).await
            }
            "starknet_getTransactionReceipt" => {
                handle_getTransactionReceipt(rpc, params).await
            }
            "starknet_getTransactionStatus" => {
                handle_getTransactionStatus(rpc, params).await
            }
            "starknet_simulateTransactions" => {
                handle_simulateTransactions(rpc, params).await
            }
            "starknet_specVersion" => handle_specVersion(rpc, params).await,
            "starknet_syncing" => handle_syncing(rpc, params).await,
            "starknet_traceBlockTransactions" => {
                handle_traceBlockTransactions(rpc, params).await
            }
            "starknet_traceTransaction" => {
                handle_traceTransaction(rpc, params).await
            }
            _ => jsonrpc::Response::error(-32601, "Method not found"),
        };

        return if let Some(id) = req.id.as_ref() {
            response.with_id(id.clone())
        } else {
            response
        };
    }

    pub mod blocking {
        use super::*;
        pub trait Rpc {
            /// Returns merkle proofs of a contract's storage state
            fn getProof(
                &self,
                block_id: BlockId,
                contract_address: Address,
                keys: Vec<StorageKey>,
            ) -> std::result::Result<GetProofResult, jsonrpc::Error>;

            /// Returns the status of a transaction
            fn getTxStatus(
                &self,
                transaction_hash: TxnHash,
            ) -> std::result::Result<TxGatewayStatus, jsonrpc::Error>;

            /// The version of the pathfinder node hosting this API.
            fn version(&self) -> std::result::Result<String, jsonrpc::Error>;

            /// Submit a new class declaration transaction
            fn addDeclareTransaction(
                &self,
                declare_transaction: BroadcastedDeclareTxn,
            ) -> std::result::Result<AddDeclareTransactionResult, jsonrpc::Error>;

            /// Submit a new deploy account transaction
            fn addDeployAccountTransaction(
                &self,
                deploy_account_transaction: BroadcastedDeployAccountTxn,
            ) -> std::result::Result<
                AddDeployAccountTransactionResult,
                jsonrpc::Error,
            >;

            /// Submit a new transaction to be added to the chain
            fn addInvokeTransaction(
                &self,
                invoke_transaction: BroadcastedInvokeTxn,
            ) -> std::result::Result<AddInvokeTransactionResult, jsonrpc::Error>;

            /// Get the most recent accepted block hash and number
            fn blockHashAndNumber(
                &self,
            ) -> std::result::Result<BlockHashAndNumberResult, jsonrpc::Error>;

            /// Get the most recent accepted block number
            fn blockNumber(
                &self,
            ) -> std::result::Result<BlockNumber, jsonrpc::Error>;

            /// call a starknet function without creating a StarkNet transaction
            fn call(
                &self,
                request: FunctionCall,
                block_id: BlockId,
            ) -> std::result::Result<Vec<Felt>, jsonrpc::Error>;

            /// Return the currently configured StarkNet chain id
            fn chainId(&self) -> std::result::Result<ChainId, jsonrpc::Error>;

            /// estimate the fee for of StarkNet transactions
            fn estimateFee(
                &self,
                request: Vec<BroadcastedTxn>,
                simulation_flags: Vec<SimulationFlagForEstimateFee>,
                block_id: BlockId,
            ) -> std::result::Result<Vec<FeeEstimate>, jsonrpc::Error>;

            /// estimate the L2 fee of a message sent on L1
            fn estimateMessageFee(
                &self,
                message: MsgFromL1,
                block_id: BlockId,
            ) -> std::result::Result<FeeEstimate, jsonrpc::Error>;

            /// Get the number of transactions in a block given a block id
            fn getBlockTransactionCount(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<
                GetBlockTransactionCountResult,
                jsonrpc::Error,
            >;

            /// Get block information with full transactions and receipts given the block id
            fn getBlockWithReceipts(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<GetBlockWithReceiptsResult, jsonrpc::Error>;

            /// Get block information with transaction hashes given the block id
            fn getBlockWithTxHashes(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<GetBlockWithTxHashesResult, jsonrpc::Error>;

            /// Get block information with full transactions given the block id
            fn getBlockWithTxs(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<GetBlockWithTxsResult, jsonrpc::Error>;

            /// Get the contract class definition in the given block associated with the given hash
            fn getClass(
                &self,
                block_id: BlockId,
                class_hash: Felt,
            ) -> std::result::Result<GetClassResult, jsonrpc::Error>;

            /// Get the contract class definition in the given block at the given address
            fn getClassAt(
                &self,
                block_id: BlockId,
                contract_address: Address,
            ) -> std::result::Result<GetClassAtResult, jsonrpc::Error>;

            /// Get the contract class hash in the given block for the contract deployed at the given address
            fn getClassHashAt(
                &self,
                block_id: BlockId,
                contract_address: Address,
            ) -> std::result::Result<Felt, jsonrpc::Error>;

            /// Returns all events matching the given filter
            fn getEvents(
                &self,
                filter: GetEventsFilter,
            ) -> std::result::Result<EventsChunk, jsonrpc::Error>;

            /// Get the nonce associated with the given address in the given block
            fn getNonce(
                &self,
                block_id: BlockId,
                contract_address: Address,
            ) -> std::result::Result<Felt, jsonrpc::Error>;

            /// Get the information about the result of executing the requested block
            fn getStateUpdate(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<GetStateUpdateResult, jsonrpc::Error>;

            /// Get the value of the storage at the given address and key
            fn getStorageAt(
                &self,
                contract_address: Address,
                key: StorageKey,
                block_id: BlockId,
            ) -> std::result::Result<Felt, jsonrpc::Error>;

            /// Get the details of a transaction by a given block id and index
            fn getTransactionByBlockIdAndIndex(
                &self,
                block_id: BlockId,
                index: GetTransactionByBlockIdAndIndexIndex,
            ) -> std::result::Result<
                GetTransactionByBlockIdAndIndexResult,
                jsonrpc::Error,
            >;

            /// Get the details and status of a submitted transaction
            fn getTransactionByHash(
                &self,
                transaction_hash: TxnHash,
            ) -> std::result::Result<GetTransactionByHashResult, jsonrpc::Error>;

            /// Get the transaction receipt by the transaction hash
            fn getTransactionReceipt(
                &self,
                transaction_hash: TxnHash,
            ) -> std::result::Result<TxnReceiptWithBlockInfo, jsonrpc::Error>;

            /// Gets the transaction status (possibly reflecting that the tx is still in the mempool, or dropped from it)
            fn getTransactionStatus(
                &self,
                transaction_hash: TxnHash,
            ) -> std::result::Result<GetTransactionStatusResult, jsonrpc::Error>;

            /// Simulate a given sequence of transactions on the requested state, and generate the execution traces. Note that some of the transactions may revert, in which case no error is thrown, but revert details can be seen on the returned trace object. . Note that some of the transactions may revert, this will be reflected by the revert_error property in the trace. Other types of failures (e.g. unexpected error or failure in the validation phase) will result in TRANSACTION_EXECUTION_ERROR.
            fn simulateTransactions(
                &self,
                block_id: BlockId,
                transactions: Vec<BroadcastedTxn>,
                simulation_flags: Vec<SimulationFlag>,
            ) -> std::result::Result<Vec<SimulatedTransaction>, jsonrpc::Error>;

            /// Returns the version of the Starknet JSON-RPC specification being used
            fn specVersion(
                &self,
            ) -> std::result::Result<String, jsonrpc::Error>;

            /// Returns an object about the sync status, or false if the node is not synching
            fn syncing(
                &self,
            ) -> std::result::Result<SyncingResult, jsonrpc::Error>;

            /// Retrieve traces for all transactions in the given block
            fn traceBlockTransactions(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<Vec<BlockTransactionTrace>, jsonrpc::Error>;

            /// For a given executed transaction, return the trace of its execution, including internal calls
            fn traceTransaction(
                &self,
                transaction_hash: TxnHash,
            ) -> std::result::Result<TransactionTrace, jsonrpc::Error>;
        }

        fn handle_getProof<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(BlockId, Address, Vec<StorageKey>);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                block_id: BlockId,
                contract_address: Address,
                keys: Vec<StorageKey>,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(block_id, contract_address, keys) =
                                args_by_pos;
                            ArgByName { block_id, contract_address, keys }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { block_id, contract_address, keys } = args;

            match rpc.getProof(block_id, contract_address, keys) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_getTxStatus<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(TxnHash);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                transaction_hash: TxnHash,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(transaction_hash) = args_by_pos;
                            ArgByName { transaction_hash }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { transaction_hash } = args;

            match rpc.getTxStatus(transaction_hash) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_version<RPC: Rpc>(
            rpc: &RPC,
            _params: &Value,
        ) -> jsonrpc::Response {
            match rpc.version() {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_addDeclareTransaction<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(BroadcastedDeclareTxn);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                declare_transaction: BroadcastedDeclareTxn,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(declare_transaction) = args_by_pos;
                            ArgByName { declare_transaction }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { declare_transaction } = args;

            match rpc.addDeclareTransaction(declare_transaction) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_addDeployAccountTransaction<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(BroadcastedDeployAccountTxn);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                deploy_account_transaction: BroadcastedDeployAccountTxn,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(deploy_account_transaction) =
                                args_by_pos;
                            ArgByName { deploy_account_transaction }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { deploy_account_transaction } = args;

            match rpc.addDeployAccountTransaction(deploy_account_transaction) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_addInvokeTransaction<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(BroadcastedInvokeTxn);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                invoke_transaction: BroadcastedInvokeTxn,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(invoke_transaction) = args_by_pos;
                            ArgByName { invoke_transaction }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { invoke_transaction } = args;

            match rpc.addInvokeTransaction(invoke_transaction) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_blockHashAndNumber<RPC: Rpc>(
            rpc: &RPC,
            _params: &Value,
        ) -> jsonrpc::Response {
            match rpc.blockHashAndNumber() {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_blockNumber<RPC: Rpc>(
            rpc: &RPC,
            _params: &Value,
        ) -> jsonrpc::Response {
            match rpc.blockNumber() {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_call<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(FunctionCall, BlockId);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                request: FunctionCall,
                block_id: BlockId,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(request, block_id) = args_by_pos;
                            ArgByName { request, block_id }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { request, block_id } = args;

            match rpc.call(request, block_id) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_chainId<RPC: Rpc>(
            rpc: &RPC,
            _params: &Value,
        ) -> jsonrpc::Response {
            match rpc.chainId() {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_estimateFee<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(
                Vec<BroadcastedTxn>,
                Vec<SimulationFlagForEstimateFee>,
                BlockId,
            );

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                request: Vec<BroadcastedTxn>,
                simulation_flags: Vec<SimulationFlagForEstimateFee>,
                block_id: BlockId,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(request, simulation_flags, block_id) =
                                args_by_pos;
                            ArgByName { request, simulation_flags, block_id }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { request, simulation_flags, block_id } = args;

            match rpc.estimateFee(request, simulation_flags, block_id) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_estimateMessageFee<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(MsgFromL1, BlockId);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                message: MsgFromL1,
                block_id: BlockId,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(message, block_id) = args_by_pos;
                            ArgByName { message, block_id }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { message, block_id } = args;

            match rpc.estimateMessageFee(message, block_id) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_getBlockTransactionCount<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(BlockId);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                block_id: BlockId,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(block_id) = args_by_pos;
                            ArgByName { block_id }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { block_id } = args;

            match rpc.getBlockTransactionCount(block_id) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_getBlockWithReceipts<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(BlockId);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                block_id: BlockId,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(block_id) = args_by_pos;
                            ArgByName { block_id }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { block_id } = args;

            match rpc.getBlockWithReceipts(block_id) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_getBlockWithTxHashes<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(BlockId);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                block_id: BlockId,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(block_id) = args_by_pos;
                            ArgByName { block_id }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { block_id } = args;

            match rpc.getBlockWithTxHashes(block_id) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_getBlockWithTxs<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(BlockId);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                block_id: BlockId,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(block_id) = args_by_pos;
                            ArgByName { block_id }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { block_id } = args;

            match rpc.getBlockWithTxs(block_id) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_getClass<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(BlockId, Felt);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                block_id: BlockId,
                class_hash: Felt,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(block_id, class_hash) = args_by_pos;
                            ArgByName { block_id, class_hash }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { block_id, class_hash } = args;

            match rpc.getClass(block_id, class_hash) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_getClassAt<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(BlockId, Address);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                block_id: BlockId,
                contract_address: Address,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(block_id, contract_address) =
                                args_by_pos;
                            ArgByName { block_id, contract_address }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { block_id, contract_address } = args;

            match rpc.getClassAt(block_id, contract_address) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_getClassHashAt<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(BlockId, Address);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                block_id: BlockId,
                contract_address: Address,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(block_id, contract_address) =
                                args_by_pos;
                            ArgByName { block_id, contract_address }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { block_id, contract_address } = args;

            match rpc.getClassHashAt(block_id, contract_address) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_getEvents<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(GetEventsFilter);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                filter: GetEventsFilter,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(filter) = args_by_pos;
                            ArgByName { filter }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { filter } = args;

            match rpc.getEvents(filter) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_getNonce<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(BlockId, Address);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                block_id: BlockId,
                contract_address: Address,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(block_id, contract_address) =
                                args_by_pos;
                            ArgByName { block_id, contract_address }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { block_id, contract_address } = args;

            match rpc.getNonce(block_id, contract_address) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_getStateUpdate<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(BlockId);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                block_id: BlockId,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(block_id) = args_by_pos;
                            ArgByName { block_id }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { block_id } = args;

            match rpc.getStateUpdate(block_id) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_getStorageAt<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(Address, StorageKey, BlockId);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                contract_address: Address,
                key: StorageKey,
                block_id: BlockId,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(contract_address, key, block_id) =
                                args_by_pos;
                            ArgByName { contract_address, key, block_id }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { contract_address, key, block_id } = args;

            match rpc.getStorageAt(contract_address, key, block_id) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_getTransactionByBlockIdAndIndex<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(BlockId, GetTransactionByBlockIdAndIndexIndex);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                block_id: BlockId,
                index: GetTransactionByBlockIdAndIndexIndex,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(block_id, index) = args_by_pos;
                            ArgByName { block_id, index }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { block_id, index } = args;

            match rpc.getTransactionByBlockIdAndIndex(block_id, index) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_getTransactionByHash<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(TxnHash);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                transaction_hash: TxnHash,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(transaction_hash) = args_by_pos;
                            ArgByName { transaction_hash }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { transaction_hash } = args;

            match rpc.getTransactionByHash(transaction_hash) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_getTransactionReceipt<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(TxnHash);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                transaction_hash: TxnHash,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(transaction_hash) = args_by_pos;
                            ArgByName { transaction_hash }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { transaction_hash } = args;

            match rpc.getTransactionReceipt(transaction_hash) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_getTransactionStatus<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(TxnHash);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                transaction_hash: TxnHash,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(transaction_hash) = args_by_pos;
                            ArgByName { transaction_hash }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { transaction_hash } = args;

            match rpc.getTransactionStatus(transaction_hash) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_simulateTransactions<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(BlockId, Vec<BroadcastedTxn>, Vec<SimulationFlag>);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                block_id: BlockId,
                transactions: Vec<BroadcastedTxn>,
                simulation_flags: Vec<SimulationFlag>,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(
                                block_id,
                                transactions,
                                simulation_flags,
                            ) = args_by_pos;
                            ArgByName {
                                block_id,
                                transactions,
                                simulation_flags,
                            }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { block_id, transactions, simulation_flags } = args;

            match rpc.simulateTransactions(
                block_id,
                transactions,
                simulation_flags,
            ) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_specVersion<RPC: Rpc>(
            rpc: &RPC,
            _params: &Value,
        ) -> jsonrpc::Response {
            match rpc.specVersion() {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_syncing<RPC: Rpc>(
            rpc: &RPC,
            _params: &Value,
        ) -> jsonrpc::Response {
            match rpc.syncing() {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(e) => jsonrpc::Response::error(1003, &format!("{e:?}")),
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_traceBlockTransactions<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(BlockId);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                block_id: BlockId,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(block_id) = args_by_pos;
                            ArgByName { block_id }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { block_id } = args;

            match rpc.traceBlockTransactions(block_id) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        fn handle_traceTransaction<RPC: Rpc>(
            rpc: &RPC,
            params: &Value,
        ) -> jsonrpc::Response {
            #[derive(Deserialize, Serialize)]
            struct ArgByPos(TxnHash);

            #[derive(Deserialize, Serialize)]
            struct ArgByName {
                transaction_hash: TxnHash,
            }

            let args = serde_json::from_value::<ArgByName>(params.clone())
                .or_else(|_| {
                    serde_json::from_value::<ArgByPos>(params.clone()).map(
                        |args_by_pos| {
                            let ArgByPos(transaction_hash) = args_by_pos;
                            ArgByName { transaction_hash }
                        },
                    )
                });

            let args: ArgByName = match args {
                Ok(args) => args,
                Err(error) => {
                    tracing::debug!(?error, "failed to parse request params");
                    return jsonrpc::Response::error(-32602, "Invalid params");
                }
            };

            let ArgByName { transaction_hash } = args;

            match rpc.traceTransaction(transaction_hash) {
                Ok(ret) => match serde_json::to_value(ret) {
                    Ok(ret) => jsonrpc::Response::result(ret),
                    Err(error) => {
                        tracing::debug!(
                            ?error,
                            "failed to parse response object"
                        );
                        jsonrpc::Response::error(-32603, "Internal error")
                    }
                },
                Err(e) => jsonrpc::Response::error(e.code, &e.message),
            }
        }

        pub fn handle<RPC: Rpc>(
            rpc: &RPC,
            req: &jsonrpc::Request,
        ) -> jsonrpc::Response {
            let params = &req.params.clone().unwrap_or_default();

            let response = match req.method.as_str() {
                "pathfinder_getProof" => handle_getProof(rpc, params),
                "pathfinder_getTxStatus" => handle_getTxStatus(rpc, params),
                "pathfinder_version" => handle_version(rpc, params),
                "starknet_addDeclareTransaction" => {
                    handle_addDeclareTransaction(rpc, params)
                }
                "starknet_addDeployAccountTransaction" => {
                    handle_addDeployAccountTransaction(rpc, params)
                }
                "starknet_addInvokeTransaction" => {
                    handle_addInvokeTransaction(rpc, params)
                }
                "starknet_blockHashAndNumber" => {
                    handle_blockHashAndNumber(rpc, params)
                }
                "starknet_blockNumber" => handle_blockNumber(rpc, params),
                "starknet_call" => handle_call(rpc, params),
                "starknet_chainId" => handle_chainId(rpc, params),
                "starknet_estimateFee" => handle_estimateFee(rpc, params),
                "starknet_estimateMessageFee" => {
                    handle_estimateMessageFee(rpc, params)
                }
                "starknet_getBlockTransactionCount" => {
                    handle_getBlockTransactionCount(rpc, params)
                }
                "starknet_getBlockWithReceipts" => {
                    handle_getBlockWithReceipts(rpc, params)
                }
                "starknet_getBlockWithTxHashes" => {
                    handle_getBlockWithTxHashes(rpc, params)
                }
                "starknet_getBlockWithTxs" => {
                    handle_getBlockWithTxs(rpc, params)
                }
                "starknet_getClass" => handle_getClass(rpc, params),
                "starknet_getClassAt" => handle_getClassAt(rpc, params),
                "starknet_getClassHashAt" => handle_getClassHashAt(rpc, params),
                "starknet_getEvents" => handle_getEvents(rpc, params),
                "starknet_getNonce" => handle_getNonce(rpc, params),
                "starknet_getStateUpdate" => handle_getStateUpdate(rpc, params),
                "starknet_getStorageAt" => handle_getStorageAt(rpc, params),
                "starknet_getTransactionByBlockIdAndIndex" => {
                    handle_getTransactionByBlockIdAndIndex(rpc, params)
                }
                "starknet_getTransactionByHash" => {
                    handle_getTransactionByHash(rpc, params)
                }
                "starknet_getTransactionReceipt" => {
                    handle_getTransactionReceipt(rpc, params)
                }
                "starknet_getTransactionStatus" => {
                    handle_getTransactionStatus(rpc, params)
                }
                "starknet_simulateTransactions" => {
                    handle_simulateTransactions(rpc, params)
                }
                "starknet_specVersion" => handle_specVersion(rpc, params),
                "starknet_syncing" => handle_syncing(rpc, params),
                "starknet_traceBlockTransactions" => {
                    handle_traceBlockTransactions(rpc, params)
                }
                "starknet_traceTransaction" => {
                    handle_traceTransaction(rpc, params)
                }
                _ => jsonrpc::Response::error(-32601, "Method not found"),
            };

            return if let Some(id) = req.id.as_ref() {
                response.with_id(id.clone())
            } else {
                response
            };
        }
    }
    pub mod client {
        use super::*;

        #[cfg(not(target_arch = "wasm32"))]
        #[async_trait::async_trait]
        pub trait HttpClient: Sync + Send {
            async fn post(
                &self,
                url: &str,
                request: &jsonrpc::Request,
            ) -> std::result::Result<jsonrpc::Response, jsonrpc::Error>;
        }

        #[cfg(target_arch = "wasm32")]
        #[async_trait::async_trait(?Send)]
        pub trait HttpClient {
            async fn post(
                &self,
                url: &str,
                request: &jsonrpc::Request,
            ) -> std::result::Result<jsonrpc::Response, jsonrpc::Error>;
        }

        #[derive(Clone)]
        pub struct Client<HTTP: HttpClient> {
            http: std::sync::Arc<HTTP>,
            pub url: String,
        }

        impl<HTTP: HttpClient> Client<HTTP> {
            pub fn new(url: &str, http: HTTP) -> Self {
                Self { url: url.to_string(), http: std::sync::Arc::new(http) }
            }
        }

        #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
        #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
        impl<HTTP: HttpClient> super::Rpc for Client<HTTP> {
            async fn getProof(
                &self,
                block_id: BlockId,
                contract_address: Address,
                keys: Vec<StorageKey>,
            ) -> std::result::Result<GetProofResult, jsonrpc::Error>
            {
                let args = (block_id, contract_address, keys);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "pathfinder_getProof".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetProofResult = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn getTxStatus(
                &self,
                transaction_hash: TxnHash,
            ) -> std::result::Result<TxGatewayStatus, jsonrpc::Error>
            {
                let args = (transaction_hash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "pathfinder_getTxStatus".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: TxGatewayStatus = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn version(
                &self,
            ) -> std::result::Result<String, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "pathfinder_version".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: String =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn addDeclareTransaction(
                &self,
                declare_transaction: BroadcastedDeclareTxn,
            ) -> std::result::Result<AddDeclareTransactionResult, jsonrpc::Error>
            {
                let args = (declare_transaction,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_addDeclareTransaction".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: AddDeclareTransactionResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn addDeployAccountTransaction(
                &self,
                deploy_account_transaction: BroadcastedDeployAccountTxn,
            ) -> std::result::Result<
                AddDeployAccountTransactionResult,
                jsonrpc::Error,
            > {
                let args = (deploy_account_transaction,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_addDeployAccountTransaction".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: AddDeployAccountTransactionResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn addInvokeTransaction(
                &self,
                invoke_transaction: BroadcastedInvokeTxn,
            ) -> std::result::Result<AddInvokeTransactionResult, jsonrpc::Error>
            {
                let args = (invoke_transaction,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_addInvokeTransaction".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: AddInvokeTransactionResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn blockHashAndNumber(
                &self,
            ) -> std::result::Result<BlockHashAndNumberResult, jsonrpc::Error>
            {
                let req = jsonrpc::Request::new(
                    "starknet_blockHashAndNumber".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: BlockHashAndNumberResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn blockNumber(
                &self,
            ) -> std::result::Result<BlockNumber, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "starknet_blockNumber".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: BlockNumber = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn call(
                &self,
                request: FunctionCall,
                block_id: BlockId,
            ) -> std::result::Result<Vec<Felt>, jsonrpc::Error> {
                let args = (request, block_id);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req =
                    jsonrpc::Request::new("starknet_call".to_string(), params)
                        .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: Vec<Felt> = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn chainId(
                &self,
            ) -> std::result::Result<ChainId, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "starknet_chainId".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: ChainId =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn estimateFee(
                &self,
                request: Vec<BroadcastedTxn>,
                simulation_flags: Vec<SimulationFlagForEstimateFee>,
                block_id: BlockId,
            ) -> std::result::Result<Vec<FeeEstimate>, jsonrpc::Error>
            {
                let args = (request, simulation_flags, block_id);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_estimateFee".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: Vec<FeeEstimate> = serde_json::from_value(value)
                        .map_err(|e| {
                        jsonrpc::Error::new(
                            5002,
                            format!("Invalid response object: {e}."),
                        )
                    })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn estimateMessageFee(
                &self,
                message: MsgFromL1,
                block_id: BlockId,
            ) -> std::result::Result<FeeEstimate, jsonrpc::Error> {
                let args = (message, block_id);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_estimateMessageFee".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: FeeEstimate = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn getBlockTransactionCount(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<
                GetBlockTransactionCountResult,
                jsonrpc::Error,
            > {
                let args = (block_id,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getBlockTransactionCount".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetBlockTransactionCountResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn getBlockWithReceipts(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<GetBlockWithReceiptsResult, jsonrpc::Error>
            {
                let args = (block_id,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getBlockWithReceipts".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetBlockWithReceiptsResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn getBlockWithTxHashes(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<GetBlockWithTxHashesResult, jsonrpc::Error>
            {
                let args = (block_id,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getBlockWithTxHashes".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetBlockWithTxHashesResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn getBlockWithTxs(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<GetBlockWithTxsResult, jsonrpc::Error>
            {
                let args = (block_id,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getBlockWithTxs".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetBlockWithTxsResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn getClass(
                &self,
                block_id: BlockId,
                class_hash: Felt,
            ) -> std::result::Result<GetClassResult, jsonrpc::Error>
            {
                let args = (block_id, class_hash);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getClass".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetClassResult = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn getClassAt(
                &self,
                block_id: BlockId,
                contract_address: Address,
            ) -> std::result::Result<GetClassAtResult, jsonrpc::Error>
            {
                let args = (block_id, contract_address);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getClassAt".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetClassAtResult = serde_json::from_value(value)
                        .map_err(|e| {
                        jsonrpc::Error::new(
                            5002,
                            format!("Invalid response object: {e}."),
                        )
                    })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn getClassHashAt(
                &self,
                block_id: BlockId,
                contract_address: Address,
            ) -> std::result::Result<Felt, jsonrpc::Error> {
                let args = (block_id, contract_address);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getClassHashAt".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: Felt =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn getEvents(
                &self,
                filter: GetEventsFilter,
            ) -> std::result::Result<EventsChunk, jsonrpc::Error> {
                let args = (filter,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getEvents".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: EventsChunk = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn getNonce(
                &self,
                block_id: BlockId,
                contract_address: Address,
            ) -> std::result::Result<Felt, jsonrpc::Error> {
                let args = (block_id, contract_address);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getNonce".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: Felt =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn getStateUpdate(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<GetStateUpdateResult, jsonrpc::Error>
            {
                let args = (block_id,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getStateUpdate".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetStateUpdateResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn getStorageAt(
                &self,
                contract_address: Address,
                key: StorageKey,
                block_id: BlockId,
            ) -> std::result::Result<Felt, jsonrpc::Error> {
                let args = (contract_address, key, block_id);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getStorageAt".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: Felt =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn getTransactionByBlockIdAndIndex(
                &self,
                block_id: BlockId,
                index: GetTransactionByBlockIdAndIndexIndex,
            ) -> std::result::Result<
                GetTransactionByBlockIdAndIndexResult,
                jsonrpc::Error,
            > {
                let args = (block_id, index);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getTransactionByBlockIdAndIndex".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetTransactionByBlockIdAndIndexResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn getTransactionByHash(
                &self,
                transaction_hash: TxnHash,
            ) -> std::result::Result<GetTransactionByHashResult, jsonrpc::Error>
            {
                let args = (transaction_hash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getTransactionByHash".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetTransactionByHashResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn getTransactionReceipt(
                &self,
                transaction_hash: TxnHash,
            ) -> std::result::Result<TxnReceiptWithBlockInfo, jsonrpc::Error>
            {
                let args = (transaction_hash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getTransactionReceipt".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: TxnReceiptWithBlockInfo =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn getTransactionStatus(
                &self,
                transaction_hash: TxnHash,
            ) -> std::result::Result<GetTransactionStatusResult, jsonrpc::Error>
            {
                let args = (transaction_hash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_getTransactionStatus".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: GetTransactionStatusResult =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn simulateTransactions(
                &self,
                block_id: BlockId,
                transactions: Vec<BroadcastedTxn>,
                simulation_flags: Vec<SimulationFlag>,
            ) -> std::result::Result<Vec<SimulatedTransaction>, jsonrpc::Error>
            {
                let args = (block_id, transactions, simulation_flags);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_simulateTransactions".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: Vec<SimulatedTransaction> =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn specVersion(
                &self,
            ) -> std::result::Result<String, jsonrpc::Error> {
                let req = jsonrpc::Request::new(
                    "starknet_specVersion".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: String =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn syncing(
                &self,
            ) -> std::result::Result<SyncingResult, jsonrpc::Error>
            {
                let req = jsonrpc::Request::new(
                    "starknet_syncing".to_string(),
                    serde_json::Value::Array(vec![]),
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: SyncingResult = serde_json::from_value(value)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn traceBlockTransactions(
                &self,
                block_id: BlockId,
            ) -> std::result::Result<Vec<BlockTransactionTrace>, jsonrpc::Error>
            {
                let args = (block_id,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_traceBlockTransactions".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: Vec<BlockTransactionTrace> =
                        serde_json::from_value(value).map_err(|e| {
                            jsonrpc::Error::new(
                                5002,
                                format!("Invalid response object: {e}."),
                            )
                        })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }

            async fn traceTransaction(
                &self,
                transaction_hash: TxnHash,
            ) -> std::result::Result<TransactionTrace, jsonrpc::Error>
            {
                let args = (transaction_hash,);

                let params: serde_json::Value = serde_json::to_value(args)
                    .map_err(|e| {
                        jsonrpc::Error::new(
                            4001,
                            format!("Invalid params: {e}."),
                        )
                    })?;
                let req = jsonrpc::Request::new(
                    "starknet_traceTransaction".to_string(),
                    params,
                )
                .with_id(jsonrpc::Id::Number(1));

                tracing::debug!(request=?req, "processing");
                let mut res: jsonrpc::Response =
                    self.http.post(&self.url, &req).await?;
                tracing::debug!(response=?res, "processing");

                if let Some(err) = res.error.take() {
                    tracing::error!(error=?err, "failed");
                    return Err(err);
                }

                if let Some(value) = res.result.take() {
                    let ret: TransactionTrace = serde_json::from_value(value)
                        .map_err(|e| {
                        jsonrpc::Error::new(
                            5002,
                            format!("Invalid response object: {e}."),
                        )
                    })?;

                    tracing::debug!(result=?ret, "ready");

                    Ok(ret)
                } else {
                    tracing::error!("both error and result are missing");
                    Err(jsonrpc::Error::new(
                        5003,
                        "Response missing".to_string(),
                    ))
                }
            }
        }

        pub mod blocking {
            use super::*;

            #[cfg(not(target_arch = "wasm32"))]
            pub trait HttpClient: Sync + Send {
                fn post(
                    &self,
                    url: &str,
                    request: &jsonrpc::Request,
                ) -> std::result::Result<jsonrpc::Response, jsonrpc::Error>;
            }

            #[cfg(target_arch = "wasm32")]
            pub trait HttpClient {
                fn post(
                    &self,
                    url: &str,
                    request: &jsonrpc::Request,
                ) -> std::result::Result<jsonrpc::Response, jsonrpc::Error>;
            }

            #[derive(Clone)]
            pub struct Client<HTTP: HttpClient> {
                http: HTTP,
                pub url: String,
            }

            impl<HTTP: HttpClient> Client<HTTP> {
                pub fn new(url: &str, http: HTTP) -> Self {
                    Self { url: url.to_string(), http }
                }
            }

            impl<HTTP: HttpClient> super::super::blocking::Rpc for Client<HTTP> {
                fn getProof(
                    &self,
                    block_id: BlockId,
                    contract_address: Address,
                    keys: Vec<StorageKey>,
                ) -> std::result::Result<GetProofResult, jsonrpc::Error>
                {
                    let args = (block_id, contract_address, keys);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "pathfinder_getProof".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: GetProofResult = serde_json::from_value(value)
                            .map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn getTxStatus(
                    &self,
                    transaction_hash: TxnHash,
                ) -> std::result::Result<TxGatewayStatus, jsonrpc::Error>
                {
                    let args = (transaction_hash,);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "pathfinder_getTxStatus".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: TxGatewayStatus =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn version(
                    &self,
                ) -> std::result::Result<String, jsonrpc::Error>
                {
                    let req = jsonrpc::Request::new(
                        "pathfinder_version".to_string(),
                        serde_json::Value::Array(vec![]),
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: String = serde_json::from_value(value)
                            .map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn addDeclareTransaction(
                    &self,
                    declare_transaction: BroadcastedDeclareTxn,
                ) -> std::result::Result<
                    AddDeclareTransactionResult,
                    jsonrpc::Error,
                > {
                    let args = (declare_transaction,);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_addDeclareTransaction".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: AddDeclareTransactionResult =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn addDeployAccountTransaction(
                    &self,
                    deploy_account_transaction: BroadcastedDeployAccountTxn,
                ) -> std::result::Result<
                    AddDeployAccountTransactionResult,
                    jsonrpc::Error,
                > {
                    let args = (deploy_account_transaction,);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_addDeployAccountTransaction".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: AddDeployAccountTransactionResult =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn addInvokeTransaction(
                    &self,
                    invoke_transaction: BroadcastedInvokeTxn,
                ) -> std::result::Result<
                    AddInvokeTransactionResult,
                    jsonrpc::Error,
                > {
                    let args = (invoke_transaction,);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_addInvokeTransaction".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: AddInvokeTransactionResult =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn blockHashAndNumber(
                    &self,
                ) -> std::result::Result<BlockHashAndNumberResult, jsonrpc::Error>
                {
                    let req = jsonrpc::Request::new(
                        "starknet_blockHashAndNumber".to_string(),
                        serde_json::Value::Array(vec![]),
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: BlockHashAndNumberResult =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn blockNumber(
                    &self,
                ) -> std::result::Result<BlockNumber, jsonrpc::Error>
                {
                    let req = jsonrpc::Request::new(
                        "starknet_blockNumber".to_string(),
                        serde_json::Value::Array(vec![]),
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: BlockNumber = serde_json::from_value(value)
                            .map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn call(
                    &self,
                    request: FunctionCall,
                    block_id: BlockId,
                ) -> std::result::Result<Vec<Felt>, jsonrpc::Error>
                {
                    let args = (request, block_id);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_call".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: Vec<Felt> = serde_json::from_value(value)
                            .map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn chainId(
                    &self,
                ) -> std::result::Result<ChainId, jsonrpc::Error>
                {
                    let req = jsonrpc::Request::new(
                        "starknet_chainId".to_string(),
                        serde_json::Value::Array(vec![]),
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: ChainId = serde_json::from_value(value)
                            .map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn estimateFee(
                    &self,
                    request: Vec<BroadcastedTxn>,
                    simulation_flags: Vec<SimulationFlagForEstimateFee>,
                    block_id: BlockId,
                ) -> std::result::Result<Vec<FeeEstimate>, jsonrpc::Error>
                {
                    let args = (request, simulation_flags, block_id);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_estimateFee".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: Vec<FeeEstimate> =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn estimateMessageFee(
                    &self,
                    message: MsgFromL1,
                    block_id: BlockId,
                ) -> std::result::Result<FeeEstimate, jsonrpc::Error>
                {
                    let args = (message, block_id);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_estimateMessageFee".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: FeeEstimate = serde_json::from_value(value)
                            .map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn getBlockTransactionCount(
                    &self,
                    block_id: BlockId,
                ) -> std::result::Result<
                    GetBlockTransactionCountResult,
                    jsonrpc::Error,
                > {
                    let args = (block_id,);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_getBlockTransactionCount".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: GetBlockTransactionCountResult =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn getBlockWithReceipts(
                    &self,
                    block_id: BlockId,
                ) -> std::result::Result<
                    GetBlockWithReceiptsResult,
                    jsonrpc::Error,
                > {
                    let args = (block_id,);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_getBlockWithReceipts".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: GetBlockWithReceiptsResult =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn getBlockWithTxHashes(
                    &self,
                    block_id: BlockId,
                ) -> std::result::Result<
                    GetBlockWithTxHashesResult,
                    jsonrpc::Error,
                > {
                    let args = (block_id,);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_getBlockWithTxHashes".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: GetBlockWithTxHashesResult =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn getBlockWithTxs(
                    &self,
                    block_id: BlockId,
                ) -> std::result::Result<GetBlockWithTxsResult, jsonrpc::Error>
                {
                    let args = (block_id,);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_getBlockWithTxs".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: GetBlockWithTxsResult =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn getClass(
                    &self,
                    block_id: BlockId,
                    class_hash: Felt,
                ) -> std::result::Result<GetClassResult, jsonrpc::Error>
                {
                    let args = (block_id, class_hash);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_getClass".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: GetClassResult = serde_json::from_value(value)
                            .map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn getClassAt(
                    &self,
                    block_id: BlockId,
                    contract_address: Address,
                ) -> std::result::Result<GetClassAtResult, jsonrpc::Error>
                {
                    let args = (block_id, contract_address);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_getClassAt".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: GetClassAtResult =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn getClassHashAt(
                    &self,
                    block_id: BlockId,
                    contract_address: Address,
                ) -> std::result::Result<Felt, jsonrpc::Error> {
                    let args = (block_id, contract_address);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_getClassHashAt".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: Felt =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn getEvents(
                    &self,
                    filter: GetEventsFilter,
                ) -> std::result::Result<EventsChunk, jsonrpc::Error>
                {
                    let args = (filter,);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_getEvents".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: EventsChunk = serde_json::from_value(value)
                            .map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn getNonce(
                    &self,
                    block_id: BlockId,
                    contract_address: Address,
                ) -> std::result::Result<Felt, jsonrpc::Error> {
                    let args = (block_id, contract_address);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_getNonce".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: Felt =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn getStateUpdate(
                    &self,
                    block_id: BlockId,
                ) -> std::result::Result<GetStateUpdateResult, jsonrpc::Error>
                {
                    let args = (block_id,);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_getStateUpdate".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: GetStateUpdateResult =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn getStorageAt(
                    &self,
                    contract_address: Address,
                    key: StorageKey,
                    block_id: BlockId,
                ) -> std::result::Result<Felt, jsonrpc::Error> {
                    let args = (contract_address, key, block_id);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_getStorageAt".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: Felt =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn getTransactionByBlockIdAndIndex(
                    &self,
                    block_id: BlockId,
                    index: GetTransactionByBlockIdAndIndexIndex,
                ) -> std::result::Result<
                    GetTransactionByBlockIdAndIndexResult,
                    jsonrpc::Error,
                > {
                    let args = (block_id, index);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_getTransactionByBlockIdAndIndex".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: GetTransactionByBlockIdAndIndexResult =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn getTransactionByHash(
                    &self,
                    transaction_hash: TxnHash,
                ) -> std::result::Result<
                    GetTransactionByHashResult,
                    jsonrpc::Error,
                > {
                    let args = (transaction_hash,);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_getTransactionByHash".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: GetTransactionByHashResult =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn getTransactionReceipt(
                    &self,
                    transaction_hash: TxnHash,
                ) -> std::result::Result<TxnReceiptWithBlockInfo, jsonrpc::Error>
                {
                    let args = (transaction_hash,);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_getTransactionReceipt".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: TxnReceiptWithBlockInfo =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn getTransactionStatus(
                    &self,
                    transaction_hash: TxnHash,
                ) -> std::result::Result<
                    GetTransactionStatusResult,
                    jsonrpc::Error,
                > {
                    let args = (transaction_hash,);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_getTransactionStatus".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: GetTransactionStatusResult =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn simulateTransactions(
                    &self,
                    block_id: BlockId,
                    transactions: Vec<BroadcastedTxn>,
                    simulation_flags: Vec<SimulationFlag>,
                ) -> std::result::Result<
                    Vec<SimulatedTransaction>,
                    jsonrpc::Error,
                > {
                    let args = (block_id, transactions, simulation_flags);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_simulateTransactions".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: Vec<SimulatedTransaction> =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn specVersion(
                    &self,
                ) -> std::result::Result<String, jsonrpc::Error>
                {
                    let req = jsonrpc::Request::new(
                        "starknet_specVersion".to_string(),
                        serde_json::Value::Array(vec![]),
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: String = serde_json::from_value(value)
                            .map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn syncing(
                    &self,
                ) -> std::result::Result<SyncingResult, jsonrpc::Error>
                {
                    let req = jsonrpc::Request::new(
                        "starknet_syncing".to_string(),
                        serde_json::Value::Array(vec![]),
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: SyncingResult = serde_json::from_value(value)
                            .map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn traceBlockTransactions(
                    &self,
                    block_id: BlockId,
                ) -> std::result::Result<
                    Vec<BlockTransactionTrace>,
                    jsonrpc::Error,
                > {
                    let args = (block_id,);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_traceBlockTransactions".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: Vec<BlockTransactionTrace> =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }

                fn traceTransaction(
                    &self,
                    transaction_hash: TxnHash,
                ) -> std::result::Result<TransactionTrace, jsonrpc::Error>
                {
                    let args = (transaction_hash,);

                    let params: serde_json::Value = serde_json::to_value(args)
                        .map_err(|e| {
                            jsonrpc::Error::new(
                                4001,
                                format!("Invalid params: {e}."),
                            )
                        })?;
                    let req = jsonrpc::Request::new(
                        "starknet_traceTransaction".to_string(),
                        params,
                    )
                    .with_id(jsonrpc::Id::Number(1));

                    tracing::debug!(request=?req, "processing");
                    let mut res: jsonrpc::Response =
                        self.http.post(&self.url, &req)?;
                    tracing::debug!(response=?res, "processing");

                    if let Some(err) = res.error.take() {
                        tracing::error!(error=?err, "failed");
                        return Err(err);
                    }

                    if let Some(value) = res.result.take() {
                        let ret: TransactionTrace =
                            serde_json::from_value(value).map_err(|e| {
                                jsonrpc::Error::new(
                                    5002,
                                    format!("Invalid response object: {e}."),
                                )
                            })?;

                        tracing::debug!(result=?ret, "ready");

                        Ok(ret)
                    } else {
                        tracing::error!("both error and result are missing");
                        Err(jsonrpc::Error::new(
                            5003,
                            "Response missing".to_string(),
                        ))
                    }
                }
            }
        }
    }
}
// ^^^ GENERATED CODE ABOVE ^^^
