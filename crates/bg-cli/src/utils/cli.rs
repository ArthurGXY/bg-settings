use std::process::exit;

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
