use std::sync::{Arc, Mutex};

/// Thread-safe SQLite wrapper for storing key-value entities.
///
/// RPC calls run on separate tokio tasks and all access the same `SqliteDb`.
/// `Arc` provides shared ownership across tasks, while `Mutex` ensures only one
/// task uses the `Connection` at a time (others wait). Without the `Mutex`,
/// concurrent SQL access on the same `Connection` would cause data races
/// (undefined behavior), since `Connection` is not `Sync`.
pub struct SqliteDb {
    conn: Mutex<rusqlite::Connection>,
}

impl SqliteDb {
    pub const DEFAULT_CONN_PATH: &str = "entity.db";

    pub fn open(path: &str) -> rusqlite::Result<Arc<Self>> {
        let conn = rusqlite::Connection::open(path)?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.setup_schema()?;
        Ok(Arc::new(db))
    }

    fn setup_schema(&self) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS entities (
                `key` TEXT PRIMARY KEY,
                `value` TEXT NOT NULL,
                created_at INTEGER NOT NULL
            )",
        )
    }

    pub fn save(&self, key: &str, value: &str) -> rusqlite::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO entities (`key`, `value`, created_at) VALUES (?1, ?2, ?3)",
            rusqlite::params![
                key,
                value,
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ],
        )?;
        Ok(())
    }

    pub fn delete(&self, key: &str) -> rusqlite::Result<bool> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute(
            "DELETE FROM entities WHERE `key` = ?1",
            rusqlite::params![key],
        )?;
        Ok(affected > 0)
    }

    pub fn get(&self, key: &str) -> rusqlite::Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT `value` FROM entities WHERE `key` = ?1")?;
        let mut rows = stmt.query(rusqlite::params![key])?;
        match rows.next()? {
            Some(row) => Ok(Some(row.get(0)?)),
            None => Ok(None),
        }
    }

    pub fn get_all(&self) -> rusqlite::Result<Vec<(String, String)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT `key`, `value` FROM entities")?;
        let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;
        rows.collect()
    }
}
