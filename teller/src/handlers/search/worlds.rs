use std::{
    fs,
    path::{Path, PathBuf},
};

use log::{error, info};
use serde_json::Value;

use crate::{
    handlers::{
        config::{
            get_config_folder,
            instance::{get_local_directories_config, get_minecraft_save_location},
        },
        world::{get_vault_id, parse_world_entry_data, process_world_data, GameType},
    },
    types::world::WorldData,
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
        if path.is_dir() && path.extension().map_or(true, |ext| ext != "zip") {
            match parse_world_entry_data(path.clone()) {
                Ok(world_data) => worlds_list.push(world_data),
                Err(_) => continue,
            }
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

    match get_local_directories_config(&config_dir) {
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

pub fn is_minecraft_world(path: &Path) -> GameType {
    if !path.is_dir() {
        return GameType::None;
    }

    let java_files = ["level.dat", "region", "data"];
    let bedrock_files = ["level.dat", "db"];

    let is_java = java_files.iter().all(|file| path.join(file).exists());
    let is_bedrock = bedrock_files.iter().all(|file| path.join(file).exists());

    if is_java {
        info!("Found java world at {:?}", path);
        return GameType::Java;
    } else if is_bedrock {
        info!("Found bedrock world at {:?}", path);
        return GameType::Bedrock;
    } else {
        error!(
            "Could not determine if path is a minecraft world: {:?}",
            path
        );

        return GameType::None;
    }
}

pub fn is_minecraft_folder(path: &Path) -> GameType {
    if path.is_dir() {
        if path.file_name().unwrap() == ".minecraft" {
            if !path.join("saves").exists() {
                fs::create_dir_all(path.join("saves")).expect("Failed to create saves directory");
            }
            return GameType::Java;
        } else if path.join("minecraftWorlds").exists() {
            return GameType::Bedrock;
        }
    }

    error!(
        "Could not determine if path is a minecraft folder: {:?}",
        path
    );

    GameType::None
}

pub fn recursive_world_search(
    path: &Path,
    depth: usize,
    max_depth: usize,
    save_folders: &mut Vec<PathBuf>,
) -> Result<(), String> {
    if depth > max_depth {
        return Ok(());
    }

    if !path.exists() {
        return Err(format!("Path {:?} does not exist", path));
    }

    if path.ends_with("node_modules") || path.extension().map_or(false, |ext| ext == "zip") {
        return Ok(());
    }

    match is_minecraft_world(path) {
        GameType::Java => {
            save_folders.push(path.parent().unwrap().to_path_buf());
        }
        GameType::Bedrock => {
            save_folders.push(path.parent().unwrap().to_path_buf());
        }
        GameType::None => match is_minecraft_folder(path) {
            GameType::Java => {
                save_folders.push(path.join("saves"));
            }
            GameType::Bedrock => {
                save_folders.push(path.join("minecraftWorlds"));
            }
            GameType::None => {
                if let Ok(entries) = path.read_dir() {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let entry_path = entry.path();
                            if entry_path.is_dir() {
                                recursive_world_search(
                                    &entry_path,
                                    depth + 1,
                                    max_depth,
                                    save_folders,
                                )?;
                            }
                        }
                    }
                }
            }
        },
    }

    Ok(())
}
