use serde::{Deserialize, Serialize};

use crate::models::proxy_settings::ProxySettings;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProxyRuntimeStatus {
    Starting,
    Healthy,
    Stopped,
    Misconfigured,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxyRuntimeSnapshot {
    pub status: ProxyRuntimeStatus,
    pub effective_base_url: String,
    pub last_error: Option<String>,
    pub last_transition_at: Option<String>,
    pub settings: ProxySettings,
}

impl ProxyRuntimeSnapshot {
    pub fn new(
        status: ProxyRuntimeStatus,
        settings: ProxySettings,
        last_error: Option<String>,
        last_transition_at: Option<String>,
    ) -> Self {
        let settings = settings
            .normalized()
            .expect("proxy runtime snapshot should only store validated settings");
        let effective_base_url = settings
            .effective_base_url()
            .expect("validated proxy settings should build a local base url");

        Self {
            status,
            effective_base_url,
            last_error,
            last_transition_at,
            settings,
        }
    }
}

impl Default for ProxyRuntimeSnapshot {
    fn default() -> Self {
        Self::new(
            ProxyRuntimeStatus::Stopped,
            ProxySettings::default(),
            None,
            None,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{ProxyRuntimeSnapshot, ProxyRuntimeStatus};

    #[test]
    fn proxy_runtime_snapshot_defaults_to_stopped_loopback_url() {
        let snapshot = ProxyRuntimeSnapshot::default();

        assert_eq!(snapshot.status, ProxyRuntimeStatus::Stopped);
        assert_eq!(snapshot.effective_base_url, "http://127.0.0.1:8787/v1");
        assert_eq!(snapshot.last_error, None);
        assert_eq!(snapshot.last_transition_at, None);
        assert_eq!(snapshot.settings.listen_host, "127.0.0.1");
    }
}
