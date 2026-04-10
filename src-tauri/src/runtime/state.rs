use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::runtime_snapshot::{RuntimeSnapshot, RuntimeStatus};

pub struct AppRuntimeState {
    snapshot: RwLock<RuntimeSnapshot>,
}

impl AppRuntimeState {
    pub fn new() -> Self {
        Self {
            snapshot: RwLock::new(RuntimeSnapshot::bootstrapping()),
        }
    }

    pub fn snapshot(&self) -> RuntimeSnapshot {
        self.snapshot
            .read()
            .expect("runtime snapshot read lock")
            .clone()
    }

    pub fn initialize(&self) -> RuntimeSnapshot {
        let mut snapshot = self
            .snapshot
            .write()
            .expect("runtime snapshot write lock");

        snapshot.app_ready = true;
        snapshot.runtime_status = RuntimeStatus::Ready;
        snapshot.active_profile = Some("desktop-default".to_string());
        snapshot.last_health_check = Some(current_health_marker());

        snapshot.clone()
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
}
