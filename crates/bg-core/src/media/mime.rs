use std::{fs::File, io::Read, path::Path};
use infer::Infer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKind {
    StaticImage,
    AnimatedImage,
    Video,
    Unsupported,
    Any
}

pub fn detect_media_kind<P: AsRef<Path>>(path: P) -> MediaKind {
    let path = path.as_ref();
    let infer = Infer::new();

    let kind = match infer.get_from_path(path) {
        Ok(Some(k)) => k,
        _ => return MediaKind::Unsupported,
    };

    match kind.mime_type() {
        // ===== 静态图片（infer 已经能确认）=====
        "image/jpeg"
        | "image/bmp"
        | "image/tiff"
        | "image/x-icon" => MediaKind::StaticImage,

        // ===== PNG / WebP 需要进一步判断 =====
        "image/png" => {
            if is_apng(path) {
                MediaKind::AnimatedImage
            } else {
                MediaKind::StaticImage
            }
        }

        "image/webp" => {
            if is_animated_webp(path) {
                MediaKind::AnimatedImage
            } else {
                MediaKind::StaticImage
            }
        }

        // ===== 明确动画图片 =====
        "image/gif" => MediaKind::AnimatedImage,

        // ===== AVIF：infer 不区分，先按静态处理（可扩展）=====
        "image/avif" => MediaKind::StaticImage,

        // ===== 视频 =====
        mime if mime.starts_with("video/") => MediaKind::Video,

        _ => MediaKind::Unsupported,
    }
}


fn is_apng(path: &Path) -> bool {
    let mut buf = Vec::new();
    if File::open(path).and_then(|mut f| f.read_to_end(&mut buf)).is_err() {
        return false;
    }
    buf.windows(4).any(|w| w == b"acTL")
}

fn is_animated_webp(path: &Path) -> bool {
    let mut buf = Vec::new();
    if File::open(path).and_then(|mut f| f.read_to_end(&mut buf)).is_err() {
        return false;
    }
    buf.windows(4).any(|w| w == b"ANIM")
}
