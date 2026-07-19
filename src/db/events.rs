use serde::{Deserialize, Serialize};

/// The type of entity event.
/// Discriminants: Saved = 0, Deleted = 1.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Saved,
    Deleted,
}

/// Emitted when an entity is created, updated, or deleted.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityEvent {
    pub event: EventType,
    pub key: String,
    pub value: Option<String>,
}

/// Emitted when a new block is received and processed by the node.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockEvent {
    pub block_number: u64,
    pub block_hash: String,
    pub parent_hash: String,
    pub timestamp: u64,
    pub tx_count: usize,
}
