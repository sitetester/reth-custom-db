use clap::Parser;
use eyre::eyre;

use crate::db::{DbType, SqliteDb};

/// Imports SQLite or MDBX db from a JSON file
#[derive(Parser, Debug)]
pub struct DbImportCommand {
    #[arg(long, default_value_t = DbType::Sqlite)]
    pub db_type: DbType,

    #[arg(long)]
    pub conn_path: Option<String>,

    #[arg(long, default_value = "entities.json")]
    pub import_path: String,
}

impl DbImportCommand {
    pub(super) fn import(&self) -> eyre::Result<()> {
        let json = std::fs::read_to_string(&self.import_path)?;
        let entities: Vec<(String, String)> = serde_json::from_str(&json)?;

        match self.db_type {
            DbType::Sqlite => {
                let path = self
                    .conn_path
                    .as_deref()
                    .unwrap_or(SqliteDb::DEFAULT_CONN_PATH);
                let db_conn = SqliteDb::open(path).map_err(|e| eyre!("{e}"))?;
                for (key, value) in &entities {
                    db_conn.save(key, value).map_err(|e| eyre!("{e}"))?;
                }
            }
            DbType::Mdbx => {
                let path = self
                    .conn_path
                    .as_deref()
                    .ok_or_else(|| eyre!("--conn-path required for mdbx backend"))?;
                let db_conn = crate::db::MdbxDb::open(path)?;
                db_conn.write_all(&entities)?;
            }
        }
        println!(
            "Imported {} entities from {}",
            entities.len(),
            self.import_path
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_sqlite_roundtrip() {
        let dir = std::env::temp_dir();
        let db_path = dir.join("test_import_sqlite.db");
        let import_path = dir.join("import_sqlite.json");
        std::fs::remove_file(&db_path).ok();

        let entries = vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "2".to_string()),
        ];
        std::fs::write(&import_path, serde_json::to_string(&entries).unwrap()).unwrap();

        let cmd = DbImportCommand {
            db_type: DbType::Sqlite,
            conn_path: Some(db_path.to_str().unwrap().into()),
            import_path: import_path.to_str().unwrap().into(),
        };
        cmd.import().unwrap();

        let db_conn = SqliteDb::open(db_path.to_str().unwrap()).unwrap();
        assert_eq!(db_conn.get("a").unwrap(), Some("1".into()));
        assert_eq!(db_conn.get("b").unwrap(), Some("2".into()));
    }

    #[test]
    fn test_import_mdbx_roundtrip() {
        let dir = std::env::temp_dir();
        let db_path = dir.join("test_import_mdbx_db");
        let import_path = dir.join("import_mdbx.json");

        let entries = vec![
            ("x".to_string(), "10".to_string()),
            ("y".to_string(), "20".to_string()),
        ];
        std::fs::write(&import_path, serde_json::to_string(&entries).unwrap()).unwrap();

        let cmd = DbImportCommand {
            db_type: DbType::Mdbx,
            conn_path: Some(db_path.to_str().unwrap().into()),
            import_path: import_path.to_str().unwrap().into(),
        };
        cmd.import().unwrap();

        let db_conn = crate::db::MdbxDb::open(db_path.to_str().unwrap()).unwrap();
        let results = db_conn.read_all().unwrap();
        assert_eq!(results.len(), 2);
        assert!(results.contains(&("x".into(), "10".into())));
        assert!(results.contains(&("y".into(), "20".into())));
    }
}
