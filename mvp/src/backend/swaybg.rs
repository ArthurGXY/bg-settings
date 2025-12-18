use super::WallpaperBackend;

use std::path::Path;
use tokio::process::Child;

pub struct SwaybgBackend;

impl WallpaperBackend for SwaybgBackend {
    fn start(&self, media: &Path) -> Result<Option<Child>, std::io::Error> {
        todo!()
    }

    fn stop(&self) -> Result<(), std::io::Error> {
        todo!()
    }

    fn update(&self, media: &Path) -> Result<Option<Child>, std::io::Error> {
        todo!()
    }
    
    fn capabilities(&self) -> Vec<super::BackendCapability> {
        todo!()
    }
}
