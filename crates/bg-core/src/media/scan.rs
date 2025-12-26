use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use infer::is_image;
use log::{error, info};
use rand::prelude::IndexedRandom;
use rand::{rng};
use rand::seq::index::sample;
use walkdir::WalkDir;
use crate::media::detect_media_kind;
use crate::media::mime::MediaKind;

#[derive(Debug, Clone, Copy)]
pub enum ScanMode {
    StaticImage,
    DynamicMedia,
}



pub fn scan_media_recursive(
    root: impl AsRef<Path>,
    filter: MediaKind,
) -> std::io::Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    scan_dir(root.as_ref(), filter, &mut result)?;
    Ok(result)
}

fn scan_dir(
    dir: &Path,
    filter: MediaKind,
    out: &mut Vec<PathBuf>,
) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            scan_dir(&path, filter, out)?;
            continue;
        }

        if !path.is_file() {
            continue;
        }

        let kind = detect_media_kind(&path);

        if filter == MediaKind::Unsupported || kind == filter {
            out.push(path);
        }
    }

    Ok(())
}
pub fn scan_media(root: Option<PathBuf>,
                  mode: MediaKind,
                  random: bool,
                  random_amount: Option<usize>
) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut result;

    if let Some(root) = root {
        let scan_result = scan_media_recursive(
            root,
            mode,
        );
        match scan_result {
            Ok(paths) => {
                info!("Found {} static media", paths.len());
                result = paths;

                let mut randomly_picked = Vec::new();

                if random {
                    let mut rng = rng();
                    let random_amount = random_amount.unwrap_or(1);

                    let indices = sample(
                        &mut rng,
                        result.len(), random_amount
                    ).into_vec();

                    for idx in indices {
                        randomly_picked.push(result.swap_remove(idx));
                    }

                    Ok(randomly_picked)
                } else {
                    Ok(result)
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
        }
    } else {
        error!("No media path provided.");
        Err(std::io::Error::new(ErrorKind::InvalidInput, "No media path provided."))
    }
}
