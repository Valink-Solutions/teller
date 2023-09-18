use std::path::PathBuf;

use log::{error, info};
use serde_json::Value;

use crate::{
    handlers::{
        config::{get_config_folder, get_minecraft_save_location, get_saves_config},
        world::{
            get_level_name, get_vault_id, is_minecraft_world, process_world_data, read_dat_file,
            GameType,
        },
    },
    types::world::WorldData,
    utils::{calculate_dir_size, encode_image_to_base64},
};

pub fn fetch_worlds_from_path(local_saves_path: PathBuf) -> Result<Vec<WorldData>, String> {
    let mut worlds_list: Vec<WorldData> = Vec::new();

    info!("Grabbing local worlds list from {:?}", local_saves_path);

    let local_saves_path = PathBuf::from(local_saves_path);

    if !local_saves_path.exists() {
        error!(
            "Could not find Minecraft save location at {:?}",
            local_saves_path
        );

        return Err("Could not find Minecraft save location".to_string());
    }

    let entries = match local_saves_path.read_dir() {
        Ok(entries) => entries,
        Err(e) => {
            error!(
                "Could not read Minecraft save location at {:?}: {:?}",
                local_saves_path, e
            );

            return Err("Could not read Minecraft save location".to_string());
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        let path = entry.path();
        if path.is_dir() {
            let game_type = is_minecraft_world(&path);

            let level_dat_path = path.join("level.dat");
            let level_dat_blob = match read_dat_file(level_dat_path, game_type) {
                Ok(blob) => blob,
                Err(e) => {
                    error!("Could not parse level.dat at {:?}: {:?}", path, e);
                    continue;
                }
            };

            let level_name = match get_level_name(level_dat_blob, game_type) {
                Ok(name) => name,
                Err(e) => {
                    error!("Could not get level name at {:?}: {:?}", path, e);
                    continue;
                }
            };

            let world_size = match calculate_dir_size(&path) {
                Ok(size) => size,
                Err(_) => 0,
            };

            let vault_id = match get_vault_id(&path) {
                Ok(id) => id,
                Err(e) => {
                    error!("Could not get vault id at {:?}: {:?}", path, e);
                    continue;
                }
            };

            let world_data = WorldData {
                id: vault_id,
                name: level_name,
                image: match game_type {
                    GameType::Java => {
                        encode_image_to_base64(path.join("icon.png")).unwrap_or("".to_string())
                    }
                    GameType::Bedrock => encode_image_to_base64(path.join("world_icon.jpeg"))
                        .unwrap_or("".to_string()),
                    GameType::None => "".to_string(),
                },
                path: path.to_string_lossy().into_owned(),
                size: world_size,
            };

            worlds_list.push(world_data);
        }
    }

    Ok(worlds_list)
}

pub fn grab_world_by_id(
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
