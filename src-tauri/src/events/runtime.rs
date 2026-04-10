pub const RUNTIME_PING_UPDATED: &str = "runtime://ping";
pub const RUNTIME_SNAPSHOT_UPDATED: &str = "runtime://snapshot-updated";

pub fn runtime_event_names() -> [&'static str; 2] {
    [RUNTIME_SNAPSHOT_UPDATED, RUNTIME_PING_UPDATED]
}
