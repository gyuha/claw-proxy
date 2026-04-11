use claw_proxy_lib::models::proxy_settings::ProxySettings;

#[test]
fn proxy_settings_default_values_are_loopback_safe() {
    let settings = ProxySettings::default();

    assert_eq!(settings.listen_host, "127.0.0.1");
    assert_eq!(settings.listen_port, 8787);
    assert_eq!(settings.base_endpoint, "/v1");
}

#[test]
fn proxy_settings_validation_accepts_loopback_hosts_and_normalizes_base_endpoint() {
    for host in ["localhost", "127.0.0.1", "::1"] {
        let normalized = ProxySettings {
            listen_host: host.to_string(),
            listen_port: 8787,
            base_endpoint: "v1".to_string(),
        }
        .normalized()
        .expect("loopback settings should validate");

        assert_eq!(normalized.listen_host, host);
        assert_eq!(normalized.base_endpoint, "/v1");
    }
}

#[test]
fn proxy_settings_validation_rejects_invalid_ports_and_absolute_base_endpoints() {
    let invalid_port = ProxySettings {
        listen_host: "127.0.0.1".to_string(),
        listen_port: 0,
        base_endpoint: "/v1".to_string(),
    };
    assert!(invalid_port.normalized().is_err());

    let invalid_host = ProxySettings {
        listen_host: "0.0.0.0".to_string(),
        listen_port: 8787,
        base_endpoint: "/v1".to_string(),
    };
    assert!(invalid_host.normalized().is_err());

    let invalid_endpoint = ProxySettings {
        listen_host: "127.0.0.1".to_string(),
        listen_port: 8787,
        base_endpoint: "https://example.com/v1".to_string(),
    };
    assert!(invalid_endpoint.normalized().is_err());

    let endpoint_with_host_segment = ProxySettings {
        listen_host: "127.0.0.1".to_string(),
        listen_port: 8787,
        base_endpoint: "example.com/v1".to_string(),
    };
    assert!(endpoint_with_host_segment.normalized().is_err());
}
