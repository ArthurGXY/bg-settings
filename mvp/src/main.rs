use rand::prelude::SliceRandom;
use std::path::{Path, PathBuf};
use std::fs;
use std::process::exit;
use tokio::{
    process::{Child, Command},
    time::{interval, Duration}
};

struct WallpaperProcess {
    backend: Box<dyn WallpaperBackend>,
    child: Option<tokio::process::Child>
}

trait WallpaperBackend {
    fn start(&self, media_path: &Path) -> Result<Option<Child>, std::io::Error>;

    fn stop(&self, child: &mut Option<Child>) -> Result<(), std::io::Error>;
}

impl WallpaperProcess {
    pub fn start() {

    }
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

    let mut img_paths = scan_images(
        format!("{}/Pictures/wallpaper/", std::env::var("HOME").unwrap())
    ).expect("Failed to scan images");

    if img_paths.is_empty() {
        eprintln!("No images in configured path. Exit.");
        exit(0)
    }

    let mut ticker = interval(Duration::from_secs(5));
    ticker.tick().await;

    let mut rng_instance = rng();
    img_paths.shuffle(&mut rng_instance);

    let mut idx = 0usize;
    let mut child = start_paper_with_image(img_paths.get(idx).unwrap()).await;

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
        let img_path = &img_paths[idx];
        child = start_paper_with_image(img_path).await;
        idx = idx + 1;
        if idx >= img_paths.len() {
            idx = 0;
            img_paths.shuffle(&mut rng_instance);
        }
    }
}

