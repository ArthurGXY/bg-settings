mod utils;
mod interact;

use log::{error, info, trace};
use env_logger;
use clap::{Args, Parser, Subcommand, ValueEnum};

use std::env;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::process::exit;
use bg_core::{backend, wl_info};
use bg_core::backend::{available_backends, get_backend_by_name, Backend};
use bg_core::media::{scan_media, scan_media_recursive, ScanMode};
use utils::constants::{ListTarget, ANIMATED_MEDIA, BACKEND, HELP, OUTPUT, SEAT, STATIC_MEDIA};

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

            let (outputs, seats) = wl_info::get_info();
            let existing_outputs;
            let backends_available = available_backends();

            if backends_available.is_empty() {
                error!("No available backend found. Supported backends:\n{}",
                    Backend::supported_backends()
                    .into_iter()
                    .map(
                        |b| b.name().to_string()
                    ).collect::<Vec<_>>().join("\n")
                );
            }

            if let Some(output_targets) = &target_output { // if
                existing_outputs = outputs.into_iter().filter_map(|t| {
                    match output_targets.contains(&t.name) {
                        false => {
                            error!("Output with name {:?} not found", t.name);
                            None
                        },
                        true => Some(t)
                    }
                }).collect::<Vec<_>>();
            } else { // user did not set desired output, default to all outputs.
                existing_outputs = outputs
            }

            let selected_backend;

            if let Some(backend_name) = args.backend { // find the backend user wants
                match get_backend_by_name(&backend_name) {
                    Some(available_backend) => selected_backend = &available_backend,
                    None => {
                        // todo!();
                        if backends_available.first().is_some() {
                            selected_backend = &backends_available.get(0).unwrap();
                        } else {
                            error!("Backend not found");
                        }
                    }
                }
            } else { // user did not provide backend param

            }
        }
        None => {
            error!("No subcommand provided");
        }
    }
}
