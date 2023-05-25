use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use starknet::providers::jsonrpc::models::EventFilter;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde_as]
pub struct EventFilterWithPage {
    pub filter: EventFilter,
    pub page: ResultPageRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde_as]
pub struct ResultPageRequest {
    /// A pointer to the last element of the delivered page, use this token in a subsequent query to
    /// obtain the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    pub chunk_size: u64,
}
