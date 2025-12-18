use std::path::Path;

use tokio::process::Child;

pub enum BackendCapability {
    Static,
    Animated,
    Audio
}

pub struct WallpaperProcess {
    backend: Box<dyn WallpaperBackend>,
    child: Option<tokio::process::Child>
}

pub trait WallpaperBackend {
    fn start(&self, media_path: &Path) -> Result<Option<Child>, std::io::Error>;
    fn update(&self, media_path: &Path) -> Result<Option<Child>, std::io::Error>;
    fn stop(&self) -> Result<(), std::io::Error>;
    fn capabilities(&self) -> Vec<BackendCapability>;
}
