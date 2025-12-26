use std::io::Error;
use crate::backend::MultiOutputBackend;

use super::{WallpaperBackend, BackendCapability, BackendSpawnSpec};

use std::path::Path;
use tokio::process::Child;
use log::{info, error, warn, debug, trace};

use which::which;
use crate::media::{scan_media, MediaKind, ScanMode};

pub struct SwaybgBackend;

impl WallpaperBackend for SwaybgBackend {
    fn start(&self, spec: &BackendSpawnSpec) -> Result<Child, std::io::Error> {
        info!("Starting swaybg backend.");

        let image = scan_media(
            Some(spec.media.clone()),
            MediaKind::StaticImage,
            // randomly pick 1 to ensure only 1 image is selected.
            true, Some(1))?.get(0).unwrap().clone();

        let mut binding = tokio::process::Command::new("swaybg");
        let command = binding
            .arg("-o")
            .arg(&spec.output.name)
            .arg("-i")
            .arg(image)
            ;

        let cmd = format!("{:?}", &command);
        debug!("Constructed command: {}", cmd);

        let child_proc = command.spawn()?;
        Ok(child_proc)
    }
    
    fn capabilities(&self) -> Vec<super::BackendCapability> {
        vec![BackendCapability::Static]
    }
    
    fn exists(&self) -> bool {
        debug!("Looking for executable `swaybg`");
        which("swaybg").is_ok()
    }

    fn name(&self) -> &str {
        "swaybg"
    }

    fn start_multi(&self, specs: Vec<BackendSpawnSpec>) -> Result<Vec<Child>, Error> {
        self.start_multi_output(&*specs)
    }
}

impl MultiOutputBackend for SwaybgBackend {
    fn start_multi_output(&self, specs: &[BackendSpawnSpec]) -> Result<Vec<tokio::process::Child>, std::io::Error> {
        let mut children = Vec::new();
        for spec in specs {
            match self.start(spec) {
                Ok(child) => children.push(child),
                Err(e) => {
                    error!("Error starting swaybg: {}", e);
                    // Rollback
                    for mut c in children {
                        let _ = self.stop(&mut c);
                    }
                    return Err(e);
                }
            }
        }

        Ok(children)
    }
}
