use std::path::PathBuf;


use log::{info, error};
use tauri::Manager;
use teller::world::recursive_world_search;

use crate::backend::get_world_by_id;

#[tauri::command]
pub fn check_path_for_save_folders(path: PathBuf) -> Result<Vec<PathBuf>, String> {
    let mut save_folders = Vec::new();
    let max_depth = 6;

    recursive_world_search(&path, 0, max_depth, &mut save_folders)?;

    save_folders.sort();
    save_folders.dedup();

    info!("Found Minecraft worlds: {:?}", save_folders);

    Ok(save_folders)
}

pub fn create_worlds_database() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn open_world_in_explorer(handle: tauri::AppHandle, world_id: &str) -> Result<(), String> {

    let path = get_world_by_id(world_id, Some(true))?.to_string();

    info!("Opening path: {}", path);
    

    match tauri::api::shell::open(&handle.shell_scope(), &path, None).map_err(|e| e.to_string()) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Could not open path: {}", e);
            Err(e.to_string())
        }
    }

}
