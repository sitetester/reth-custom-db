mod key;
mod table;
mod value;

pub use key::EntityKey;
pub use table::EntityTable;
pub use value::EntityValue;

use reth_ethereum::provider::db::mdbx::DatabaseEnv;

use table::EntityTableSet;

/// Ensures the `EntityTable` table exists in the MDBX database, creating it if
/// it doesn't already exist. Called during startup before the RPC server begins
/// serving requests.
pub fn ensure_entity_table(db: &mut DatabaseEnv) -> eyre::Result<()> {
    db.create_and_track_tables_for::<EntityTableSet>()?;
    Ok(())
}
