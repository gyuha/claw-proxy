#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use crate::models::proxy_runtime::ProxyRuntimeStatus;
    use crate::models::proxy_settings::ProxySettings;
    use crate::runtime::state::AppRuntimeState;
    use tempfile::tempdir;

    struct RecordingEventSink {
        events: Mutex<Vec<&'static str>>,
    }

    impl RecordingEventSink {
        fn new() -> Self {
            Self {
                events: Mutex::new(Vec::new()),
            }
        }

        fn names(&self) -> Vec<&'static str> {
            self.events.lock().expect("event list lock").clone()
        }
    }

    #[test]
    fn get_proxy_settings_returns_persisted_or_default_settings() {
        let tempdir = tempdir().expect("temp directory");
        let state = AppRuntimeState::new_in_directory(tempdir.path()).expect("state");
        let persisted = ProxySettings {
            listen_host: "127.0.0.1".to_string(),
            listen_port: available_loopback_port(),
            base_endpoint: "/persisted".to_string(),
        };

        state
            .save_proxy_settings(&persisted)
            .expect("persist proxy settings");

        let settings = get_proxy_settings_for_test(&state);

        assert_eq!(settings, persisted);
    }

    #[tokio::test]
    async fn start_and_stop_proxy_runtime_emit_proxy_invalidation() {
        let tempdir = tempdir().expect("temp directory");
        let state = AppRuntimeState::new_in_directory(tempdir.path()).expect("state");
        let events = RecordingEventSink::new();
        let persisted = ProxySettings {
            listen_host: "127.0.0.1".to_string(),
            listen_port: available_loopback_port(),
            base_endpoint: "/started".to_string(),
        };

        state
            .save_proxy_settings(&persisted)
            .expect("persist proxy settings");

        let started = start_proxy_runtime_for_test(&state, &events)
            .await
            .expect("start proxy runtime");
        let stopped = stop_proxy_runtime_for_test(&state, &events)
            .await
            .expect("stop proxy runtime");

        assert_eq!(started.status, ProxyRuntimeStatus::Healthy);
        assert_eq!(stopped.status, ProxyRuntimeStatus::Stopped);
        assert_eq!(
            events.names(),
            vec![PROXY_RUNTIME_UPDATED, PROXY_RUNTIME_UPDATED]
        );
    }

    #[tokio::test]
    async fn apply_proxy_settings_preserves_last_known_good_on_failure() {
        let tempdir = tempdir().expect("temp directory");
        let state = AppRuntimeState::new_in_directory(tempdir.path()).expect("state");
        let events = RecordingEventSink::new();
        let working = ProxySettings {
            listen_host: "127.0.0.1".to_string(),
            listen_port: available_loopback_port(),
            base_endpoint: "/healthy".to_string(),
        };

        let healthy = apply_proxy_settings_for_test(&state, working.clone(), &events)
            .await
            .expect("apply working settings");
        assert_eq!(healthy.status, ProxyRuntimeStatus::Healthy);
        assert_eq!(state.persisted_proxy_settings(), working);

        let occupied_listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("reserve an occupied loopback port");
        let occupied_port = occupied_listener
            .local_addr()
            .expect("occupied listener has local addr")
            .port();
        let candidate = ProxySettings {
            listen_port: occupied_port,
            ..working.clone()
        };

        let error = apply_proxy_settings_for_test(&state, candidate, &events)
            .await
            .expect_err("occupied candidate should fail");

        assert_eq!(error.code, "proxy_apply_failed");
        assert_eq!(state.persisted_proxy_settings(), working);
        assert_eq!(state.last_known_good_proxy_settings(), working);
        assert_eq!(state.proxy_snapshot().settings, working);
        assert_eq!(state.proxy_snapshot().status, ProxyRuntimeStatus::Misconfigured);
        assert!(state.proxy_runtime_is_running().await);
        assert_eq!(events.names().last().copied(), Some(PROXY_RUNTIME_UPDATED));
    }

    fn available_loopback_port() -> u16 {
        std::net::TcpListener::bind("127.0.0.1:0")
            .expect("bind temporary loopback listener")
            .local_addr()
            .expect("temporary listener has local addr")
            .port()
    }
}
