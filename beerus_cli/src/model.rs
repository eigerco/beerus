use clap::{Parser, Subcommand};
use ethers::prelude::Log;
use ethers::types::{H256, U256};
use helios::types::ExecutionBlock;
use serde_json::json;
use starknet::core::types::FieldElement;
use starknet::providers::jsonrpc::models::{BlockHashAndNumber, ContractClass, SyncStatusType};
use std::{fmt::Display, path::PathBuf};

/// Main struct for the Beerus CLI args.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Set a custom config file
    #[arg(short, long, value_name = "FILE", global = true)]
    pub config: Option<PathBuf>,
    /// List of supported commands.
    #[command(subcommand)]
    pub command: Commands,
}

/// List of supported commands.
#[derive(Subcommand)]
pub enum Commands {
    /// Ethereum related subcommands
    #[command(about = "Ethereum related subcommands")]
    Ethereum(EthereumCommands),
    /// StarkNet related subcommands
    #[command(name = "starknet", about = "StarkNet related subcommands")]
    StarkNet(StarkNetCommands),
}

/// Ethereum related commands.
#[derive(Parser, Debug)]
pub struct EthereumCommands {
    /// Ethereum related subcommands.
    #[command(subcommand)]
    pub command: EthereumSubCommands,
}

/// Ethereum related subcommands.
#[derive(Subcommand, Debug)]
pub enum EthereumSubCommands {
    /// Sends a Raw Transaction.
    SendRawTransaction {
        /// Bytes of the Raw Transaction
        #[arg(short, long, value_name = "BYTES")]
        bytes: String,
    },
    /// Query the balance of an Ethereum address.
    QueryBalance {
        /// The address to query the balance of
        #[arg(short, long, value_name = "ADDRESS")]
        address: String,
    },
    QueryNonce {
        /// The address to query the nonce of
        #[arg(short, long, value_name = "ADDRESS")]
        address: String,
    },

    QueryBlockNumber {},

    QueryChainId {},

    QueryCode {
        /// The address of the contract to query the code
        #[arg(short, long, value_name = "ADDRESS")]
        address: String,
    },
    /// Query the transaction of an Ethereum address from the given block.
    QueryTxCount {
        /// The ethereum address
        /// The block from which to query the txs count
        #[arg(short, long, value_name = "ADDRESS")]
        address: String,
        #[arg(short, long, value_name = "BLOCK")]
        block: String,
    },
    QueryBlockTxCountByNumber {
        /// The block from which to query the txs count
        #[arg(short, long, value_name = "BLOCK")]
        block: u64,
    },
    QueryBlockTxCountByHash {
        /// The block from which to query the txs count
        #[arg(short, long, value_name = "HASH")]
        hash: String,
    },
    QueryTxByHash {
        #[arg(short, long, value_name = "HASH")]
        hash: String,
    },
    QueryGasPrice {},
    QueryEstimateGas {
        #[arg(short, long, value_name = "params")]
        params: String,
    },
    QueryBlockByHash {
        /// The block number to query
        #[arg(short, long, value_name = "BLOCK_HASH")]
        hash: String,

        /// Fetch full transaction objects or just the transaction hashes
        #[arg(short, long, value_name = "FULL_TRANSACTIONS")]
        full_tx: bool,
    },

    QueryPriorityFee {},
    QueryBlockByNumber {
        /// The block number to query
        #[arg(short, long, value_name = "BLOCK_NUMBER")]
        block: String,

        /// Fetch full transaction objects or just the transaction hashes
        #[arg(short, long, value_name = "FULL_TRANSACTIONS")]
        full_tx: bool,
    },
    /// Query Logs (blockchain events) that match
    /// the given parameters.
    QueryLogs {
        /// Address from which the log comes from.
        #[arg(short, long, value_name = "ADDRESS")]
        address: Option<String>,
        /// Equivalent to from_block = to_block,
        /// only allowed if neither from_block or to_block
        /// is supplied.
        #[arg(short, long, value_name = "BLOCK_HASH")]
        blockhash: Option<String>,
        /// Starting block to filter from, defaults to "latest".
        #[arg(short, long, value_name = "FROM_BLOCK")]
        from_block: Option<String>,
        /// Ending block to filter to, defaults to "latest".
        #[arg(short, long, value_name = "TO_BLOCK")]
        to_block: Option<String>,
        /// Topics to filter, up to 4 allowed.
        #[arg(short, long, value_name = "TOPICS", value_delimiter = ',')]
        topics: Option<Vec<String>>,
    },
}

