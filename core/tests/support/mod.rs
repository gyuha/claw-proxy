use std::fs;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::Value;
use tokio::time::sleep;

pub struct RunningProxy {
    child: Option<Child>,
    config_path: PathBuf,
    admin_port: u16,
}

impl RunningProxy {
    pub async fn wait_for_status(&mut self) -> Result<Value, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let status_url = format!("http://127.0.0.1:{}/status", self.admin_port);

        for _ in 0..50 {
            if let Some(status) = self.child_mut().try_wait()? {
                let output = self.child.take().unwrap().wait_with_output()?;
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stdout = String::from_utf8_lossy(&output.stdout);

                return Err(format!(
                    "claw-proxy exited early with {status}: stdout={stdout:?} stderr={stderr:?}"
                )
                .into());
            }

            match client.get(&status_url).send().await {
                Ok(response) if response.status().is_success() => {
                    return Ok(response.json().await?);
                }
                Ok(_) | Err(_) => {
                    sleep(Duration::from_millis(100)).await;
                }
            }
        }

        Err(format!("timed out waiting for {status_url}").into())
    }

    fn child_mut(&mut self) -> &mut Child {
        self.child.as_mut().expect("child process missing")
    }
}

impl Drop for RunningProxy {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }

        let _ = fs::remove_file(&self.config_path);
    }
}

pub fn allocate_ports() -> std::io::Result<(u16, u16, u16)> {
    let proxy_listener = std::net::TcpListener::bind(("127.0.0.1", 0))?;
    let admin_listener = std::net::TcpListener::bind(("127.0.0.1", 0))?;
    let ws_listener = std::net::TcpListener::bind(("127.0.0.1", 0))?;

    let proxy_port = proxy_listener.local_addr()?.port();
    let admin_port = admin_listener.local_addr()?.port();
    let ws_port = ws_listener.local_addr()?.port();

    drop(proxy_listener);
    drop(admin_listener);
    drop(ws_listener);

    Ok((proxy_port, admin_port, ws_port))
}

pub fn write_temp_config(
    proxy_port: u16,
    admin_port: u16,
    ws_port: u16,
) -> std::io::Result<PathBuf> {
    let mut path = std::env::temp_dir();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos();

    path.push(format!(
        "claw-proxy-test-{}-{timestamp}.yaml",
        std::process::id()
    ));

    let config = format!(
        r#"server:
  proxy_port: {proxy_port}
  admin_port: {admin_port}
  ws_port: {ws_port}

routing:
  strategy: round_robin

providers:
  - name: test-openai
    type: openai
    api_key: sk-test
    models:
      - gpt-4o-mini
"#
    );

    fs::write(&path, config)?;

    Ok(path)
}

pub async fn start_proxy(
    proxy_port: u16,
    admin_port: u16,
    ws_port: u16,
) -> Result<RunningProxy, Box<dyn std::error::Error>> {
    let config_path = write_temp_config(proxy_port, admin_port, ws_port)?;
    let child = Command::new(env!("CARGO_BIN_EXE_claw-proxy"))
        .arg(&config_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut proxy = RunningProxy {
        child: Some(child),
        config_path,
        admin_port,
    };

    let _ = proxy.wait_for_status().await?;

    Ok(proxy)
}
