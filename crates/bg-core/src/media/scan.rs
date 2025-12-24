use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use log::{error, info};
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy)]
pub enum ScanMode {
    StaticImage,
    DynamicMedia,
}


fn is_dynamic_image(mime: &str) -> bool {
    matches!(
        mime,
        "image/gif"
            | "image/webp"
            | "image/apng"
    )
}

pub fn scan_media_recursive(
    root: impl AsRef<Path>,
    mode: ScanMode,
) -> std::io::Result<Vec<PathBuf>> {
    let mut results = Vec::new();
    let infer = infer::Infer::new();

    for entry in WalkDir::new(root).follow_links(true) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();

        let kind = match infer.get_from_path(path) {
            Ok(Some(k)) => k,
            _ => continue, // 无法识别的直接忽略
        };

        let mime = kind.mime_type();

        let matched = match mode {
            ScanMode::StaticImage => {
                mime.starts_with("image/")
                    && !is_dynamic_image(mime)
            }
            ScanMode::DynamicMedia => {
                is_dynamic_image(mime) || mime.starts_with("video/")
            }
        };

        if matched {
            results.push(path.to_path_buf());
        }
    }

    Ok(results)
}

pub fn scan_media(root: Option<PathBuf>, mode: ScanMode) -> Result<Vec<PathBuf>, std::io::Error> {
    if let Some(root) = root {
        let static_media = scan_media_recursive(
            root,
            mode
        );
        match static_media {
            Ok(paths) => {
                info!("Found {} static media", paths.len());
                Ok(paths)
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
