#[cfg(test)]
mod tests {
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

        persistence
            .save_settings(&settings)
            .expect("save settings");

        let db = rusqlite::Connection::open(tempdir.path().join("proxy-settings.sqlite"))
            .expect("open database");
        let (id, host, port, endpoint, updated_at): (i64, String, i64, String, String) = db
            .query_row(
                "SELECT id, listen_host, listen_port, base_endpoint, updated_at FROM proxy_settings",
                [],
                |row| {
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

        persistence
            .save_settings(&settings)
            .expect("save settings");

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
