use claw_proxy_lib::models::proxy_runtime::ProxyRuntimeStatus;
use claw_proxy_lib::models::proxy_settings::ProxySettings;
use claw_proxy_lib::runtime::state::AppRuntimeState;

#[tokio::test]
async fn proxy_runtime_start_produces_healthy_snapshot() {
    let state = AppRuntimeState::new();
    let settings = ProxySettings {
        listen_port: available_loopback_port(),
        ..ProxySettings::default()
    };

    let snapshot = state.start_proxy_runtime(settings.clone()).await;

    assert_eq!(snapshot.status, ProxyRuntimeStatus::Healthy);
    assert_eq!(
        snapshot.effective_base_url,
        settings
            .effective_base_url()
            .expect("default settings build a local base url")
    );

    let stopped = state.stop_proxy_runtime().await;
    assert_eq!(stopped.status, ProxyRuntimeStatus::Stopped);
}

#[tokio::test]
async fn proxy_runtime_stop_releases_the_running_task_cleanly() {
    let state = AppRuntimeState::new();

    state
        .start_proxy_runtime(ProxySettings {
            listen_port: available_loopback_port(),
            ..ProxySettings::default()
        })
        .await;
    assert!(state.proxy_runtime_is_running().await);

    let stopped = state.stop_proxy_runtime().await;

    assert_eq!(stopped.status, ProxyRuntimeStatus::Stopped);
    assert!(!state.proxy_runtime_is_running().await);
}

#[tokio::test]
async fn proxy_runtime_failed_restart_preserves_last_known_good_settings() {
    let state = AppRuntimeState::new();
    let healthy_settings = ProxySettings {
        listen_port: available_loopback_port(),
        ..ProxySettings::default()
    };

    let healthy = state.start_proxy_runtime(healthy_settings.clone()).await;
    assert_eq!(healthy.status, ProxyRuntimeStatus::Healthy);

    let occupied_listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("reserve an occupied loopback port");
    let occupied_port = occupied_listener
        .local_addr()
        .expect("occupied listener has local addr")
        .port();

    let failed = state
        .start_proxy_runtime(ProxySettings {
            listen_port: occupied_port,
            ..healthy_settings.clone()
        })
        .await;

    assert_eq!(failed.status, ProxyRuntimeStatus::Misconfigured);
    assert!(
        failed
            .last_error
            .as_deref()
            .is_some_and(|error| !error.trim().is_empty())
    );
    assert_eq!(
        state.last_known_good_proxy_settings(),
        healthy_settings
            .normalized()
            .expect("healthy settings should remain normalized")
    );
    assert!(!state.proxy_runtime_is_running().await);
}

fn available_loopback_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0")
        .expect("bind temporary loopback listener")
        .local_addr()
        .expect("temporary listener has local addr")
        .port()
}
