use std::path::PathBuf;

use log::{error, info};
use tauri::Manager;
use teller::world::recursive_world_search;

use crate::backend::get_world_by_id;

#[tauri::command]
pub fn check_path_for_save_folders(path: PathBuf) -> Result<Vec<PathBuf>, String> {
    info!("Checking path for saves folder: {}", path.to_string_lossy());

    let mut save_folders = Vec::new();
    let max_depth = 6;

    recursive_world_search(&path, 0, max_depth, &mut save_folders)?;

    save_folders.sort();
    save_folders.dedup();

    Ok(save_folders)
}

pub fn create_worlds_database() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn open_world_in_explorer(
    handle: tauri::AppHandle,
    world_id: &str,
    category: Option<&str>,
) -> Result<(), String> {
    let path_str = get_world_by_id(world_id, Some(true), category)?.to_string();
    let path_str = path_str.replace(" ", r" ").replace("\"", "");

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
