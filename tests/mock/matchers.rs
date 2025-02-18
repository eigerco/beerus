use iamgroot::jsonrpc;
use serde_json::Value;
use wiremock::{Match, Request};

#[allow(dead_code)]
#[derive(Eq, Hash, PartialEq)]
pub enum StarknetMatcher {
    AddDeclareTransaction,
    AddDeclareTransactionMalicious,
    AddDeployAccountTransaction,
    AddInvokeTransaction,
    ChainId,
    ChainIdMalicious,
    ClassError,
    ClassSuccess,
    ClassMalicious,
    EstimateFee,
    EstimateFeeMalicious,
    GetTransactionReceipt,
    Nonce,
    NonceMalicious,
    SpecVersion,
    SpecVersionMalicious,
}

pub trait Response {
    fn response(&self) -> Value;
}

pub struct ChainIdMatcher {
    pub response: Value,
}

impl ChainIdMatcher {
    pub fn malicious() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": "malicious_result"
            }),
        }
    }
}

impl Default for ChainIdMatcher {
    fn default() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0x4b4154414e41"
            }),
        }
    }
}

impl Response for ChainIdMatcher {
    fn response(&self) -> Value {
        self.response.clone()
    }
}

impl Match for ChainIdMatcher {
    fn matches(&self, request: &Request) -> bool {
        let request = request.body_json::<jsonrpc::Request>().unwrap();
        matches!(request.method.as_str(), "starknet_chainId")
    }
}

pub struct NonceMatcher {
    pub response: Value,
}

impl NonceMatcher {
    pub fn malicious() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": "malicious_result"
            }),
        }
    }
}

impl Default for NonceMatcher {
    fn default() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0x0"
            }),
        }
    }
}

impl Match for NonceMatcher {
    fn matches(&self, request: &Request) -> bool {
        let request = request.body_json::<jsonrpc::Request>().unwrap();
        matches!(request.method.as_str(), "starknet_getNonce")
    }
}

impl Response for NonceMatcher {
    fn response(&self) -> Value {
        self.response.clone()
    }
}

pub struct ClassMatcher {
    pub response: Value,
}

impl ClassMatcher {
    pub fn error() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "error": {
                    "code": 28,
                    "message": "Class hash not found"
                }
            }),
        }
    }

    pub fn success() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": {
                    "sierra_program": ["0x1"],
                    "contract_class_version": "0.1.0",
                    "entry_points_by_type": {
                        "CONSTRUCTOR": [{
                            "selector": "0x2",
                            "function_idx": 2,
                        }],
                        "EXTERNAL": [{
                            "selector": "0x3",
                            "function_idx": 3,
                        }, {
                            "selector": "0x4",
                            "function_idx": 4,
                        }],
                        "L1_HANDLER": [],
                    },
                    "abi": "some_abi"
                }
            }),
        }
    }

    pub fn malicious() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": "malicious"
            }),
        }
    }
}

impl Match for ClassMatcher {
    fn matches(&self, request: &Request) -> bool {
        let request = request.body_json::<jsonrpc::Request>().unwrap();
        matches!(request.method.as_str(), "starknet_getClass")
    }
}

impl Response for ClassMatcher {
    fn response(&self) -> Value {
        self.response.clone()
    }
}

pub struct SpecVersionMatcher {
    pub response: Value,
}

impl SpecVersionMatcher {
    pub fn malicious() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": 42
            }),
        }
    }
}

impl Default for SpecVersionMatcher {
    fn default() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0.6.0"
            }),
        }
    }
}

impl Match for SpecVersionMatcher {
    fn matches(&self, request: &Request) -> bool {
        let request = request.body_json::<jsonrpc::Request>().unwrap();
        matches!(request.method.as_str(), "starknet_specVersion")
    }
}

impl Response for SpecVersionMatcher {
    fn response(&self) -> Value {
        self.response.clone()
    }
}

pub struct EstimateFeeMatcher {
    pub response: Value,
}

impl EstimateFeeMatcher {
    pub fn malicious() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": "malicious"
            }),
        }
    }
}

impl Default for EstimateFeeMatcher {
    fn default() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": [{
                    "gas_consumed": "0x18bf",
                    "gas_price": "0x174876e800",
                    "overall_fee": "0x2402a36771800",
                    "unit": "WEI"
                }]
            }),
        }
    }
}

