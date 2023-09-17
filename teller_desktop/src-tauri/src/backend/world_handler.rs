use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use log::{error, info};
use serde_json::Value;
use teller::{
    configuration::{get_config_folder, get_saves_config},
    utils::{
        player_handler::{fetch_player_data_from_uuid, fetch_players_meta_data},
        PlayerData,
    },
    world::{get_vault_id, is_minecraft_world, process_world_data},
};

use crate::config::get_minecraft_save_location;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Wry,
};

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("world_handler")
        .invoke_handler(tauri::generate_handler![
            get_world_by_id,
            grab_player_meta_from_uuids,
            grab_player_meta_from_uuid,
            grab_player_from_uuid
        ])
        .build()
}

#[tauri::command]
pub fn get_world_by_id(
    world_id: &str,
    return_path: Option<bool>,
    category: Option<&str>,
) -> Result<Value, String> {
    let config_dir = get_config_folder();

    info!("Searching for world: {}", world_id);

    let mut paths: Vec<PathBuf> = Vec::new();

    match get_saves_config(&config_dir) {
        Ok(config) => {
            if let Some(category) = category {
                if category == "default" {
                    match get_minecraft_save_location() {
                        Some(path) => paths.push(path),
                        None => {}
                    };
                } else if let Some(vault_entries) = config.categories.get(category) {
                    for (_, path) in vault_entries.paths.iter() {
                        paths.push(path.clone());
                    }
                }
            }
        }
        Err(_e) => {}
    };

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
                    let game_type = is_minecraft_world(&world_folder);

                    info!("Found world: {world_id}");

                    if let Some(true) = return_path {
                        return Ok(Value::String(world_folder.to_string_lossy().into_owned()));
                    } else {
                        match process_world_data(&world_folder, game_type) {
                            Ok(data) => {
                                let data_value = serde_json::to_value(data).unwrap();
                                return Ok(data_value);
                            }
                            Err(e) => {
                                error!("Could not process world data: {:?}", e);
                                continue;
                            }
                        };
                    }
                }
            }
        }
    }

    Err("Could not find world".to_string())
}

#[tauri::command]
fn grab_player_meta_from_uuids(
    player_data_list: Vec<PlayerData>,
) -> Result<HashMap<String, Value>, String> {
    info!("Fetching  {} player's meta data", player_data_list.len());

    let player_meta_map = match fetch_players_meta_data(player_data_list) {
        Ok(meta_map) => meta_map,
        Err(_) => return Err("Failed to fetch player meta data".into()),
    };

    Ok(player_meta_map)
}

#[tauri::command]
fn grab_player_meta_from_uuid(player_uuid: String) -> Result<Value, String> {
    info!("Fetching player meta data with UUID: {}", player_uuid);

    let player_meta_data = match fetch_player_data_from_uuid(player_uuid) {
        Ok(meta_data) => meta_data,
        Err(_) => return Err("Failed to fetch player meta data".into()),
    };

    Ok(player_meta_data)
}

#[tauri::command]
fn grab_player_from_uuid(player_uuid: String, path: &Path) -> Result<PlayerData, String> {
    info!("Fetching player data with UUID: {}", player_uuid);

    let game_type = is_minecraft_world(path);

    let player_data = match teller::utils::player_handler::grab_player_from_uuid(
        player_uuid,
        &path.to_path_buf(),
        game_type,
    ) {
        Ok(data) => data,
        Err(_) => return Err("Failed to fetch player data".into()),
    };

    Ok(player_data)
}
