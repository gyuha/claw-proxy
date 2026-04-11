use std::fmt;

use serde::{Deserialize, Serialize};

const ALLOWED_LOOPBACK_HOSTS: [&str; 3] = ["localhost", "127.0.0.1", "::1"];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxySettings {
    pub listen_host: String,
    pub listen_port: u16,
    pub base_endpoint: String,
}

impl ProxySettings {
    pub fn normalized(&self) -> Result<Self, ProxySettingsValidationError> {
        let listen_host = normalize_listen_host(&self.listen_host)?;
        validate_listen_port(self.listen_port)?;
        let base_endpoint = normalize_base_endpoint(&self.base_endpoint)?;

        Ok(Self {
            listen_host,
            listen_port: self.listen_port,
            base_endpoint,
        })
    }

    pub fn effective_base_url(&self) -> Result<String, ProxySettingsValidationError> {
        let normalized = self.normalized()?;
        let authority = match normalized.listen_host.as_str() {
            "::1" => "[::1]".to_string(),
            _ => normalized.listen_host.clone(),
        };

        Ok(format!(
            "http://{}:{}{}",
            authority, normalized.listen_port, normalized.base_endpoint
        ))
    }
}

impl Default for ProxySettings {
    fn default() -> Self {
        Self {
            listen_host: "127.0.0.1".to_string(),
            listen_port: 8787,
            base_endpoint: "/v1".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProxySettingsValidationError {
    InvalidListenHost(String),
    InvalidListenPort(u16),
    InvalidBaseEndpoint(String),
}

impl fmt::Display for ProxySettingsValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidListenHost(host) => {
                write!(f, "listen host must be loopback-safe: {host}")
            }
            Self::InvalidListenPort(port) => write!(f, "listen port must be non-zero: {port}"),
            Self::InvalidBaseEndpoint(endpoint) => {
                write!(f, "base endpoint must be a local path prefix: {endpoint}")
            }
        }
    }
}

impl std::error::Error for ProxySettingsValidationError {}

fn normalize_listen_host(host: &str) -> Result<String, ProxySettingsValidationError> {
    let trimmed = host.trim();

    if ALLOWED_LOOPBACK_HOSTS.contains(&trimmed) {
        return Ok(trimmed.to_string());
    }

    Err(ProxySettingsValidationError::InvalidListenHost(
        host.to_string(),
    ))
}

fn validate_listen_port(port: u16) -> Result<(), ProxySettingsValidationError> {
    if port == 0 {
        return Err(ProxySettingsValidationError::InvalidListenPort(port));
    }

    Ok(())
}

fn normalize_base_endpoint(base_endpoint: &str) -> Result<String, ProxySettingsValidationError> {
    let trimmed = base_endpoint.trim();
    if trimmed.is_empty() {
        return Err(ProxySettingsValidationError::InvalidBaseEndpoint(
            base_endpoint.to_string(),
        ));
    }

    let without_leading_slash = trimmed.trim_start_matches('/');
    if without_leading_slash.is_empty()
        || without_leading_slash.starts_with('/')
        || without_leading_slash.contains("http://")
        || without_leading_slash.contains("https://")
        || first_segment_looks_like_host(without_leading_slash)
    {
        return Err(ProxySettingsValidationError::InvalidBaseEndpoint(
            base_endpoint.to_string(),
        ));
    }

    Ok(format!("/{}", without_leading_slash))
}

fn first_segment_looks_like_host(value: &str) -> bool {
    value
        .split('/')
        .next()
        .map(|segment| segment.contains('.') || segment.contains(':') || segment.starts_with('['))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::ProxySettings;

    #[test]
    fn proxy_settings_default_values_are_loopback_safe() {
        let settings = ProxySettings::default();

        assert_eq!(settings.listen_host, "127.0.0.1");
        assert_eq!(settings.listen_port, 8787);
        assert_eq!(settings.base_endpoint, "/v1");
    }

    #[test]
    fn proxy_settings_normalization_accepts_loopback_hosts() {
        for listen_host in ["localhost", "127.0.0.1", "::1"] {
            let normalized = ProxySettings {
                listen_host: listen_host.to_string(),
                listen_port: 8787,
                base_endpoint: "v1".to_string(),
            }
            .normalized()
            .expect("loopback settings validate");

            assert_eq!(normalized.listen_host, listen_host);
            assert_eq!(normalized.base_endpoint, "/v1");
        }
    }

    #[test]
    fn proxy_settings_normalization_rejects_invalid_values() {
        let invalid_host = ProxySettings {
            listen_host: "0.0.0.0".to_string(),
            ..ProxySettings::default()
        };
        assert!(invalid_host.normalized().is_err());

        let invalid_port = ProxySettings {
            listen_port: 0,
            ..ProxySettings::default()
        };
        assert!(invalid_port.normalized().is_err());

        let invalid_endpoint = ProxySettings {
            base_endpoint: "https://example.com/v1".to_string(),
            ..ProxySettings::default()
        };
        assert!(invalid_endpoint.normalized().is_err());
    }
}
