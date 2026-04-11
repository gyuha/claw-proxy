pub const RUNTIME_PING_UPDATED: &str = "runtime://ping";
pub const PROXY_RUNTIME_UPDATED: &str = "runtime://proxy-updated";
pub const RUNTIME_SNAPSHOT_UPDATED: &str = "runtime://snapshot-updated";

pub fn runtime_event_names() -> [&'static str; 3] {
    [
        RUNTIME_SNAPSHOT_UPDATED,
        RUNTIME_PING_UPDATED,
        PROXY_RUNTIME_UPDATED,
    ]
}
