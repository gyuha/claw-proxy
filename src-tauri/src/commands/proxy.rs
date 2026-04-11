use std::fmt;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Runtime, State};

use crate::events::runtime::PROXY_RUNTIME_UPDATED;
use crate::models::proxy_runtime::ProxyRuntimeSnapshot;
use crate::models::proxy_settings::ProxySettings;
use crate::runtime::state::AppRuntimeState;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ProxyCommandError {
    pub code: &'static str,
    pub message: String,
}

impl ProxyCommandError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for ProxyCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ProxyCommandError {}

trait ProxyRuntimeEventEmitter {
    fn emit_proxy_runtime_updated(&self) -> Result<(), ProxyCommandError>;
}

impl<R: Runtime> ProxyRuntimeEventEmitter for AppHandle<R> {
    fn emit_proxy_runtime_updated(&self) -> Result<(), ProxyCommandError> {
        self.emit(PROXY_RUNTIME_UPDATED, ())
            .map_err(|error| ProxyCommandError::new("proxy_event_emit_failed", error.to_string()))
    }
}

fn get_proxy_runtime_snapshot_for_test(state: &AppRuntimeState) -> ProxyRuntimeSnapshot {
    state.proxy_snapshot()
}

fn get_proxy_settings_for_test(state: &AppRuntimeState) -> ProxySettings {
    state.persisted_proxy_settings()
}

async fn start_proxy_runtime_for_test<E: ProxyRuntimeEventEmitter>(
    state: &AppRuntimeState,
    emitter: &E,
) -> Result<ProxyRuntimeSnapshot, ProxyCommandError> {
    let snapshot = state.start_proxy_runtime(state.persisted_proxy_settings()).await;
    emitter.emit_proxy_runtime_updated()?;

    Ok(snapshot)
}

async fn stop_proxy_runtime_for_test<E: ProxyRuntimeEventEmitter>(
    state: &AppRuntimeState,
    emitter: &E,
) -> Result<ProxyRuntimeSnapshot, ProxyCommandError> {
    let snapshot = state.stop_proxy_runtime().await;
    emitter.emit_proxy_runtime_updated()?;

    Ok(snapshot)
}

async fn apply_proxy_settings_for_test<E: ProxyRuntimeEventEmitter>(
    state: &AppRuntimeState,
    settings: ProxySettings,
    emitter: &E,
) -> Result<ProxyRuntimeSnapshot, ProxyCommandError> {
    let previous_persisted = state.persisted_proxy_settings();
    let previous_good = state.last_known_good_proxy_settings();
    let was_running = state.proxy_runtime_is_running().await;
    let candidate = match settings.normalized() {
        Ok(candidate) => candidate,
        Err(error) => {
            let command_error =
                ProxyCommandError::new("proxy_apply_failed", error.to_string());
            state.mark_proxy_misconfigured(previous_good.clone(), command_error.message.clone());
            emitter.emit_proxy_runtime_updated()?;
            return Err(command_error);
        }
    };

    let started = state.start_proxy_runtime(candidate.clone()).await;
    if started.status == crate::models::proxy_runtime::ProxyRuntimeStatus::Healthy {
        if let Err(error) = state.save_proxy_settings(&candidate) {
            let command_error = rollback_after_failed_apply(
                state,
                was_running,
                &previous_good,
                &previous_persisted,
                &format!("failed to persist proxy settings: {error}"),
            )
            .await;
            emitter.emit_proxy_runtime_updated()?;
            return Err(command_error);
        }
        emitter.emit_proxy_runtime_updated()?;
        return Ok(started);
    }

    let last_error = started
        .last_error
        .clone()
        .unwrap_or_else(|| "failed to apply proxy settings".to_string());
    rollback_after_failed_apply(
        state,
        was_running,
        &previous_good,
        &previous_persisted,
        &last_error,
    )
    .await;
    emitter.emit_proxy_runtime_updated()?;

    Err(ProxyCommandError::new("proxy_apply_failed", last_error))
}

async fn rollback_after_failed_apply(
    state: &AppRuntimeState,
    was_running: bool,
    previous_good: &ProxySettings,
    previous_persisted: &ProxySettings,
    message: &str,
) -> ProxyCommandError {
    if was_running {
        let recovered = state.start_proxy_runtime(previous_good.clone()).await;
        if recovered.status == crate::models::proxy_runtime::ProxyRuntimeStatus::Healthy {
            state.mark_proxy_misconfigured(previous_good.clone(), message);
            return ProxyCommandError::new("proxy_apply_failed", message);
        }
    } else {
        let _ = state.stop_proxy_runtime().await;
    }

    state.mark_proxy_misconfigured(previous_persisted.clone(), message);
    ProxyCommandError::new("proxy_apply_failed", message)
}

#[tauri::command]
pub fn get_proxy_runtime_snapshot(state: State<'_, AppRuntimeState>) -> ProxyRuntimeSnapshot {
    get_proxy_runtime_snapshot_for_test(state.inner())
}

#[tauri::command]
pub fn get_proxy_settings(state: State<'_, AppRuntimeState>) -> ProxySettings {
    get_proxy_settings_for_test(state.inner())
}

#[tauri::command]
pub async fn start_proxy_runtime(
    app: AppHandle,
    state: State<'_, AppRuntimeState>,
) -> Result<ProxyRuntimeSnapshot, ProxyCommandError> {
    start_proxy_runtime_for_test(state.inner(), &app).await
}