/// StarkNet related commands.
#[derive(Parser, Debug)]
pub struct StarkNetCommands {
    /// StarkNet related subcommands.
    #[command(subcommand)]
    pub command: StarkNetSubCommands,
}

/// StarkNet related subcommands.
#[derive(Subcommand, Debug)]
pub enum StarkNetSubCommands {
    /// Query the state root of StarkNet.
    QueryStateRoot {},
    /// Query a StarkNet contract view.
    QueryContract {
        /// The address of the contract to query
        #[arg(short, long, value_name = "ADDRESS")]
        address: String,
        /// The selector of the function to call
        #[arg(short, long, value_name = "SELECTOR")]
        selector: String,
        /// The calldata of the function to call
        #[arg(long, value_name = "CALLDATA", use_value_delimiter = true)]
        calldata: Vec<String>,
    },
    QueryGetStorageAt {
        /// The address of the contract to query
        #[arg(short, long, value_name = "ADDRESS")]
        address: String,
        /// The slot of the storage to query
        #[arg(short, long, value_name = "KEY")]
        key: String,
    },
    QueryNonce {
        /// The address of the contract to query
        #[arg(short, long, value_name = "ADDRESS")]
        address: String,
    },
    L1ToL2MessageCancellations {
        /// The hash of the message
        #[arg(short, long, value_name = "MSG_HASH")]
        msg_hash: String,
    },
    L1ToL2Messages {
        /// The hash of the message
        #[arg(short, long, value_name = "MSG_HASH")]
        msg_hash: String,
    },
    L2ToL1Messages {
        /// The hash of the message
        #[arg(short, long, value_name = "MSG_HASH")]
        msg_hash: String,
    },
    /// The nonce of the L1 to L2 message bridge
    L1ToL2MessageNonce {},
    QueryChainId {},
    /// The current block number of the StarkNet network
    QueryBlockNumber {},
    /// The current block hash and number of the StarkNet network
    QueryBlockHashAndNumber {},
    /// The contract class definition
    QueryGetClass {
        /// Type of block identifier
        /// eg. hash, number, tag
        #[arg(short, long, value_name = "BLOCK_ID_TYPE")]
        block_id_type: String,
        /// The block identifier
        /// eg. 0x123, 123, pending, or latest
        #[arg(short, long, value_name = "BLOCK_ID")]
        block_id: String,
        /// The class hash
        #[arg(short, long, value_name = "CLASS_HASH")]
        class_hash: String,
    },
    /// The contract class definition
    QueryGetClassHash {
        /// Type of block identifier
        /// eg. hash, number, tag
        #[arg(short, long, value_name = "BLOCK_ID_TYPE")]
        block_id_type: String,
        /// The block identifier
        /// eg. 0x123, 123, pending, or latest
        #[arg(short, long, value_name = "BLOCK_ID")]
        block_id: String,
        /// The contract address

        #[arg(short, long, value_name = "CONTRACT_ADDRESS")]
        contract_address: String,
    },

