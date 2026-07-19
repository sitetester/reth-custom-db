use clap::Parser;
use eyre::eyre;

use crate::db::{DbType, SqliteDb};

/// `#[derive(Parser)]` auto-generates code to parse command-line arguments into our struct fields.
#[derive(Parser, Debug)]
pub struct DbExportCommand {
    #[arg(long, default_value_t = DbType::Sqlite)]
    pub db_type: DbType,

    /// `#[arg(long)]` would cause
    /// - `clap` to convert your Rust field name into kebab-case to create the command-line flag.
    /// - By omitting it, the argument becomes positional. The user would have to type the values in
    ///   a strict fields specific order
    #[arg(long)]
    pub conn_path: Option<String>,

    #[arg(long, default_value = "entities.json")]
    pub export_path: String,
}

impl DbExportCommand {
    pub(super) fn export(&self) -> eyre::Result<()> {
        let entities = match self.db_type {
            DbType::Sqlite => {
                let path = self
                    .conn_path
                    .as_deref()
                    .unwrap_or(SqliteDb::DEFAULT_CONN_PATH);
                let db_conn = SqliteDb::open(path)?;
                db_conn.get_all()?
            }
            DbType::Mdbx => {
                let path = self
                    .conn_path
                    .as_deref()
                    .ok_or_else(|| eyre!("--conn-path required for mdbx backend"))?;
                let db_conn = crate::db::MdbxDb::open(path)?;
                db_conn.read_all()?
            }
        };

        let json = serde_json::to_string(&entities)?;
        std::fs::write(&self.export_path, json)?; // overwrites existing file
        println!(
            "Exported {} entities to {}",
            entities.len(),
            self.export_path
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_sqlite_roundtrip() {
        let temp_dir = std::env::temp_dir();

        let db_path = temp_dir.join("test_export_sqlite.db");
        std::fs::remove_file(&db_path).ok();
        {
            let db_conn = SqliteDb::open(db_path.to_str().unwrap()).unwrap();
            db_conn.save("key1", "val1").unwrap();
            db_conn.save("key2", "val2").unwrap();
            // connection auto dropped here
        }

        let export_path = temp_dir.join("export_sqlite.json");
        let cmd = DbExportCommand {
            db_type: DbType::Sqlite,
            conn_path: Some(db_path.to_str().unwrap().into()),
            export_path: export_path.to_str().unwrap().into(),
        };
        cmd.export().unwrap();

        let json = std::fs::read_to_string(&export_path).unwrap();
        let entries: Vec<(String, String)> = serde_json::from_str(&json).unwrap();

        assert_eq!(entries.len(), 2);

        assert!(entries.contains(&("key1".into(), "val1".into())));
        assert!(entries.contains(&("key2".into(), "val2".into())));
    }

    #[test]
    fn test_export_mdbx_roundtrip() {
        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join("test_export_mdbx_db");
        std::fs::remove_dir_all(&db_path).ok();
        {
            let db_conn = crate::db::MdbxDb::open(db_path.to_str().unwrap()).unwrap();
            db_conn
                .write_all(&[("k1".into(), "v1".into()), ("k2".into(), "v2".into())])
                .unwrap();
        }

        let export_path = temp_dir.join("export_mdbx.json");
        let cmd = DbExportCommand {
            db_type: DbType::Mdbx,
            conn_path: Some(db_path.to_str().unwrap().into()),
            export_path: export_path.to_str().unwrap().into(),
        };
        cmd.export().unwrap();

        let json = std::fs::read_to_string(&export_path).unwrap();
        let entries: Vec<(String, String)> = serde_json::from_str(&json).unwrap();

        assert_eq!(entries.len(), 2);

        assert!(entries.contains(&("k1".into(), "v1".into())));
        assert!(entries.contains(&("k2".into(), "v2".into())));
    }

    #[test]
    fn test_export_mdbx_missing_conn_path() {
        let temp_dir = std::env::temp_dir();

        let cmd = DbExportCommand {
            db_type: DbType::Mdbx,
            conn_path: None,
            export_path: temp_dir.join("x.json").to_str().unwrap().into(),
        };
        let err = cmd.export().unwrap_err();
        assert!(err.to_string().contains("--conn-path required"));
    }
}
