use std::path::Path;

use reth_ethereum::provider::db::mdbx::{DatabaseArguments, DatabaseEnv, DatabaseEnvKind};
use reth_ethereum::provider::db::{
    Database,
    cursor::DbCursorRO,
    transaction::{DbTx, DbTxMut},
};

use crate::db::ensure_entity_table;
use crate::db::{EntityKey, EntityTable, EntityValue};

/// DatabaseEnv is reth MDBX database handle
///
/// `DatabaseEnv` is `Sync`, so no `Mutex` is needed internally.
/// Most callers share the db via `Arc` across threads.
pub struct MdbxDb(DatabaseEnv);

impl MdbxDb {
    pub fn open(db_path: &str) -> eyre::Result<Self> {
        let path = Path::new(db_path);
        std::fs::create_dir_all(path)?;

        let mut db = DatabaseEnv::open(path, DatabaseEnvKind::RW, DatabaseArguments::default())?;
        ensure_entity_table(&mut db)?; // check EntityTable in reth_table/table.rs
        Ok(Self(db))
    }

    pub fn into_inner(self) -> DatabaseEnv {
        self.0
    }

    pub fn read_all(&self) -> eyre::Result<Vec<(String, String)>> {
        let tx = self.0.tx()?;
        let mut cursor = tx.cursor_read::<EntityTable>()?;

        let mut result: Vec<(String, String)> = Vec::new();
        for entry in cursor.walk(None)? {
            // None would start from the first entry,
            let (key, value) = entry?;
            result.push((
                String::from_utf8_lossy(&key.0).to_string(),
                String::from_utf8_lossy(&value.0).to_string(),
            ));
        }
        Ok(result)
    }

    pub fn write_all(&self, entities: &[(String, String)]) -> eyre::Result<()> {
        let tx = self.0.tx_mut()?;
        for (key, value) in entities {
            tx.put::<EntityTable>(
                EntityKey(key.as_bytes().to_vec()),
                EntityValue(value.as_bytes().to_vec()),
            )?;
        }
        tx.commit()?;
        Ok(())
    }
}
