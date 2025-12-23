use crate::backend::MultiOutputBackend;

use super::{WallpaperBackend, BackendCapability, BackendSpawnSpec};

use std::path::Path;
use tokio::process::Child;
use log::{info, error, warn, debug, trace};

use which::which;

pub struct SwaybgBackend;

impl WallpaperBackend for SwaybgBackend {
    fn start(&self, spec: &BackendSpawnSpec) -> Result<Child, std::io::Error> {
        info!("Starting swaybg backend.");
        
        let mut binding = tokio::process::Command::new("swaybg");
        let command = binding
            .arg("-i")
            .arg(spec.media.clone());

        let cmd = format!("{:?}", &command);
        debug!("Constructed command: {}", cmd);

        let child_proc = command.spawn();
        if child_proc.is_err() {
            error!("Failed starting process.");
            return Err(child_proc.unwrap_err());
        } else {
            info!("Started swaybg process.");
            let c = child_proc.unwrap();
            Ok(c)
        }
    }
    
    fn capabilities(&self) -> Vec<super::BackendCapability> {
        vec![BackendCapability::Static]
    }
    
    fn exists(&self) -> bool {
        which("swaybg").is_ok()
    }

    fn name(&self) -> &str {
        "swaybg"
    }
}

impl MultiOutputBackend for SwaybgBackend {
    fn start_multi_output(&self, specs: &[BackendSpawnSpec]) -> Result<Vec<tokio::process::Child>, std::io::Error> {
        let mut children = Vec::new();
        for spec in specs {
            match self.start(spec) {
                Ok(child) => children.push(child),
                Err(e) => {
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
