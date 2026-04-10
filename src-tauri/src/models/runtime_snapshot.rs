use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeStatus {
    Starting,
    Ready,
    Stopped,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeSnapshot {
    pub app_ready: bool,
    pub runtime_status: RuntimeStatus,
    pub active_profile: Option<String>,
    pub provider_slots: u32,
    pub account_slots: u32,
    pub last_health_check: Option<String>,
}

impl RuntimeSnapshot {
    pub fn bootstrapping() -> Self {
        Self {
            app_ready: false,
            runtime_status: RuntimeStatus::Starting,
            active_profile: None,
            provider_slots: 0,
            account_slots: 0,
            last_health_check: None,
        }
    }
}

impl Default for RuntimeSnapshot {
    fn default() -> Self {
        Self::bootstrapping()
    }
}

#[cfg(test)]
mod tests {
    use super::{RuntimeSnapshot, RuntimeStatus};

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
