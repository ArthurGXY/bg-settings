use std::io::Error;

use crate::backend::MultiOutputBackend;
use super::{WallpaperBackend, BackendCapability, BackendSpawnSpec};

use tokio::process::Child;
use log::{info, error, debug};
use which::which;


pub struct SwaybgBackend;

impl WallpaperBackend for SwaybgBackend {
    fn name(&self) -> &str {
        "swaybg"
    }

    fn start(&self, spec: &BackendSpawnSpec) -> Result<Child, std::io::Error> {
        info!("Starting swaybg backend.");
        
        let image = &spec.media;

        let mut binding = tokio::process::Command::new("swaybg");
        let command = binding
            .arg("-o")
            .arg(&spec.output.name)
            .arg("-i")
            .arg(image)
            ;
        let cmd_std = &command.as_std();
        let cmd = format!("{}", vec![
            cmd_std.get_program().to_str().unwrap(),
              cmd_std.get_args()
                  .collect::<Vec<_>>()
              .join(" ".as_ref()).to_str().unwrap()].join(" "));

        debug!("Constructed command: {}", cmd);

        let child_proc = command.spawn()?;
        Ok(child_proc)
    }

    fn exists(&self) -> bool {
        debug!("Looking for executable `swaybg`");
        which("swaybg").is_ok()
    }

    fn capabilities(&self) -> Vec<super::BackendCapability> {
        vec![BackendCapability::Static]
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
