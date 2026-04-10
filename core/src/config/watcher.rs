use notify::{Watcher, RecursiveMode, recommended_watcher, Event};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
use tokio::sync::watch;
use super::{Config, load};

pub fn start(path: PathBuf, tx: watch::Sender<Config>) {
    let path_clone = path.clone();
    std::thread::spawn(move || {
        let (notify_tx, notify_rx) = mpsc::channel();
        let mut watcher = recommended_watcher(move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                let _ = notify_tx.send(event);
            }
        }).expect("Failed to create watcher");

        watcher.watch(&path_clone, RecursiveMode::NonRecursive)
            .expect("Failed to watch config file");

        loop {
            if notify_rx.recv_timeout(Duration::from_secs(1)).is_ok() {
                // debounce: 100ms 대기 후 추가 이벤트 소진
                std::thread::sleep(Duration::from_millis(100));
                while notify_rx.try_recv().is_ok() {}

                match load(&path_clone) {
                    Ok(new_config) => {
                        tracing::info!("Config reloaded from {:?}", path_clone);
                        let _ = tx.send(new_config);
                    }
                    Err(e) => tracing::error!("Config reload failed: {}", e),
                }
            }
        }
    });
}
