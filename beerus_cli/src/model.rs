use clap::{Parser, Subcommand};
use ethers::types::U256;
use starknet::core::types::FieldElement;

use starknet::providers::jsonrpc::models::BlockHashAndNumber;
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
    QueryBlockTxCountByNumber {
        /// The block from which to query the txs count
        #[arg(short, long, value_name = "BLOCK")]
        block: u64,
    },
    QueryTxByHash {
        #[arg(short, long, value_name = "HASH")]
        hash: String,
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
    L1ToL2MessageNonce {},
    /// The nonce of the L1 to L2 message bridge
    QueryChainId {},
    QueryBlockNumber {},
    QueryBlockHashAndNumber {},
}

/// The response from a CLI command.
pub enum CommandResponse {
    EthereumQueryBalance(String),
    EthereumQueryNonce(u64),
    EthereumQueryBlockNumber(u64),
    EthereumQueryChainId(u64),
    EthereumQueryCode(Vec<u8>),
    EthereumQueryBlockTxCountByNumber(u64),
    EthereumQueryTxByHash(String),
    StarkNetQueryStateRoot(U256),
    StarkNetQueryContract(Vec<FieldElement>),
    StarkNetQueryGetStorageAt(FieldElement),
    StarkNetQueryNonce(FieldElement),
    StarknetQueryChainId(FieldElement),
    StarknetQueryBlockNumber(u64),
    StarknetQueryBlockHashAndNumber(BlockHashAndNumber),
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
            // Print the count of txs of a block
            // Result looks like: 150
            CommandResponse::EthereumQueryBlockTxCountByNumber(tx_count) => {
                write!(f, "{tx_count}")
            }
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
        }
    }
}
