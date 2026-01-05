use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use log::{error, info};
use crate::media::detect_media_kind;
use crate::media::mime::MediaKind;

pub struct ScanConfig {
    pub recurse: bool,
    pub max_recurses: isize,
}

pub fn scan_media_recursive(
    root: impl AsRef<Path>,
    filter: MediaKind,
    scan_config: &mut ScanConfig
) -> std::io::Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    scan_dir(root.as_ref(), filter, scan_config, &mut result)?;
    Ok(result)
}

fn scan_dir(
    dir: &Path,
    filter: MediaKind,
    scan_config: &mut ScanConfig,
    out: &mut Vec<PathBuf>,
) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if scan_config.recurse && scan_config.max_recurses != 0 {
                // -1 = infinite recursion
                if scan_config.max_recurses > 0 {
                    scan_config.max_recurses -= 1;
                }
                scan_dir(&path, filter, scan_config, out)?;
                if scan_config.max_recurses > 0 {
                    scan_config.max_recurses += 1;
                }
            }
            continue;
        }

        if !path.is_file() {
            continue;
        }

        let kind = detect_media_kind(&path);
        let is_any_supported = filter != MediaKind::Unsupported && filter == MediaKind::Any;
        if is_any_supported || kind == filter {
            out.push(path);
        }
    }

    Ok(())
}
pub fn scan_media(root: Option<PathBuf>,
                  mode: MediaKind,
                  random: bool,
                  random_amount: Option<usize>,
                  scan_config: &mut ScanConfig
) -> Result<Vec<PathBuf>, std::io::Error> {
    if let Some(root) = root {
        let scan_result = scan_media_recursive(
            root,
            mode,
            scan_config,
        );
        match scan_result {
            Ok(paths) => {
                // 根据媒体类型记录不同的日志消息
                let media_type_str = match mode {
                    MediaKind::StaticImage => "static",
                    MediaKind::AnimatedImage => "animated",
                    MediaKind::Video => "video",
                    MediaKind::Unsupported => "unsupported",
                    MediaKind::Any => "any",
                };
                info!("Listing {} media. Count: {} ", media_type_str, paths.len());

                if random && !paths.is_empty() {
                    let mut rng = rand::rng();
                    let random_amount = random_amount.unwrap_or(1).min(paths.len());

                    let indices = rand::seq::index::sample(
                        &mut rng,
                        paths.len(), random_amount
                    ).into_vec();

                    let randomly_picked = indices.into_iter()
                        .map(|idx| paths[idx].clone())
                        .collect();
                    
                    Ok(randomly_picked)
                } else {
                    Ok(paths)
                }
            }
            Err(e) => {
                error!("Failed to scan media: {}", e);
                Err(e)
            }
        }
    } else {
        error!("No media path provided.");
        Err(std::io::Error::new(ErrorKind::InvalidInput, "No media path provided."))
    }
}

/// 列出媒体文件的辅助函数
pub fn list_media(
    media_path: Option<PathBuf>,
    kind: MediaKind,
    recursive: bool,
    max_recurse_depth: i8,
) -> std::io::Result<()> {
    let mut scan_config = ScanConfig {
        recurse: recursive,
        max_recurses: max_recurse_depth.into(),
    };
    
    let media_info = scan_media(media_path, kind, false, None, &mut scan_config)?;
    
    for info in media_info {
        println!("{}", info.to_string_lossy());
    }
    
    Ok(())
}
