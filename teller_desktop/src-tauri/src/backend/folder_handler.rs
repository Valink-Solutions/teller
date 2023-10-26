use std::path::PathBuf;

use log::{error, info};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Wry,
};
use teller::{
    handlers::search::worlds::{
        fetch_worlds_from_instance, get_world_path_by_id, recursive_world_search,
    },
    types::world::WorldData,
};

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("folder_handler")
        .invoke_handler(tauri::generate_handler![
            check_path_for_save_folders,
            grab_local_worlds_list,
            open_world_in_explorer,
            open_path_in_explorer,
        ])
        .build()
}

#[tauri::command]
async fn check_path_for_save_folders(path: PathBuf) -> Result<Vec<PathBuf>, String> {
    info!("Checking path for saves folder: {}", path.to_string_lossy());

    let mut save_folders = Vec::new();
    let max_depth = 6;

    recursive_world_search(&path, 0, max_depth, &mut save_folders).await?;

    save_folders.sort();
    save_folders.dedup();

    Ok(save_folders)
}

#[tauri::command]
async fn grab_local_worlds_list(category: &str, instance: &str) -> Result<Vec<WorldData>, String> {
    fetch_worlds_from_instance(category, instance).await
}

#[tauri::command]
async fn open_world_in_explorer(
    handle: tauri::AppHandle,
    world_id: &str,
    category: Option<&str>,
    instance: Option<&str>,
) -> Result<(), String> {
    let path = get_world_path_by_id(world_id, category, instance).await?;

    if path.is_dir() {
        match tauri::api::shell::open(&handle.shell_scope(), &path.to_string_lossy(), None)
            .map_err(|e| e.to_string())
        {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Could not open path: {}", e);
                Err(e.to_string())
            }
        }
    } else {
        Err("Path is not a valid directory".to_string())
    }
}

#[tauri::command]
fn open_path_in_explorer(handle: tauri::AppHandle, path: &str) -> Result<(), String> {
    let path_str = path.replace(" ", r" ").replace("\"", "");

    let path = PathBuf::from(path_str);

    if path.is_dir() {
        match tauri::api::shell::open(&handle.shell_scope(), &path.to_string_lossy(), None)
            .map_err(|e| e.to_string())
        {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Could not open path: {}", e);
                Err(e.to_string())
            }
        }
    } else {
        Err("Path is not a valid directory".to_string())
    }
}
