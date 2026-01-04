mod utils;

use log::{error, info, trace};
use env_logger;
use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;
use std::process::exit;
use bg_core::{wl, orchestrator, backend};
use bg_core::backend::WallpaperMode;
use bg_core::media::{scan_media, MediaKind};
use utils::constants::{ListTarget, ANIMATED_MEDIA, BACKEND, HELP, OUTPUT, SEAT, STATIC_MEDIA};
use crate::utils::wait_for_shutdown_signal;

#[derive(Parser, Debug)]
#[command(name="bg-settings", version = "0.1", about = "A wallpaper orchestrator for wayland")]
struct Cli {
    media_path: Option<PathBuf>,

    backend: Option<String>,

    #[clap(short, long)]
    #[clap(default_value_t = false)]
    recursive: bool,

    #[clap(short, long)]
    max_recurse_depth: Option<u8>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    List {
        target: Option<ListTarget>
    },
    Setup {
        outputs: Option<Vec<String>>
    },
}

#[tokio::main]
async fn main() {
    unsafe {
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", "trace");
        }
    }
    env_logger::init();

    let args = Cli::parse();
    trace!("Got args: {:?}", args);

    let mut thread_pool: Vec<tokio::process::Child> = Vec::new();

    match args.command {
        // Command: List
        Some(Commands::List{target: opt_target }) => {
            match opt_target {
                Some(target) => {
                    // check media path for those commands that requires it.
                    if target.requires_media_path() & args.media_path.is_none() {
                        error!("media_path is required for listing {:?}", target);
                        exit(1);
                    }

                    match target {
                        t if t.is_in(&SEAT) => {
                            let (_, seat_info) = wl::get_info();
                            for info in seat_info {
                                println!("{}", info)
                            }
                        }

                        t if t.is_in(&OUTPUT) => {
                            let (output_info, _) = wl::get_info();
                            for info in output_info {
                                println!("{}", info)
                            }
                        }

                        t if t.is_in(&STATIC_MEDIA) => {
                            if let Ok(media_info) = scan_media(
                                args.media_path,
                                MediaKind::StaticImage,
                                false, None
                            ) {
                                for info in media_info {
                                    println!("{}", info.to_string_lossy())
                                }
                            }
                        }

                        t if t.is_in(&ANIMATED_MEDIA) => {
                            if let Ok(media_info) = scan_media(
                                args.media_path,
                                MediaKind::AnimatedImage,
                                false, None
                            ) {
                                for info in media_info {
                                    println!("{}", info.to_string_lossy())
                                }
                            }
                        }

                        t if t.is_in(&BACKEND) => {
                            let backends = backend::available_backends();
                            print!("Detected backends: ");
                            for backend in backends {
                                print!("{} ", backend.name())
                            }
                        }

                        t if t.is_in(&HELP) => {
                            todo!()
                        }

                        _ => {
                            error!("Unknown target {:?} selected", target);
                        }
                    }
                },

                None => todo!(),
            }
        }
        // default: Fill all outputs with random pictures if provided media_path, else noop.
        Some(Commands::Setup{
                 outputs: target_output,
             }) => {
            let media_path = match args.media_path {
                Some(path) => path,
                None => {
                    error!("media_path is required for setting up wallpaper.");
                    exit(1)
                }
            };

            match orchestrator::setup_wallpaper(
                media_path,
                args.backend,
                target_output,
                WallpaperMode::Fit,
            ).await { // if setup, put into thread_pool and wait for shutdown signal.
                Ok(children) => {
                    thread_pool.extend(children);
                    info!("Main function reaching end");

                    wait_for_shutdown_signal(
                        || async move {
                            let mut ok = true;
                            info!("Received kill signal, exiting.");

                            for mut child in thread_pool {
                                if let Some(pid) = child.id() {
                                    info!("Killing child thread: {}", pid);
                                }

                                if child.kill().await.is_err() {
                                    ok = false;
                                }
                            }

                            if ok { 0 } else {
                                error!("Failed terminating all child processes.");
                                1
                            }
                        }
                    ).await
                }
                
                Err(e) => {
                    error!("Setup failed: {}", e);
                    exit(1);
                }
            }
        }
        None => {
            error!("No subcommand provided");
        }
    }
}
