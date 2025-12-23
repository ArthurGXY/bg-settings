mod utils;

use bg_core::backend::*;

use log::{info, error};
use env_logger;
use clap::Parser;

use std::env;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    media_path: PathBuf,
    output: Option<String>,
}

fn detect_backends() {
    info!("Detecting backends");
    for backend in Backend::available_backends() {
        if backend.exists() {
            info!("Found backend {:?}", backend.name());
        } else {
            error!("No backend {:?} found", backend.name());
        }
    }
}

fn main() {
    unsafe { env::set_var("RUST_LOG", "info"); }
    env_logger::init();

    let args = Cli::parse();
    println!("{:?}", args);

    // println!("Hello, world!");
    detect_backends();
}
