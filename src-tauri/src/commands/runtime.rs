use serde::Serialize;
use tauri::State;

use crate::models::runtime_snapshot::{RuntimeSnapshot, RuntimeStatus};
use crate::runtime::state::AppRuntimeState;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RuntimePingResponse {
    pub reachable: bool,
    pub runtime_status: RuntimeStatus,
    pub status: &'static str,
}

pub fn initialize_runtime_state_for_test(state: &AppRuntimeState) -> RuntimeSnapshot {
    state.initialize()
}

pub fn ping_runtime_for_test(state: &AppRuntimeState) -> RuntimePingResponse {
    RuntimePingResponse {
        reachable: true,
        runtime_status: state.snapshot().runtime_status,
        status: "ok",
    }
}

#[tauri::command]
pub fn get_runtime_snapshot(state: State<'_, AppRuntimeState>) -> RuntimeSnapshot {
    state.snapshot()
}

#[tauri::command]
pub fn initialize_runtime_state(state: State<'_, AppRuntimeState>) -> RuntimeSnapshot {
    initialize_runtime_state_for_test(state.inner())
}

#[tauri::command]
pub fn ping_runtime(state: State<'_, AppRuntimeState>) -> RuntimePingResponse {
    ping_runtime_for_test(state.inner())
}

#[cfg(test)]
mod tests {
    use super::ping_runtime_for_test;
    use crate::runtime::state::AppRuntimeState;

    #[test]
    fn ping_runtime_reports_host_reachability() {
        let state = AppRuntimeState::new();

        let ping = ping_runtime_for_test(&state);

        assert!(ping.reachable);
        assert_eq!(ping.status, "ok");
    }
}
