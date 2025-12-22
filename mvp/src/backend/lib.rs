use std::path::Path;

use tokio::process::Child;

pub enum ImageType {
    APNG,
    PNG,
    JPEG,
    JPEGXL,
    GIF,
    SVG,
    WEBP,
}

pub enum BackendCapability {
    Static,
    Animated,
    MultiOutput,
    HotReload
}

pub enum WallpaperMode {
    Stretch, 
    Fit, 
    Fill, 
    Center, 
    Tile, 
    SolidColor
}

pub enum Backend {
    Swaybg(SwaybgBackend),
    MpvPaper(MpvPaperBackend),
    Awww(AwwwBackend)
}

pub struct WallpaperProcess {
    backend: Box<dyn WallpaperBackend>,
    child: Option<tokio::process::Child>
}

pub trait WallpaperBackend {
    fn start(&self, media_path: &BackendSpawnSpec) -> Result<Child, std::io::Error>;
    // Removed update, move this to HotReload trait. 
    // If no such trait, we stop and restart the backend manually.
    // fn update(&self, media_path: &Path) -> Result<Option<Child>, std::io::Error>;
    fn exists(&self) -> bool;
    fn stop(&self, c: &mut Child) -> Result<(), std::io::Error> {
        c.start_kill()
    }
    fn capabilities(&self) -> Vec<BackendCapability>;
}

pub trait MultiOutputBackend {
    fn start_multi_output(&self, specs: &[BackendSpawnSpec]) -> Result<Vec<Child>, std::io::Error>;
}

use std::ffi::OsString;
use std::path::PathBuf;

use crate::backend::awww::AwwwBackend;
use crate::backend::mpvpaper::MpvPaperBackend;
use crate::backend::swaybg::SwaybgBackend;

pub struct BackendSpawnSpec {
    pub media: PathBuf,
    pub mode: WallpaperMode,
    pub output: OsString,
    pub extra_args: Vec<OsString>,
}

pub async fn stop_and_wait(
    backend: &dyn WallpaperBackend,
    mut child: Child,
) -> std::io::Result<()> {
    backend.stop(&mut child)?;
    child.wait().await?;
    Ok(())
}
