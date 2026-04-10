pub mod commands;
pub mod events;
pub mod models;
pub mod runtime;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(runtime::state::AppRuntimeState::new())
        .invoke_handler(tauri::generate_handler![
            commands::runtime::get_runtime_snapshot,
            commands::runtime::initialize_runtime_state,
            commands::runtime::ping_runtime
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod runtime_command_contract_tests {
    use crate::commands::runtime::{
        initialize_runtime_state_for_test, ping_runtime_for_test,
    };
    use crate::models::runtime_snapshot::RuntimeStatus;
    use crate::runtime::state::AppRuntimeState;

    #[test]
    fn runtime_state_initialization_promotes_snapshot_to_ready() {
        let state = AppRuntimeState::new();

        let snapshot = initialize_runtime_state_for_test(&state);

        assert!(snapshot.app_ready);
        assert_eq!(snapshot.runtime_status, RuntimeStatus::Ready);
        assert_eq!(snapshot.active_profile.as_deref(), Some("desktop-default"));
        assert!(snapshot.last_health_check.is_some());
    }

    #[test]
    fn ping_runtime_reports_host_reachability() {
        let state = AppRuntimeState::new();

        let ping = ping_runtime_for_test(&state);

        assert!(ping.reachable);
        assert_eq!(ping.status, "ok");
    }
}
