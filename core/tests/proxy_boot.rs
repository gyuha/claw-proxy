mod support;

use support::{allocate_ports, start_proxy};

#[tokio::test]
async fn boot_smoke_starts_servers() -> Result<(), Box<dyn std::error::Error>> {
    let (proxy_port, admin_port, ws_port) = allocate_ports()?;
    let mut proxy = start_proxy(proxy_port, admin_port, ws_port).await?;
    let status = proxy.wait_for_status().await?;

    assert_eq!(status["status"], "running");
    assert_eq!(status["proxy_port"], proxy_port);
    assert_eq!(status["admin_port"], admin_port);

    Ok(())
}
