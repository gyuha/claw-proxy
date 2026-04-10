#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use crate::models::runtime_snapshot::{RuntimeSnapshot, RuntimeStatus};

    #[test]
    fn runtime_snapshot_serializes_stable_fields() {
        let snapshot = RuntimeSnapshot {
            app_ready: true,
            runtime_status: RuntimeStatus::Ready,
            active_profile: Some("desktop-default".to_string()),
            provider_slots: 2,
            account_slots: 4,
            last_health_check: Some("1712793600".to_string()),
        };

        let serialized = serde_json::to_value(snapshot).expect("runtime snapshot serializes");

        assert_eq!(serialized["app_ready"], true);
        assert_eq!(serialized["runtime_status"], "ready");
        assert_eq!(serialized["active_profile"], "desktop-default");
        assert_eq!(serialized["provider_slots"], 2);
        assert_eq!(serialized["account_slots"], 4);
        assert_eq!(serialized["last_health_check"], "1712793600");
    }
}
