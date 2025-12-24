mod utils;

use log::{error, info, trace};
use env_logger;
use clap::{Args, Parser, Subcommand, ValueEnum};

use std::env;
use std::path::PathBuf;
use std::process::exit;
use bg_core::wl_info;
use bg_core::media::{scan_media, scan_media_recursive, ScanMode};
use utils::constants::{ListTarget, ANIMATED_MEDIA, HELP, BACKEND, OUTPUT, SEAT, STATIC_MEDIA};
use utils::functions;

#[derive(Parser, Debug)]
#[command(name="bg-settings", version = "0.1", about = "A wallpaper orchestrator for wayland")]
struct Cli {
    media_path: Option<PathBuf>,

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
    }
}

fn main() {
    // unsafe { env::set_var("RUST_LOG", "debug"); }
    env_logger::init();

    let args = Cli::parse();
    trace!("Got args: {:?}", args);

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
                            let (_, seat_info) = wl_info::get_info();
                            for info in seat_info {
                                println!("{}", info)
                            }
                        }

                        t if t.is_in(&OUTPUT) => {
                            let (output_info, _) = wl_info::get_info();
                            for info in output_info {
                                println!("{}", info)
                            }
                        }

                        t if t.is_in(&STATIC_MEDIA) => {
                            if let Ok(media_info) = scan_media(
                                args.media_path,
                                ScanMode::StaticImage
                            ) {
                                for info in media_info {
                                    println!("{}", info.to_string_lossy())
                                }
                            }
                        }

                        t if t.is_in(&ANIMATED_MEDIA) => {
                            if let Ok(media_info) = scan_media(
                                args.media_path,
                                ScanMode::DynamicMedia
                            ) {
                                for info in media_info {
                                    println!("{}", info.to_string_lossy())
                                }
                            }
                        }

                        t if t.is_in(&BACKEND) => {
                            let backends = functions::detect_backends();
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
        None => todo!()
    }

    // functions::detect_backends();
}
