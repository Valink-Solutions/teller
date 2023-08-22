use std::path::PathBuf;

use log::info;
use teller::world::recursive_world_search;

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