    /// The contract class definition
    QueryGetClassAt {
        /// Type of block identifier
        /// eg. hash, number, tag
        #[arg(short, long, value_name = "BLOCK_ID_TYPE")]
        block_id_type: String,
        /// The block identifier
        /// eg. 0x123, 123, pending, or latest
        #[arg(short, long, value_name = "BLOCK_ID")]
        block_id: String,

        /// The class hash
        #[arg(short, long, value_name = "CONTRACT_ADDRESS")]
        contract_address: String,
    },
    // The number of transactions in a block given a block id of the StarkNet network
    QueryGetBlockTransactionCount {
        /// Type of block identifier
        /// eg. hash, number, tag
        #[arg(short, long, value_name = "BLOCK_ID_TYPE")]
        block_id_type: String,
        /// The block identifier
        /// eg. 0x123, 123, pending, or latest
        #[arg(short, long, value_name = "BLOCK_ID")]
        block_id: String,
    },
    QuerySyncing {},
}

/// The response from a CLI command.
pub enum CommandResponse {
    EthereumSendRawTransaction(H256),
    EthereumQueryBalance(String),
    EthereumQueryNonce(u64),
    EthereumQueryBlockNumber(u64),
    EthereumQueryChainId(u64),
    EthereumQueryCode(Vec<u8>),
    EthereumQueryTxCount(u64),
    EthereumQueryBlockTxCountByNumber(u64),
    EthereumQueryBlockTxCountByHash(u64),
    EthereumQueryTxByHash(String),
    EthereumQueryGasPrice(U256),
    EthereumQueryEstimateGas(u64),
    EthereumQueryLogs(Vec<Log>),
    EthereumQueryBlockByHash(Option<ExecutionBlock>),
    EthereumQueryGetPriorityFee(U256),
    EthereumQueryBlockByNumber(Option<ExecutionBlock>),
    StarkNetQueryStateRoot(U256),
    StarkNetQueryContract(Vec<FieldElement>),
    StarkNetQueryGetStorageAt(FieldElement),
    StarkNetQueryNonce(FieldElement),
    StarknetQueryChainId(FieldElement),
    StarknetQueryBlockNumber(u64),
    StarknetQueryBlockHashAndNumber(BlockHashAndNumber),
    StarknetQueryGetClass(ContractClass),
    StarknetQueryGetClassHash(FieldElement),
    StarknetQueryGetClassAt(ContractClass),
    StarknetQueryGetBlockTransactionCount(u64),
    StarknetQuerySyncing(SyncStatusType),
    StarkNetL1ToL2MessageCancellations(U256),
    StarkNetL1ToL2Messages(U256),
    StarkNetL1ToL2MessageNonce(U256),
    StarkNetL2ToL1Messages(U256),
}

/// Display implementation for the CLI command response.
/// This is used to print the response to the user.
impl Display for CommandResponse {
    /// See the documentation for `std::fmt::Display`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Print raw Transaction response
            // Result looks like: 0.000000000000000001 ETH
            CommandResponse::EthereumSendRawTransaction(response) => write!(f, "{response}"),
            // Print the balance in Ether.
            // Result looks like: 0.000000000000000001 ETH
            CommandResponse::EthereumQueryBalance(balance) => write!(f, "{balance} ETH"),
            // Print the address nonce.
            // Result looks like: Nonce: 10
            CommandResponse::EthereumQueryNonce(nonce) => write!(f, "Nonce: {nonce}"),
            // Print the current block number.
            // Result looks like: 123456
            CommandResponse::EthereumQueryBlockNumber(block_number) => {
                write!(f, "{block_number}")
            }
            // Print the chain id.
            // Result looks like: 1
            CommandResponse::EthereumQueryChainId(chain_id) => write!(f, "{chain_id}"),
            // Print the code of a contract in 256bits vector
            // Result looks like: [1,1,10,ff]
            //TODO: Add Opt to save the file (ex: -o code.json)
            CommandResponse::EthereumQueryCode(code) => {
                write!(f, "{code:?}")
            }
            // Print the transaction count of a given Ethereum address from a given block
            // Result looks like: 123
            CommandResponse::EthereumQueryTxCount(tx_count) => {
                write!(f, "{tx_count:?}")
            }
            // Print the count of txs of a block
            // Result looks like: 150
            CommandResponse::EthereumQueryBlockTxCountByNumber(tx_count) => {
                write!(f, "{tx_count}")
            }
            // Print the count of txs of a block
            // Result looks like: 150
            CommandResponse::EthereumQueryBlockTxCountByHash(tx_count) => {
                write!(f, "{tx_count}")
            }
            // Print the gas price from the Ethereum Network
            // Result looks like: 15000
            CommandResponse::EthereumQueryGasPrice(gas_price) => {
                write!(f, "{gas_price}")
            }
            // Print the estimated gas from the Ethereum Network
            // Result looks like: 15000
            CommandResponse::EthereumQueryEstimateGas(gas) => {
                write!(f, "{gas}")
            }

