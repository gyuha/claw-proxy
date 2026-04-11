use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection, OptionalExtension};
use rusqlite_migration::{Migrations, M};

use crate::models::proxy_settings::ProxySettings;

const DATABASE_FILE_NAME: &str = "proxy-settings.sqlite";

pub struct ProxySettingsPersistence {
    database_path: DatabasePath,
}

enum DatabasePath {
    InMemory,
    OnDisk(PathBuf),
}

impl ProxySettingsPersistence {
    pub fn new(config_dir: impl AsRef<Path>) -> Result<Self, ProxyPersistenceError> {
        Ok(Self {
            database_path: DatabasePath::OnDisk(config_dir.as_ref().join(DATABASE_FILE_NAME)),
        })
    }

    pub fn in_memory() -> Result<Self, ProxyPersistenceError> {
        Ok(Self {
            database_path: DatabasePath::InMemory,
        })
    }

    pub fn load_settings(&self) -> Result<ProxySettings, ProxyPersistenceError> {
        let connection = self.open_connection()?;
        let stored = connection
            .query_row(
                "SELECT listen_host, listen_port, base_endpoint FROM proxy_settings WHERE id = 1",
                [],
                |row| {
                    Ok(ProxySettings {
                        listen_host: row.get(0)?,
                        listen_port: row.get(1)?,
                        base_endpoint: row.get(2)?,
                    })
                },
            )
            .optional()?;

        stored
            .map(|settings| settings.normalized())
            .transpose()
            .map_err(ProxyPersistenceError::from)?
            .map_or_else(|| Ok(ProxySettings::default()), Ok)
    }

    pub fn save_settings(
        &self,
        settings: &ProxySettings,
    ) -> Result<ProxySettings, ProxyPersistenceError> {
        let normalized = settings.normalized()?;
        let connection = self.open_connection()?;
        connection.execute(
            "INSERT INTO proxy_settings (id, listen_host, listen_port, base_endpoint, updated_at)
             VALUES (1, ?1, ?2, ?3, ?4)
             ON CONFLICT(id) DO UPDATE SET
               listen_host = excluded.listen_host,
               listen_port = excluded.listen_port,
               base_endpoint = excluded.base_endpoint,
               updated_at = excluded.updated_at",
            params![
                normalized.listen_host,
                normalized.listen_port,
                normalized.base_endpoint,
                current_updated_at()
            ],
        )?;

        Ok(normalized)
    }

    fn open_connection(&self) -> Result<Connection, ProxyPersistenceError> {
        let mut connection = match &self.database_path {
            DatabasePath::InMemory => Connection::open_in_memory()?,
            DatabasePath::OnDisk(path) => {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }
                Connection::open(path)?
            }
        };
        migrations().to_latest(&mut connection)?;

        Ok(connection)
    }
}

#[derive(Debug)]
pub enum ProxyPersistenceError {
    Io(std::io::Error),
    Sql(rusqlite::Error),
    Migration(rusqlite_migration::Error),
    Settings(crate::models::proxy_settings::ProxySettingsValidationError),
}

impl std::fmt::Display for ProxyPersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "failed to prepare proxy settings storage: {error}"),
            Self::Sql(error) => write!(f, "failed to access proxy settings database: {error}"),
            Self::Migration(error) => {
                write!(f, "failed to migrate proxy settings database: {error}")
            }
            Self::Settings(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for ProxyPersistenceError {}

impl From<std::io::Error> for ProxyPersistenceError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<rusqlite::Error> for ProxyPersistenceError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sql(value)
    }
}

impl From<rusqlite_migration::Error> for ProxyPersistenceError {
    fn from(value: rusqlite_migration::Error) -> Self {
        Self::Migration(value)
    }
}

impl From<crate::models::proxy_settings::ProxySettingsValidationError> for ProxyPersistenceError {
    fn from(value: crate::models::proxy_settings::ProxySettingsValidationError) -> Self {
        Self::Settings(value)
    }
}

fn migrations() -> Migrations<'static> {
    Migrations::new(vec![M::up(
        "CREATE TABLE proxy_settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            listen_host TEXT NOT NULL,
            listen_port INTEGER NOT NULL,
            base_endpoint TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
    )])
}

fn current_updated_at() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_secs()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::ProxySettingsPersistence;
    use crate::models::proxy_settings::ProxySettings;
    use tempfile::tempdir;

    #[test]
    fn proxy_persistence_first_load_bootstraps_schema_and_returns_defaults() {
        let tempdir = tempdir().expect("temp directory");
        let persistence =
            ProxySettingsPersistence::new(tempdir.path()).expect("create persistence");

        let settings = persistence.load_settings().expect("load settings");

        assert_eq!(settings, ProxySettings::default());
    }

    #[test]
    fn proxy_persistence_save_writes_single_row_settings_record() {
        let tempdir = tempdir().expect("temp directory");
        let persistence =
            ProxySettingsPersistence::new(tempdir.path()).expect("create persistence");
        let settings = ProxySettings {
            listen_host: "localhost".to_string(),
            listen_port: 9898,
            base_endpoint: "/proxy".to_string(),
        };

        persistence.save_settings(&settings).expect("save settings");

        let db = rusqlite::Connection::open(tempdir.path().join("proxy-settings.sqlite"))
            .expect("open database");
        let (id, host, port, endpoint, updated_at): (i64, String, i64, String, String) = db
            .query_row(
                "SELECT id, listen_host, listen_port, base_endpoint, updated_at FROM proxy_settings",
                [],
                |row: &rusqlite::Row<'_>| {
                    Ok((
                        row.get(0)?,
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        row.get(4)?,
                    ))
                },
            )
            .expect("read saved row");

        assert_eq!(id, 1);
        assert_eq!(host, "localhost");
        assert_eq!(port, 9898);
        assert_eq!(endpoint, "/proxy");
        assert!(!updated_at.is_empty());
    }

    #[test]
    fn proxy_persistence_load_after_save_returns_same_validated_settings() {
        let tempdir = tempdir().expect("temp directory");
        let persistence =
            ProxySettingsPersistence::new(tempdir.path()).expect("create persistence");
        let settings = ProxySettings {
            listen_host: "127.0.0.1".to_string(),
            listen_port: 8788,
            base_endpoint: "nested/path".to_string(),
        };

        persistence.save_settings(&settings).expect("save settings");

        let loaded = persistence.load_settings().expect("reload settings");

        assert_eq!(
            loaded,
            ProxySettings {
                listen_host: "127.0.0.1".to_string(),
                listen_port: 8788,
                base_endpoint: "/nested/path".to_string(),
            }
        );
    }
}
