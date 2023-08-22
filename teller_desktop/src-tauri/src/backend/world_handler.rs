use std::path::PathBuf;

use log::info;
use serde_json::Value;
use teller::{
    configuration::{get_config_folder, get_saves_config},
    world::{get_vault_id, get_world_data},
};

use crate::config::get_minecraft_save_location;

#[tauri::command]
pub fn get_world_by_id(world_id: &str) -> Result<Value, String> {
    let config_dir = get_config_folder();

    let saves_config = match get_saves_config(&config_dir) {
        Ok(config) => config,
        Err(e) => return Err(e),
    };

    let mut paths: Vec<PathBuf> = Vec::new();

    match get_minecraft_save_location() {
        Some(path) => paths.push(path),
        None => {}
    };

    saves_config.paths.iter().for_each(|(_, path)| {
        paths.push(PathBuf::from(path));
    });

    for save_location in paths {
        let world_folders = match std::fs::read_dir(&save_location) {
            Ok(folders) => folders,
            Err(_) => continue,
        };

        for entry in world_folders {
            if let Ok(world_folder) = entry {
                let world_folder = world_folder.path();

                if !world_folder.is_dir() {
                    continue;
                }

                let vault_id = match get_vault_id(&world_folder) {
                    Ok(id) => id,
                    Err(_) => continue,
                };

                if vault_id == world_id {
                    match get_world_data(&world_folder) {
                        Ok(data) => {
                            info!("Found world: {world_id}");
                            return Ok(data);
                        }
                        Err(_) => continue,
                    };
                }
            }
        }
    }

    Err("Could not find world".to_string())
}
