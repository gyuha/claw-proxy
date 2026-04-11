use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::proxy_runtime::{ProxyRuntimeSnapshot, ProxyRuntimeStatus};
use crate::models::proxy_settings::ProxySettings;
use crate::models::runtime_snapshot::{RuntimeSnapshot, RuntimeStatus};
use crate::runtime::proxy_runtime::ProxyRuntimeSupervisor;

pub struct AppRuntimeState {
    app_snapshot: RwLock<RuntimeSnapshot>,
    proxy_snapshot: RwLock<ProxyRuntimeSnapshot>,
    last_known_good: RwLock<ProxySettings>,
    proxy_supervisor: ProxyRuntimeSupervisor,
}

impl AppRuntimeState {
    pub fn new() -> Self {
        Self {
            app_snapshot: RwLock::new(RuntimeSnapshot::bootstrapping()),
            proxy_snapshot: RwLock::new(ProxyRuntimeSnapshot::default()),
            last_known_good: RwLock::new(ProxySettings::default()),
            proxy_supervisor: ProxyRuntimeSupervisor::new(),
        }
    }

    pub fn snapshot(&self) -> RuntimeSnapshot {
        self.app_snapshot
            .read()
            .expect("runtime snapshot read lock")
            .clone()
    }

    pub fn initialize(&self) -> RuntimeSnapshot {
        let mut snapshot = self
            .app_snapshot
            .write()
            .expect("runtime snapshot write lock");

        snapshot.app_ready = true;
        snapshot.runtime_status = RuntimeStatus::Ready;
        snapshot.active_profile = Some("desktop-default".to_string());
        snapshot.last_health_check = Some(current_health_marker());

        snapshot.clone()
    }

    pub fn proxy_snapshot(&self) -> ProxyRuntimeSnapshot {
        self.proxy_snapshot
            .read()
            .expect("proxy runtime snapshot read lock")
            .clone()
    }

    pub fn last_known_good_proxy_settings(&self) -> ProxySettings {
        self.last_known_good
            .read()
            .expect("proxy settings read lock")
            .clone()
    }

    pub async fn start_proxy_runtime(&self, settings: ProxySettings) -> ProxyRuntimeSnapshot {
        let _ = self.proxy_supervisor.stop().await;

        let fallback_settings = self.proxy_snapshot().settings;
        let normalized = match settings.normalized() {
            Ok(normalized) => normalized,
            Err(error) => {
                let snapshot = ProxyRuntimeSnapshot::new(
                    ProxyRuntimeStatus::Misconfigured,
                    fallback_settings,
                    Some(error.to_string()),
                    Some(current_health_marker()),
                );
                self.store_proxy_snapshot(snapshot.clone());
                return snapshot;
            }
        };

        self.store_proxy_snapshot(ProxyRuntimeSnapshot::new(
            ProxyRuntimeStatus::Starting,
            normalized.clone(),
            None,
            Some(current_health_marker()),
        ));

        match self.proxy_supervisor.start(normalized.clone()).await {
            Ok(()) => {
                let snapshot = ProxyRuntimeSnapshot::new(
                    ProxyRuntimeStatus::Healthy,
                    normalized.clone(),
                    None,
                    Some(current_health_marker()),
                );
                self.store_proxy_snapshot(snapshot.clone());
                self.store_last_known_good(normalized);
                snapshot
            }
            Err(error) => {
                let snapshot = ProxyRuntimeSnapshot::new(
                    ProxyRuntimeStatus::Misconfigured,
                    normalized,
                    Some(error.to_string()),
                    Some(current_health_marker()),
                );
                self.store_proxy_snapshot(snapshot.clone());
                snapshot
            }
        }
    }

    pub async fn stop_proxy_runtime(&self) -> ProxyRuntimeSnapshot {
        let _ = self.proxy_supervisor.stop().await;
        let settings = self.proxy_snapshot().settings;
        let snapshot = ProxyRuntimeSnapshot::new(
            ProxyRuntimeStatus::Stopped,
            settings,
            None,
            Some(current_health_marker()),
        );
        self.store_proxy_snapshot(snapshot.clone());
        snapshot
    }

    pub async fn proxy_runtime_is_running(&self) -> bool {
        self.proxy_supervisor.is_running().await
    }

    fn store_proxy_snapshot(&self, snapshot: ProxyRuntimeSnapshot) {
        let mut guard = self
            .proxy_snapshot
            .write()
            .expect("proxy runtime snapshot write lock");
        *guard = snapshot;
    }

    fn store_last_known_good(&self, settings: ProxySettings) {
        let mut guard = self
            .last_known_good
            .write()
            .expect("proxy settings write lock");
        *guard = settings;
    }
}

impl Default for AppRuntimeState {
    fn default() -> Self {
        Self::new()
    }
}

fn current_health_marker() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_secs()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::AppRuntimeState;
    use crate::models::runtime_snapshot::RuntimeStatus;

    #[test]
    fn runtime_state_initialization_promotes_snapshot_to_ready() {
        let state = AppRuntimeState::new();

        let snapshot = state.initialize();

        assert!(snapshot.app_ready);
        assert_eq!(snapshot.runtime_status, RuntimeStatus::Ready);
        assert_eq!(snapshot.active_profile.as_deref(), Some("desktop-default"));
        assert!(snapshot.last_health_check.is_some());
    }

    #[test]
    fn proxy_runtime_state_defaults_to_loopback_settings() {
        let state = AppRuntimeState::new();

        let snapshot = state.proxy_snapshot();

        assert_eq!(snapshot.status, crate::models::proxy_runtime::ProxyRuntimeStatus::Stopped);
        assert_eq!(snapshot.effective_base_url, "http://127.0.0.1:8787/v1");
        assert_eq!(
            state.last_known_good_proxy_settings(),
            crate::models::proxy_settings::ProxySettings::default()
        );
    }
}
