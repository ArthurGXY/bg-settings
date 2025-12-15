use rand::prelude::SliceRandom;
use std::path::{Path, PathBuf};
use std::fs;
use tokio::{
    process::{Child, Command},
    time::{interval, Duration}
};

async fn reset() {
    println!("periodic reset");
}

pub fn scan_images<P: AsRef<Path>>(dir: P) -> std::io::Result<Vec<PathBuf>> {
    let mut images = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        // infer 会读取文件头的一小部分
        if let Ok(Some(kind)) = infer::get_from_path(&path) {
            if kind.mime_type().starts_with("image/") {
                images.push(path);
            }
        }
    }

    Ok(images)
}


// pub fn scan_images<P: AsRef<Path>>(dir: P) -> std::io::Result<Vec<PathBuf>> {
//     let mut images = Vec::new();
//
//     for entry in fs::read_dir(dir)? {
//         let entry = entry?;
//         let path = entry.path();
//
//         if path.is_file() {
//             if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
//                 match ext.to_lowercase().as_str() {
//                     "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "tiff" => {
//                         images.push(path);
//                     }
//                     _ => {}
//                 }
//             }
//         }
//     }
//
//     Ok(images)
// }

async fn start_paper() -> Child {
    Command::new("swaybg")
        .arg("--folder")
        .arg(format!("{}/Pictures", std::env::var("HOME").unwrap()))
        .spawn()
        .expect("failed to start backend.")
}

async fn start_paper_with_image<P: AsRef<Path>>(image: P) -> Child {
    Command::new("swaybg")
        .arg("--image")
        .arg(image.as_ref().as_os_str().to_owned())
        .spawn()
        .expect("failed to start backend.")
}

#[tokio::main]
async fn main() {
    use rand::{rng};

    let mut images = scan_images(
        format!("{}/Pictures/wallpaper/", std::env::var("HOME").unwrap())
    ).expect("Failed to scan images");

    let mut ticker = interval(Duration::from_secs(5));
    let mut child = start_paper().await;

    // 随机打乱一次
    let mut rng = rng();
    images.shuffle(&mut rng);

    let mut idx = 0usize;

    loop {
        ticker.tick().await;

        println!("timer expired, restarting waypaper");

        // 尝试结束旧进程
        if let Some(id) = child.id() {
            println!("killing waypaper (pid {})", id);
        }

        // kill 是 async 的
        let _ = child.kill().await;
        let _ = child.wait().await;

        // 启动新进程
        child = start_paper_with_image(images.get(idx).unwrap()).await;
        idx = idx + 1;
    }
}