#[tauri::command]
pub async fn stop_proxy_runtime(
    app: AppHandle,
    state: State<'_, AppRuntimeState>,
) -> Result<ProxyRuntimeSnapshot, ProxyCommandError> {
    stop_proxy_runtime_for_test(state.inner(), &app).await
}

#[tauri::command]
pub async fn apply_proxy_settings(
    app: AppHandle,
    state: State<'_, AppRuntimeState>,
    settings: ProxySettings,
) -> Result<ProxyRuntimeSnapshot, ProxyCommandError> {
    apply_proxy_settings_for_test(state.inner(), settings, &app).await
}

#[cfg(test)]
mod tests {
    use super::{
        apply_proxy_settings_for_test, get_proxy_settings_for_test, start_proxy_runtime_for_test,
        stop_proxy_runtime_for_test, ProxyRuntimeEventEmitter, PROXY_RUNTIME_UPDATED,
    };
    use std::sync::Mutex;

    use crate::models::proxy_runtime::ProxyRuntimeStatus;
    use crate::models::proxy_settings::ProxySettings;
    use crate::runtime::state::AppRuntimeState;
    use tempfile::tempdir;

    struct RecordingEventSink {
        events: Mutex<Vec<&'static str>>,
    }

    impl RecordingEventSink {
        fn new() -> Self {
            Self {
                events: Mutex::new(Vec::new()),
            }
        }

        fn names(&self) -> Vec<&'static str> {
            self.events.lock().expect("event list lock").clone()
        }
    }

    impl ProxyRuntimeEventEmitter for RecordingEventSink {
        fn emit_proxy_runtime_updated(&self) -> Result<(), super::ProxyCommandError> {
            self.events
                .lock()
                .expect("event list lock")
                .push(PROXY_RUNTIME_UPDATED);
            Ok(())
        }
    }

    #[test]
    fn get_proxy_settings_returns_persisted_or_default_settings() {
        let tempdir = tempdir().expect("temp directory");
        let state = AppRuntimeState::new_in_directory(tempdir.path()).expect("state");
        let persisted = ProxySettings {
            listen_host: "127.0.0.1".to_string(),
            listen_port: available_loopback_port(),
            base_endpoint: "/persisted".to_string(),
        };

        state
            .save_proxy_settings(&persisted)
            .expect("persist proxy settings");

        let settings = get_proxy_settings_for_test(&state);

        assert_eq!(settings, persisted);
    }

    #[tokio::test]
    async fn start_and_stop_proxy_runtime_emit_proxy_invalidation() {
        let tempdir = tempdir().expect("temp directory");
        let state = AppRuntimeState::new_in_directory(tempdir.path()).expect("state");
        let events = RecordingEventSink::new();
        let persisted = ProxySettings {
            listen_host: "127.0.0.1".to_string(),
            listen_port: available_loopback_port(),
            base_endpoint: "/started".to_string(),
        };

        state
            .save_proxy_settings(&persisted)
            .expect("persist proxy settings");

        let started = start_proxy_runtime_for_test(&state, &events)
            .await
            .expect("start proxy runtime");
        let stopped = stop_proxy_runtime_for_test(&state, &events)
            .await
            .expect("stop proxy runtime");

        assert_eq!(started.status, ProxyRuntimeStatus::Healthy);
        assert_eq!(stopped.status, ProxyRuntimeStatus::Stopped);
        assert_eq!(
            events.names(),
            vec![PROXY_RUNTIME_UPDATED, PROXY_RUNTIME_UPDATED]
        );
    }

    #[tokio::test]
    async fn apply_proxy_settings_preserves_last_known_good_on_failure() {
        let tempdir = tempdir().expect("temp directory");
        let state = AppRuntimeState::new_in_directory(tempdir.path()).expect("state");
        let events = RecordingEventSink::new();
        let working = ProxySettings {
            listen_host: "127.0.0.1".to_string(),
            listen_port: available_loopback_port(),
            base_endpoint: "/healthy".to_string(),
        };

        let healthy = apply_proxy_settings_for_test(&state, working.clone(), &events)
            .await
            .expect("apply working settings");
        assert_eq!(healthy.status, ProxyRuntimeStatus::Healthy);
        assert_eq!(state.persisted_proxy_settings(), working);

        let occupied_listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("reserve an occupied loopback port");
        let occupied_port = occupied_listener
            .local_addr()
            .expect("occupied listener has local addr")
            .port();
        let candidate = ProxySettings {
            listen_port: occupied_port,
            ..working.clone()
        };

        let error = apply_proxy_settings_for_test(&state, candidate, &events)
            .await
            .expect_err("occupied candidate should fail");

        assert_eq!(error.code, "proxy_apply_failed");
        assert_eq!(state.persisted_proxy_settings(), working);
        assert_eq!(state.last_known_good_proxy_settings(), working);
        assert_eq!(state.proxy_snapshot().settings, working);
        assert_eq!(state.proxy_snapshot().status, ProxyRuntimeStatus::Misconfigured);
        assert!(state.proxy_runtime_is_running().await);
        assert_eq!(events.names().last().copied(), Some(PROXY_RUNTIME_UPDATED));
    }

    fn available_loopback_port() -> u16 {
        std::net::TcpListener::bind("127.0.0.1:0")
            .expect("bind temporary loopback listener")
            .local_addr()
            .expect("temporary listener has local addr")
            .port()
    }
}
