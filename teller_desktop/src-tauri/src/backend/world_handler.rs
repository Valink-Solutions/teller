use std::{collections::HashMap, path::PathBuf};

use log::info;
use serde_json::Value;
use teller::{
    configuration::{get_config_folder, get_saves_config},
    utils::{player_handler::fetch_players_meta_data, PlayerData},
    world::{get_vault_id, is_minecraft_world, process_world_data},
};
use uuid::Uuid;

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
                    let game_type = is_minecraft_world(&world_folder);

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

    Err("Could not find world".to_string())
}

#[tauri::command]
pub fn grab_player_meta_from_uuids(
    player_data_list: Vec<PlayerData>,
) -> Result<HashMap<Uuid, Value>, String> {
    let player_meta_map = match fetch_players_meta_data(player_data_list) {
        Ok(meta_map) => meta_map,
        Err(_) => return Err("Failed to fetch player data".into()),
    };

    Ok(player_meta_map)
}
