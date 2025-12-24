use bg_core::backend::{Backend, WallpaperBackend};
use log::{error, info};

pub fn detect_backends() -> Vec<Box<dyn WallpaperBackend>> {
    info!("Detecting backends");
    let mut backends: Vec<Box<dyn WallpaperBackend>> = Vec::new();
    for backend in Backend::available_backends() {
        if backend.exists() {
            info!("Found backend {:?}", backend.name());
            backends.push(backend);
        } else {
            error!("No backend {:?} found", backend.name());
        }
    }
    backends
}