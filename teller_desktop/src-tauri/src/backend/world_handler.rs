use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use log::info;
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

#[tauri::command]
pub fn get_world_by_id(world_id: &str, return_path: Option<bool>) -> Result<Value, String> {
    let config_dir = get_config_folder();

    let mut paths: Vec<PathBuf> = Vec::new();

    match get_minecraft_save_location() {
        Some(path) => paths.push(path),
        None => {}
    };

    match get_saves_config(&config_dir) {
        Ok(config) => {
            config.paths.iter().for_each(|(_, path)| {
                paths.push(PathBuf::from(path));
            });
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

                    if let Some(true) = return_path {
                        return Ok(Value::String(world_folder.to_string_lossy().into_owned()));
                    } else {
                        match process_world_data(&world_folder, game_type) {
                            Ok(data) => {
                                info!("Found world: {world_id}");
                                let data_value = serde_json::to_value(data).unwrap();
                                return Ok(data_value);
                            }
                            Err(_) => continue,
                        };
                    }
                }
            }
        }
    }

    Err("Could not find world".to_string())
}

#[tauri::command]
pub fn grab_player_meta_from_uuids(
    player_data_list: Vec<PlayerData>,
) -> Result<HashMap<String, Value>, String> {
    let player_meta_map = match fetch_players_meta_data(player_data_list) {
        Ok(meta_map) => meta_map,
        Err(_) => return Err("Failed to fetch player meta data".into()),
    };

    Ok(player_meta_map)
}

#[tauri::command]
pub fn grab_player_meta_from_uuid(player_uuid: String) -> Result<Value, String> {
    let player_meta_data = match fetch_player_data_from_uuid(player_uuid) {
        Ok(meta_data) => meta_data,
        Err(_) => return Err("Failed to fetch player meta data".into()),
    };

    info!("Player meta data: {:?}", player_meta_data);

    Ok(player_meta_data)
}

#[tauri::command]
pub fn grab_player_from_uuid(player_uuid: String, path: &Path) -> Result<PlayerData, String> {
    let game_type = is_minecraft_world(path);

    let player_data = match teller::utils::player_handler::grab_player_from_uuid(
        player_uuid,
        &path.to_path_buf(),
        game_type,
    ) {
        Ok(data) => data,
        Err(_) => return Err("Failed to fetch player data".into()),
    };

    info!("Player data: {:?}", player_data);

    Ok(player_data)
}