            CommandResponse::EthereumQueryLogs(logs) => {
                let logs = logs
                    .iter()
                    .map(|log| serde_json::to_string(&log).unwrap())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "[{logs}]")
            }
            // Print Block given a block hash
            // Result looks like:
            CommandResponse::EthereumQueryBlockByHash(block) => match block {
                Some(block) => {
                    let json_block = serde_json::to_string(&block).unwrap();
                    write!(f, "{json_block}")
                }
                None => write!(f, "No block found"),
            },
            // Print the max priority fee per gas from the Ethereum Network
            // Result looks like:
            CommandResponse::EthereumQueryGetPriorityFee(get_priority_fee) => {
                write!(f, "{get_priority_fee}")
            }
            CommandResponse::EthereumQueryBlockByNumber(block) => match block {
                Some(block) => {
                    let json_block = serde_json::to_string(&block).unwrap();
                    write!(f, "{json_block}")
                }
                None => write!(f, "No block found"),
            },
            // Print the state root.
            // Result looks like: 2343271987571512511202187232154229702738820280823720849834887135668366687374
            CommandResponse::StarkNetQueryStateRoot(state_root) => write!(f, "{state_root}"),
            // Print the contract view response .
            // Result looks like: [123], [456]
            CommandResponse::StarkNetQueryContract(response) => {
                let formatted_str = response
                    .iter()
                    .by_ref()
                    .map(|s| format!("{s}"))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "[{formatted_str}]")
            }
            // Print the storage value.
            // Result looks like: 15527784
            CommandResponse::StarkNetQueryGetStorageAt(response) => {
                write!(f, "{response}")
            }
            // Print the nonce value.
            // Result looks like: 3
            CommandResponse::StarkNetQueryNonce(nonce) => {
                write!(f, "{nonce}")
            }
            // Print the timestamp of the cancellation.
            // Result looks like: 123456
            // If the message was not cancelled, the timestamp will be 0.
            CommandResponse::StarkNetL1ToL2MessageCancellations(timestamp) => {
                write!(f, "{timestamp}")
            }
            // Print  msg_fee + 1 for the message with the given 'msgHash',
            // Result looks like: 123456
            CommandResponse::StarkNetL1ToL2Messages(fee) => {
                write!(f, "{fee}")
            }
            // Print the (msg_fee +1) for the message with the given 'msgHash',
            // Result looks like: 123456
            CommandResponse::StarkNetL2ToL1Messages(fee) => {
                write!(f, "{fee}")
            }
            // Print the current nonce of the L1 to L2 message bridge,
            // Result looks like: 123456
            CommandResponse::StarkNetL1ToL2MessageNonce(nonce) => {
                write!(f, "L1 to L2 Message Nonce: {nonce}")
            }
            // Print the chain id.
            // Result looks like: `Chain id: 1`
            CommandResponse::StarknetQueryChainId(chain_id) => {
                write!(f, "Chain id: {chain_id}")
            }
            // Print the current block number.
            // Result looks like: `Block number: 123456`
            CommandResponse::StarknetQueryBlockNumber(block_number) => {
                write!(f, "Block number: {block_number}")
            }

            // Print the Tx data given a Tx Hash
            // Result looks like: `Transaction Data: { hash: 0x00.. , nonce: 10, ..}`
            CommandResponse::EthereumQueryTxByHash(tx_data) => {
                write!(f, "Transaction Data: {tx_data:?}")
            }
            // Print the current block hash and number.
            // Result looks like: `Block hash: 123456, Block number: 123456`
            CommandResponse::StarknetQueryBlockHashAndNumber(response) => {
                write!(
                    f,
                    "Block hash: {}, Block number: {}",
                    response.block_hash, response.block_number
                )
            }
            // Print the contract class definition in the given block associated with the given hash.
            // Result looks like:
            // {
            //    "abi": [
            //      {
            //          "inputs": [
            //              {
            //                  "name": "amount",
            //                  "type": "felt"
            //              }
            //          ]
            //      }
            //    ],
            //    "entry_points_by_type": {
            //      "CONSTRUCTOR": [],
            //      "EXTERNAL": [],
            //      "L1_HANDLER": []
            //    },
            //    "program": "AQID"
            // }
            CommandResponse::StarknetQueryGetClass(response) => {
                let json_response = json!(
                    {
                        "program": base64::encode(&response.program),
                        "entry_points_by_type": response.entry_points_by_type,
                        "abi": response.abi.as_ref().unwrap()
                    }
                );
                write!(f, "{json_response}")
            }

            // Print the class hash.
            // Result looks like: `Class hash 12341663341423143215656`
            CommandResponse::StarknetQueryGetClassHash(response) => {
                write!(f, "Class hash: {response}")
            }
            // Print the contract class definition in the given block associated with the given hash.
            // Result looks like:
            // {
            //    "abi": [
            //      {
            //          "inputs": [
            //              {
            //                  "name": "amount",
            //                  "type": "felt"
            //              }
            //          ]
            //      }
            //    ],
            //    "entry_points_by_type": {
            //      "CONSTRUCTOR": [],
            //      "EXTERNAL": [],
            //      "L1_HANDLER": []
            //    },
            //    "program": "AQID"
            // }
            CommandResponse::StarknetQueryGetClassAt(response) => {
                let json_response = json!(
                    {
                        "program": base64::encode(&response.program),
                        "entry_points_by_type": response.entry_points_by_type,
                        "abi": response.abi.as_ref().unwrap()
                    }
                );
                write!(f, "{json_response}")
            }
            // Print the number of transactions in a block.
            // Result looks like: `Block transaction count: 240`
            CommandResponse::StarknetQueryGetBlockTransactionCount(block_transaction_count) => {
                write!(f, "Block transaction count: {block_transaction_count}")
            }
            // Print an object about the sync status of a node
            // Result looks like:
            // {
            // "status": "Syncing",
            // "data": {
            //     "current_block_hash": "0x326fc63ee7013fba27182bc323b2aec846b0e459269fe23cb62f433ddcc2b7",
            //     "current_block_num": "0x971d4",
            //     "highest_block_hash": "0x326fc63ee7013fba27182bc323b2aec846b0e459269fe23cb62f433ddcc2b7",
            //     "highest_block_num": "0x971d4",
            //     "starting_block_hash": "0x5156662f793e667af6624e27e89e1fa49fdabb0b9ff77b56a83782367f2744d",
            //     "starting_block_num": "0x95064"
            //     }
            // }
            //
            // or
            //
            // {
            //     "status": "NotSyncing",
            //     "data": null
            // }
            CommandResponse::StarknetQuerySyncing(response) => match response {
                SyncStatusType::Syncing(status) => {
                    let json_response = json!(
                        {
                            "status": "Syncing",
                            "data": status,
                        }
                    );
                    write!(f, "{json_response}")
                }
                SyncStatusType::NotSyncing => {
                    let json_response = json!(
                        {
                            "status": "NotSyncing",
                            "data": null,
                        }
                    );
                    write!(f, "{json_response}")
                }
            },
        }
    }
}
