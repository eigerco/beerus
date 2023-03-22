use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet::core::serde::unsigned_field_element::UfeHex;
use starknet::providers::jsonrpc::models::BlockId as StarknetBlockId;
use starknet::providers::jsonrpc::models::EventFilter as StarknetEventFilter;
use starknet::{core::types::FieldElement, providers::jsonrpc::models::BlockTag};

// Copied from starknet-rs because it doesn't implement `Deserialize`
// TODO: should be removed after making the changes in starknet-rs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockId {
    Hash(FieldElement),
    Number(u64),
    Tag(BlockTag),
}

// Copied from starknet-rs because it doesn't implement `Deserialize`
// TODO: should be removed after making the changes in starknet-rs
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFilter {
    /// From block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_block: Option<BlockId>,
    /// To block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_block: Option<BlockId>,
    /// From contract
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<UfeHex>")]
    pub address: Option<FieldElement>,
    /// Filter key values
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<Vec<UfeHex>>")]
    pub keys: Option<Vec<FieldElement>>,
}

impl EventFilter {
    pub fn to_starknet_event_filter(self) -> StarknetEventFilter {
        StarknetEventFilter {
            address: self.address,
            from_block: self.from_block.map(BlockId::to_starknet_block_id),
            to_block: self.to_block.map(BlockId::to_starknet_block_id),
            keys: self.keys,
        }
    }
}

impl BlockId {
    pub fn to_starknet_block_id(self) -> StarknetBlockId {
        match self {
            BlockId::Hash(h) => StarknetBlockId::Hash(h),
            BlockId::Number(n) => StarknetBlockId::Number(n),
            BlockId::Tag(t) => StarknetBlockId::Tag(t),
        }
    }
}
