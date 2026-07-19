mod events;
mod mdbx;
mod reth_table;
mod sqlite;

pub use events::{BlockEvent, EntityEvent, EventType};
pub use mdbx::MdbxDb;
pub use reth_table::{EntityKey, EntityTable, EntityValue, ensure_entity_table};
pub use sqlite::SqliteDb;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum DbType {
    Sqlite,
    Mdbx,
}

impl std::fmt::Display for DbType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sqlite => write!(f, "sqlite"),
            Self::Mdbx => write!(f, "mdbx"),
        }
    }
}
