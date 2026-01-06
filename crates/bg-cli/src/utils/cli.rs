use std::path::PathBuf;
use std::process::exit;
use log::error;
use bg_core::media::{scan_media, MediaKind, ScanConfig};

pub async fn wait_for_shutdown_signal<F, Fut>(on_exit: F)
where F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = i32>, {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};

        let mut sigterm = signal(SignalKind::terminate()).unwrap();
        let mut sigint  = signal(SignalKind::interrupt()).unwrap();
        let mut sighup  = signal(SignalKind::hangup()).unwrap();

        tokio::select! {
            _ = sigterm.recv() => {}
            _ = sigint.recv() => {}
            _ = sighup.recv() => {}
        }

        exit(on_exit().await)
    }
}

pub fn expand_media_path (media_path: Option<PathBuf>, mut scan_config: ScanConfig) -> Vec<PathBuf> {
    if let Some(path) = media_path {
        scan_media(
            Some(path),
            MediaKind::StaticImage,
            false,
            None,
            &mut scan_config
        ).unwrap_or_else(
            |e| {
                error!("Error scanning media: {}", e);
                exit(1);
            }
        )
    } else {
        error!("Missing media_path, stop executing.");
        exit(1);
    }
}