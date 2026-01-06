use std::path::{Path, PathBuf};
use log::{info, error, trace};
use rand::prelude::IndexedRandom;
use crate::backend::{BackendSpawnSpec, WallpaperMode, select_backend, available_backends};
use crate::wl::{OutputInfo, get_info};

/// Filter outputs by names. If target_names is None, returns all outputs.
pub fn filter_outputs_by_names(
    outputs: Vec<OutputInfo>,
    target_names: Option<Vec<String>>,
) -> Vec<OutputInfo> {
    match target_names {
        Some(names) => {
            outputs.into_iter()
                .filter(|output| names.contains(&output.name))
                .collect()
        }
        None => outputs,
    }
}

/// Create spawn specs for given outputs, media path, and mode.
pub fn create_spawn_specs(
    outputs: Vec<OutputInfo>,
    media_path: Vec<impl AsRef<Path>>,
    mode: WallpaperMode,
) -> Vec<BackendSpawnSpec> {
    
    outputs.into_iter()
        .zip(media_path.into_iter())
        .into_iter().map(
        |(output, media_path)| {
            BackendSpawnSpec {
                media: media_path.as_ref().to_path_buf(),
                mode: mode.clone(),
                output,
                extra_args: vec![],
            }
        }).collect()
}

/// Orchestrate wallpaper setup.
/// Returns a vector of child processes if successful.
pub async fn setup_wallpaper(
    media_path: Vec<PathBuf>,
    backend_name: Option<String>,
    target_outputs: Option<Vec<String>>,
    mode: WallpaperMode,
) -> Result<Vec<tokio::process::Child>, String> {
    // Get outputs and backends
    let (all_outputs, _) = get_info();
    let available = available_backends();

    if available.is_empty() {
        return Err("No available backend found".to_string());
    }

    let rng = &mut rand::rng();
    // Filter outputs
    let selected_outputs = filter_outputs_by_names(all_outputs, target_outputs);
    if selected_outputs.is_empty() {
        return Err("No outputs selected".to_string());
    }

    let selected_media = media_path.choose_multiple(rng, selected_outputs.len()).collect();

    // Select backend
    let backend = select_backend(backend_name, available);

    // Create spawn specs
    let spawn_specs = create_spawn_specs(
        selected_outputs,
        selected_media,
        mode
    );

    // Start backend(s)
    let mut children = Vec::new();
    if backend.capabilities().contains(&crate::backend::BackendCapability::MultiOutput) {
        info!("Calling start_multi for {}", backend.name());
        match backend.start_multi(spawn_specs) {
            Ok(mut cs) => {
                children.append(&mut cs);
                Ok(children)
            }
            Err(e) => Err(format!("Failed to start multi: {}", e)),
        }
    } else {
        info!("Calling start for {}", backend.name());
        for spec in spawn_specs {
            match backend.start(&spec) {
                Ok(c) => {
                    children.push(c);
                    info!("Spawned client {}, with media in {:?}", backend.name(), spec.media);
                }
                Err(e) => {
                    error!("Spawn failed: {}", e);
                    return Err(format!("Failed to start backend: {}", e));
                }
            }
        }
        Ok(children)
    }
}
