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