impl Match for EstimateFeeMatcher {
    fn matches(&self, request: &Request) -> bool {
        let request = request.body_json::<jsonrpc::Request>().unwrap();
        matches!(request.method.as_str(), "starknet_estimateFee")
    }
}

impl Response for EstimateFeeMatcher {
    fn response(&self) -> Value {
        self.response.clone()
    }
}

pub struct AddDeclareTransactionMatcher {
    pub response: Value,
}

impl AddDeclareTransactionMatcher {
    pub fn malicious() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": 42
            }),
        }
    }
}

impl Default for AddDeclareTransactionMatcher {
    fn default() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": {
                    "transaction_hash": "0x0",
                    "class_hash": "0x1",
                }
            }),
        }
    }
}

impl Match for AddDeclareTransactionMatcher {
    fn matches(&self, request: &Request) -> bool {
        let request = request.body_json::<jsonrpc::Request>().unwrap();
        matches!(request.method.as_str(), "starknet_addDeclareTransaction")
    }
}

impl Response for AddDeclareTransactionMatcher {
    fn response(&self) -> Value {
        self.response.clone()
    }
}

pub struct AddInvokeTransactionMatcher {
    pub response: Value,
}

impl Default for AddInvokeTransactionMatcher {
    fn default() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": {
                    "transaction_hash": "0x0",
                }
            }),
        }
    }
}

impl Match for AddInvokeTransactionMatcher {
    fn matches(&self, request: &Request) -> bool {
        let request = request.body_json::<jsonrpc::Request>().unwrap();
        matches!(request.method.as_str(), "starknet_addInvokeTransaction")
    }
}

impl Response for AddInvokeTransactionMatcher {
    fn response(&self) -> Value {
        self.response.clone()
    }
}

pub struct AddDeployAccountTransactionMatcher {
    pub response: Value,
}

impl Default for AddDeployAccountTransactionMatcher {
    fn default() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": {
                    "contract_address": "0x0",
                    "transaction_hash": "0x0",
                }
            }),
        }
    }
}

impl Match for AddDeployAccountTransactionMatcher {
    fn matches(&self, request: &Request) -> bool {
        let request = request.body_json::<jsonrpc::Request>().unwrap();
        matches!(
            request.method.as_str(),
            "starknet_addDeployAccountTransaction"
        )
    }
}

impl Response for AddDeployAccountTransactionMatcher {
    fn response(&self) -> Value {
        self.response.clone()
    }
}

pub struct GetTransactionReceiptMatcher {
    pub response: Value,
}

impl Default for GetTransactionReceiptMatcher {
    fn default() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc":"2.0",
                "result": {
                    "transaction_hash":"0x1",
                    "actual_fee": {
                        "amount":"0x2",
                        "unit":"WEI"
                    },
                    "finality_status":"ACCEPTED_ON_L2",
                    "messages_sent":[],
                    "events":[{
                        "from_address":"0x3",
                        "keys":[
                            "0x4"
                        ],
                        "data":[
                            "0x5",
                            "0x6",
                            "0x7",
                            "0x8"
                        ]
                    }],
                    "execution_resources": {
                        "steps": 1,
                        "memory_holes": 2,
                        "range_check_builtin_applications": 3,
                        "pedersen_builtin_applications": 4,
                        "ec_op_builtin_applications": 5,
                        "data_availability": {
                            "l1_gas": 6,
                            "l1_data_gas": 7
                        }
                    },
                    "execution_status":"SUCCEEDED",
                    "type":"DEPLOY_ACCOUNT",
                    "contract_address":"0x9",
                    "block_hash":"0x10",
                    "block_number":8
                },
                "id":1
            }),
        }
    }
}

impl Match for GetTransactionReceiptMatcher {
    fn matches(&self, request: &Request) -> bool {
        let request = request.body_json::<jsonrpc::Request>().unwrap();
        matches!(request.method.as_str(), "starknet_getTransactionReceipt")
    }
}

impl Response for GetTransactionReceiptMatcher {
    fn response(&self) -> Value {
        self.response.clone()
    }
}
