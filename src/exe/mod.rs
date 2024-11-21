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

        let ret = self.client
            .getNonce(block_id, contract_address)
            .map_err(Into::<Error>::into)?;

        let nonce = Nonce(ret.try_into().unwrap());
        Ok(nonce)
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
        tracing::info!(?class_hash, ?compiled_class_hash, "set_compiled_class_hash");
        Ok(())
    }

    fn add_visited_pcs(&mut self, class_hash: ClassHash, pcs: &HashSet<usize>) {
        tracing::info!(?class_hash, pcs.len = pcs.len(), "add_visited_pcs");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use blockifier::execution::contract_class::ContractClassV1;
    use starknet_api::core::PatriciaKey;
    use std::collections::VecDeque;
    use std::sync::RwLock;


    #[tokio::test]
    async fn test_get_nonce_at() {
        use wiremock::{MockServer, Mock, Request, ResponseTemplate};
        use wiremock::matchers::{method, path};

        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(|req: &Request| {
                let data: iamgroot::jsonrpc::Request = serde_json::from_slice(&req.body).unwrap();
                let body_string = match data.method.as_str() {
                    "starknet_getNonce" => {
                        r#"{
                            "jsonrpc": "2.0",
                            "result": "0x51",
                            "id": 1
                        }"#
                    },
                    "starknet_getClassHashAt" => {
                        r#"{
                            "jsonrpc": "2.0",
                            "result": "0xd0e183745e9dae3e4e78a8ffedcce0903fc4900beace4e0abf192d4c202da3",
                            "id": 1
                        }"# 
                    },
                    _ => "",
                };

                ResponseTemplate::new(200).set_body_string(body_string)
            })
            .mount(&mock_server)
            .await;

            let client = gen::client::blocking::Client::new(&mock_server.uri(), crate::client::Http::new());

            let state = State {
                    block_number: 0,
                    block_hash: gen::Felt::try_new("0x0").unwrap(),
                    root: gen::Felt::try_new("0x0").unwrap(),
            };

            let state_proxy = StateProxy { client, state };

            let result = state_proxy.get_nonce_at(
                ContractAddress(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap())
            ).unwrap();

            assert_eq!(result, Nonce(starknet_crypto::Felt::from_hex_unchecked("0x51")));


            drop(mock_server);

            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(|req: &Request| {
                let data: iamgroot::jsonrpc::Request = serde_json::from_slice(&req.body).unwrap();
                let body_string = match data.method.as_str() {
                    "starknet_getNonce" => {
                        r#"{
                            "jsonrpc": "2.0",
                            "result": "0x51",
                            "error": {
                                "code": 1,
                                "message": "test"

                            },
                            "id": 1
                        }"#
                    },
                    "starknet_getClassHashAt" => {
                        r#"{
                            "jsonrpc": "2.0",
                            "result": "0xd0e183745e9dae3e4e78a8ffedcce0903fc4900beace4e0abf192d4c202da3",
                            "id": 1
                        }"# 
                    },
                    _ => "",
                };

                ResponseTemplate::new(200).set_body_string(body_string)
            })
            .mount(&mock_server)
            .await;
    }

    struct MockHttpClient {
        responses:  RwLock<VecDeque<std::result::Result<iamgroot::jsonrpc::Response, iamgroot::jsonrpc::Error>>>,
    }

    impl MockHttpClient {
        fn new(responses: RwLock<VecDeque<std::result::Result<iamgroot::jsonrpc::Response, iamgroot::jsonrpc::Error>>>) -> Self {
            Self {
                responses,
            }
        }
    }

    impl gen::client::blocking::HttpClient for MockHttpClient {
        fn post(
            &self,
            _url: &str,
            _request: &iamgroot::jsonrpc::Request,
        ) -> std::result::Result<iamgroot::jsonrpc::Response, iamgroot::jsonrpc::Error> {

            match self.responses.write().unwrap().pop_front() {
                Some(response) => return response,
                None => return Err(iamgroot::jsonrpc::Error::new(32101, format!("request failed")))
            }
        }
    }

    fn state_proxy_with_responses(responses: Vec<std::result::Result<iamgroot::jsonrpc::Response, iamgroot::jsonrpc::Error>>) -> StateProxy<MockHttpClient>{
        StateProxy {
            client: gen::client::blocking::Client::new(
                "test",
                MockHttpClient::new(RwLock::new(VecDeque::from(responses)))
            ),
            state: State {
                block_number: 0,
                block_hash: gen::Felt::try_new("0x0").unwrap(),
                root: gen::Felt::try_new("0x0").unwrap(),
            }
        }
    }

    #[test]
    fn test_get_class_hash_at() {
        let state_proxy = state_proxy_with_responses(
            vec![
                Ok(iamgroot::jsonrpc::Response::result(serde_json::json!("0x1"))),
            ]
        );
        assert_eq!(
            state_proxy.get_class_hash_at(
                ContractAddress(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
            ).unwrap(),
            ClassHash(StarkFelt::from_hex("0x1").unwrap()),
        );

        let state_proxy = state_proxy_with_responses(
            vec![
                Err(iamgroot::jsonrpc::Error::new(32101, format!("request failed"))), 
            ]
        );

        assert!(matches!(
            state_proxy.get_class_hash_at(
                ContractAddress(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
            ).unwrap_err(),
            StateError::StateReadError(..)
        ));


        let state_proxy = StateProxy {
            client: gen::client::blocking::Client::new("test", MockHttpClient::new(RwLock::new(
                VecDeque::from(vec![
                    Ok(iamgroot::jsonrpc::Response::result(serde_json::json!("0x1"))),
                ])
            ))),
            state: State {
                block_number: 0,
                block_hash: gen::Felt::try_new("0x2").unwrap(),
                root: gen::Felt::try_new("0x0").unwrap(),
            }
        };

        assert_eq!(
            state_proxy.get_class_hash_at(
                ContractAddress(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
            ).unwrap(),
            ClassHash(StarkFelt::from_hex("0x1").unwrap()),
        );

        

    }


    #[test]
    fn test_get_storage_at_skips_proof_for_zero_value_storage() {
        let state_proxy = state_proxy_with_responses(
            vec![
                Ok(iamgroot::jsonrpc::Response::result(serde_json::json!("0x0"))),
            ]
        );
        assert_eq!(
            state_proxy.get_storage_at(
                ContractAddress(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
                StarknetStorageKey(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
            ).unwrap(),
            StarkFelt::from_hex("0x0").unwrap()
        );
    }

    #[test]
    fn test_get_storage_at_returns_error_when_rpc_call_returns_error() {
        let state_proxy = state_proxy_with_responses(
            vec![
                Err(iamgroot::jsonrpc::Error::new(32101, format!("request failed"))),
            ]
        );

        let err = state_proxy.get_storage_at(
            ContractAddress(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
            StarknetStorageKey(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
        ).unwrap_err();
        assert!(matches!(err, StateError::StateReadError(..)));

        let state_proxy = state_proxy_with_responses(
            vec![
                Ok(iamgroot::jsonrpc::Response::result(serde_json::json!("0x1"))),
                Err(iamgroot::jsonrpc::Error::new(32101, format!("request failed"))),
            ]
        );
        let err = state_proxy.get_storage_at(
            ContractAddress(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
            StarknetStorageKey(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
        ).unwrap_err();
        assert!(matches!(err, StateError::StateReadError(..)));
    }

    #[test]
    fn test_get_storage_at_returns_error_for_invalid_storage_key() {
        let state_proxy = state_proxy_with_responses(
            vec![
                Err(iamgroot::jsonrpc::Error::new(32101, format!("request failed"))),
            ]
        );
        let err = state_proxy.get_storage_at(
            ContractAddress(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
            StarknetStorageKey(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
        ).unwrap_err();
        assert!(matches!(err, StateError::StateReadError(..)));
    }


     #[test]
    fn test_get_storage_at_returns_error_for_invalid_proof() {
        let proof = serde_json::json!({
            "class_commitment": "0x0",
            "state_commitment": "0x0",
            "contract_data": {
                "class_hash": "0x0",
                "contract_state_hash_version": "0x0",
                "nonce": "0x0",
                "root": "0x0",
                "storage_proofs": [[]],
            },
            "contract_proof": [],
        });

        let state_proxy = StateProxy {
            client: gen::client::blocking::Client::new("test", MockHttpClient::new(RwLock::new(
                VecDeque::from(vec![
                    Ok(iamgroot::jsonrpc::Response::result(serde_json::json!("0x1"))),
                    Ok(iamgroot::jsonrpc::Response::result(serde_json::json!(proof))),
                ])
            ))),
            state: State {
                block_number: 0,
                block_hash: gen::Felt::try_new("0x0").unwrap(),
                root: gen::Felt::try_new("0x0").unwrap(),
            }
        };

        let err = state_proxy.get_storage_at(
            ContractAddress(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
            StarknetStorageKey(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
        ).unwrap_err();
        assert!(matches!(err, StateError::StateReadError(..)));
    }

    #[test]
    fn test_ok() {
        let proof = serde_json::json!({
            "class_commitment": "0x0",
            "state_commitment": "0x157598a5ab5c9f01da1a279e2fba356e3f7d0ee9977c80e32922f2ca5cd5d56",
            "contract_data": {
                "class_hash": "0x0",
                "contract_state_hash_version": "0x0",
                "nonce": "0x0",
                "root": "0x1e224db31dfb3e1b8c95670a12f1903d4a32ac7bb83f4b209029e14155bbca9",
                "storage_proofs": [[
                    {
                        "edge": {
                            "child": "0x47616d65206f66204c69666520546f6b656e",
                            "path": {
                                "len": 231,
                                "value": "0x3dfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1"
                            }
                        }
                    }
                ]],
            },
            "contract_proof": []
        });


        let state_proxy = StateProxy {
            client: gen::client::blocking::Client::new("test", MockHttpClient::new(RwLock::new(
                VecDeque::from(vec![
                    Ok(iamgroot::jsonrpc::Response::result(serde_json::json!("0x47616d65206f66204c69666520546f6b656e"))),
                    Ok(iamgroot::jsonrpc::Response::result(serde_json::json!(proof))),
                ])
            ))),
            state: State {
                block_number: 0,
                block_hash: gen::Felt::try_new("0x0").unwrap(),
                root: gen::Felt::try_new("0x157598a5ab5c9f01da1a279e2fba356e3f7d0ee9977c80e32922f2ca5cd5d56").unwrap(),
            }
        };

        let result = state_proxy.get_storage_at(
            ContractAddress(
                PatriciaKey::try_from(
                    starknet_crypto::Felt::from_hex("0x0341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1").unwrap()
                ).unwrap()
            ),
            StarknetStorageKey(
                PatriciaKey::try_from(
                    starknet_crypto::Felt::from_hex("0x0341c1bdfd89f69748aa00b5742b03adbffd79b8e80cab5c50d91cd8c2a79be1").unwrap()
                ).unwrap()
            )
        ).unwrap();
        assert_eq!(result, starknet_crypto::Felt::from_hex("0x47616d65206f66204c69666520546f6b656e").unwrap());
    }


    #[test]
    fn test_get_compiled_contract_class() {
        let contract_class = r#"{
            "program": "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",
            "entry_points_by_type": {
              "CONSTRUCTOR": [
                {
                  "offset": "0xA1",
                  "selector": "0x28ffe4ff0f226a9107253e17a904099aa4f63a02a5621de0576e5aa71bc5194"
                }
              ],
              "EXTERNAL": [
                {
                  "offset": "0xD0",
                  "selector": "0x0"
                }
              ],
              "L1_HANDLER": [
                {
                  "offset": "0xE9",
                  "selector": "0x0"
                }
              ]
            },
            "abi": [
              {
                "type": "event",
                "name": "Upgraded",
                "keys": [],
                "data": [
                  {
                    "name": "implementation",
                    "type": "felt"
                  }
                ]
              },
              {
                "type": "event",
                "name": "AdminChanged",
                "keys": [],
                "data": [
                  {
                    "name": "previousAdmin",
                    "type": "felt"
                  },
                  {
                    "name": "newAdmin",
                    "type": "felt"
                  }
                ]
              },
              {
                "type": "constructor",
                "name": "constructor",
                "inputs": [
                  {
                    "name": "implementation_hash",
                    "type": "felt"
                  },
                  {
                    "name": "selector",
                    "type": "felt"
                  },
                  {
                    "name": "calldata_len",
                    "type": "felt"
                  },
                  {
                    "name": "calldata",
                    "type": "felt*"
                  }
                ],
                "outputs": []
              },
              {
                "type": "function",
                "name": "__default__",
                "inputs": [
                  {
                    "name": "selector",
                    "type": "felt"
                  },
                  {
                    "name": "calldata_size",
                    "type": "felt"
                  },
                  {
                    "name": "calldata",
                    "type": "felt*"
                  }
                ],
                "outputs": [
                  {
                    "name": "retdata_size",
                    "type": "felt"
                  },
                  {
                    "name": "retdata",
                    "type": "felt*"
                  }
                ]
              },
              {
                "type": "l1_handler",
                "name": "__l1_default__",
                "inputs": [
                  {
                    "name": "selector",
                    "type": "felt"
                  },
                  {
                    "name": "calldata_size",
                    "type": "felt"
                  },
                  {
                    "name": "calldata",
                    "type": "felt*"
                  }
                ],
                "outputs": []
                }]
              }"#;

        let state_proxy = state_proxy_with_responses(vec![
            Ok(iamgroot::jsonrpc::Response::result(serde_json::from_str(contract_class).unwrap())),
        ]);

        assert!(matches!(
            state_proxy.get_compiled_contract_class(ClassHash(starknet_crypto::Felt::ZERO)).unwrap(),
            ContractClass::V0(..)
        ));

        let state_proxy = state_proxy_with_responses(vec![
            Err(iamgroot::jsonrpc::Error::new(32101, format!("request failed"))),
        ]);

        assert!(matches!(
            state_proxy.get_compiled_contract_class(ClassHash(starknet_crypto::Felt::ZERO)).unwrap_err(),
            StateError::StateReadError(..)
        ));

    }

    #[test]
    fn test_get_compiled_class_hash() {
        let state_proxy = state_proxy_with_responses(vec![]);
        assert!(matches!(
            state_proxy.get_compiled_class_hash(ClassHash(starknet_crypto::Felt::ZERO)).unwrap_err(),
            StateError::UndeclaredClassHash(..)
        ));
    }

    #[test]
    fn test_rest_methods_that_doesnt_do_anything_yet() {
        let mut state_proxy = state_proxy_with_responses(vec![]);

        state_proxy.set_storage_at(
            ContractAddress(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
            StarknetStorageKey(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
            starknet_crypto::Felt::ZERO,
        ).unwrap();
        state_proxy.increment_nonce(
            ContractAddress(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
        ).unwrap();
        state_proxy.set_class_hash_at(
            ContractAddress(PatriciaKey::try_from(starknet_crypto::Felt::ZERO).unwrap()),
            ClassHash(starknet_crypto::Felt::ZERO),
        ).unwrap();
        state_proxy.set_contract_class(ClassHash(starknet_crypto::Felt::ZERO), ContractClass::V1(ContractClassV1::empty_for_testing())).unwrap();
        state_proxy.set_compiled_class_hash(ClassHash(starknet_crypto::Felt::ZERO), CompiledClassHash(starknet_crypto::Felt::ZERO)).unwrap();
        state_proxy.add_visited_pcs(ClassHash(starknet_crypto::Felt::ZERO), &HashSet::new());

    }
}