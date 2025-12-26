use tokio::process::Child;
use crate::wl_info;
use crate::media::MediaKind;

pub enum ImageType {
    APNG,
    PNG,
    JPEG,
    JPEGXL,
    GIF,
    SVG,
    WEBP,
}

#[derive(PartialEq)]
pub enum BackendCapability {
    Static,
    Animated,
    Video,
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

impl Backend {
    pub fn exists(&self) -> bool {
        match self {
            Backend::Swaybg(backend) => backend.exists(),
            // Backend::MpvPaper(backend) => backend.exists(),
            // Backend::Awww(backend) => backend.exists(),
            _ => false
        }
    }

    pub fn supported_backends() -> Vec<Box<dyn WallpaperBackend>> {
        let mut backends: Vec<Box<dyn WallpaperBackend>>  = Vec::new();
        backends.push(Box::new(SwaybgBackend));
        backends
    }
}

impl From<MediaKind> for Option<BackendCapability> {
    fn from(kind: MediaKind) -> Self {
        match kind {
            MediaKind::StaticImage => Some(BackendCapability::Static),
            MediaKind::AnimatedImage => Some(BackendCapability::Animated),
            MediaKind::Video => Some(BackendCapability::Video),
            MediaKind::Unsupported | MediaKind::Any => None,
        }
    }
}

pub struct WallpaperProcess {
    backend: Box<dyn WallpaperBackend>,
    child: Option<tokio::process::Child>
}

pub trait WallpaperBackend {
    fn name(&self) -> &str;
    fn start(&self, media_path: &BackendSpawnSpec) -> Result<Child, std::io::Error>;
    // Removed update, move this to HotReload trait. 
    // If no such trait, we stop and restart the backend manually.
    // fn update(&self, media_path: &Path) -> Result<Option<Child>, std::io::Error>;
    fn exists(&self) -> bool;
    fn stop(&self, c: &mut Child) -> Result<(), std::io::Error> {
        c.start_kill()
    }
    fn capabilities(&self) -> Vec<BackendCapability>;
    fn start_multi(&self, specs: Vec<BackendSpawnSpec>) -> Result<Vec<Child>, std::io::Error> {
        error!("Not implemented.");
        exit(1)
    }
}

pub trait MultiOutputBackend {
    fn start_multi_output(&self, specs: &[BackendSpawnSpec]) -> Result<Vec<Child>, std::io::Error>;
}

use std::ffi::OsString;
use std::path::PathBuf;
use std::process::exit;
use log::{error, info};
use crate::backend::awww::AwwwBackend;
use crate::backend::mpvpaper::MpvPaperBackend;
use crate::backend::swaybg::SwaybgBackend;
use crate::wl_info::OutputInfo;

pub struct BackendSpawnSpec {
    pub media: PathBuf,
    pub mode: WallpaperMode,
    pub output: OutputInfo,
    pub extra_args: Vec<OsString>,
}

pub struct MultiMonitorBackendSpawnSpec {
    pub media: PathBuf,
    pub mode: WallpaperMode,
    pub outputs: Vec<OsString>,
    pub extra_args: Vec<OsString>
}

pub async fn stop_and_wait(
    backend: &dyn WallpaperBackend,
    mut child: Child,
) -> std::io::Result<()> {
    backend.stop(&mut child)?;
    child.wait().await?;
    Ok(())
}

pub fn available_backends() -> Vec<Box<dyn WallpaperBackend>> {
    info!("Detecting backends");
    let mut backends: Vec<Box<dyn WallpaperBackend>> = Vec::new();
    for backend in Backend::supported_backends() {
        if backend.exists() {
            info!("Found backend {:?}", backend.name());
            backends.push(backend);
        } else {
            error!("No backend {:?} found", backend.name());
        }
    }
    backends
}

pub fn get_backend_by_name(name: &String) -> Option<Box<dyn WallpaperBackend>> {
    available_backends().into_iter().find(
        |backend| backend.name() == name
    )
}

pub fn get_first_backend() -> Box<dyn WallpaperBackend> {
    available_backends().into_iter().find(
        |backend| backend.exists()
    ).unwrap()
}