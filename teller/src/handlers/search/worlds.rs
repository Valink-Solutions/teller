use std::path::{Path, PathBuf};

use async_recursion::async_recursion;
use log::{error, info};
use tokio::fs;

use crate::{
    handlers::{
        config::{
            get_config_folder,
            instance::{get_local_directories_config, get_minecraft_save_location},
        },
        world::{get_vault_id, parse_world_entry_data, process_world_data, GameType},
    },
    types::world::{WorldData, WorldLevelData},
};

pub async fn fetch_worlds_from_instance(
    selected_category: &str,
    instance: &str,
) -> Result<Vec<WorldData>, String> {
    let mut worlds_list: Vec<WorldData> = Vec::new();

    let config_dir = get_config_folder();

    let config = match get_local_directories_config(&config_dir) {
        Ok(config) => config,
        Err(e) => {
            error!("Could not get local directories config: {:?}", e);
            return Err("Could not get local directories config".to_string());
        }
    };
    let local_saves_path = if selected_category == "default" {
        match get_minecraft_save_location() {
            Some(path) => path,
            None => {
                error!("Could not find Minecraft save location");
                return Err("Could not find Minecraft save location".to_string());
            }
        }
    } else {
        match config.categories.get(selected_category) {
            Some(category) => match category.paths.get(instance) {
                Some(path) => path.to_owned(),
                None => {
                    error!(
                        "Could not find instance {} in category {}",
                        instance, selected_category
                    );
                    return Err("Could not find instance".to_string());
                }
            },
            None => {
                error!("Could not find category {}", selected_category);
                return Err("Could not find category".to_string());
            }
        }
    };

    info!("Grabbing local worlds list from {:?}", local_saves_path);

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
            match parse_world_entry_data(path.clone()).await {
                Ok(world_data) => worlds_list.push(world_data),
                Err(_) => continue,
            }
        }
    }

    Ok(worlds_list)
}

// pub fn world_path_from_id(world_id: &str, category: Option<&str>) -> Result<PathBuf, String> {
//     let config_dir = get_config_folder();

//     info!("Searching for world: {}", world_id);

//     let mut paths: Vec<PathBuf> = Vec::new();

//     match get_local_directories_config(&config_dir) {
//         Ok(config) => {
//             if let Some(category) = category {
//                 if category == "default" {
//                     match get_minecraft_save_location() {
//                         Some(path) => paths.push(path),
//                         None => {}
//                     };
//                 } else if let Some(vault_entries) = config.categories.get(category) {
//                     for (_, path) in vault_entries.paths.iter() {
//                         paths.push(path.clone());
//                     }
//                 }
//             }
//         }
//         Err(_e) => {}
//     };

//     for save_location in paths {
//         let world_folders = match std::fs::read_dir(&save_location) {
//             Ok(folders) => folders,
//             Err(_) => continue,
//         };

//         for entry in world_folders {
//             if let Ok(world_folder) = entry {
//                 let world_folder = world_folder.path();

//                 if !world_folder.is_dir() {
//                     continue;
//                 }

//                 let vault_id = match get_vault_id(&world_folder) {
//                     Ok(id) => id,
//                     Err(_) => continue,
//                 };

//                 if vault_id == world_id {
//                     info!("Found world: {world_id}");
//                     return Ok(world_folder);
//                 }
//             }
//         }
//     }

//     Err("Could not find world".to_string())
// }

pub async fn world_path_from_id(
    world_id: &str,
    category: Option<&str>,
    instance: Option<&str>,
) -> Result<PathBuf, String> {
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
            } else if let Some(instance) = instance {
                config.categories.iter().for_each(|(_, category)| {
                    if let Some(path) = category.paths.get(instance) {
                        paths.push(path.clone());
                    }
                });
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

                let vault_id = match get_vault_id(&world_folder).await {
                    Ok(id) => id,
                    Err(_) => continue,
                };

                if vault_id == world_id {
                    info!("Found world: {world_id}");
                    return Ok(world_folder);
                }
            }
        }
    }

    Err("Could not find world".to_string())
}

pub async fn grab_world_by_id(
    world_id: &str,
    category: Option<&str>,
    instance: Option<&str>,
) -> Result<WorldLevelData, String> {
    match world_path_from_id(world_id, category, instance).await {
        Ok(path) => {
            let game_type = is_minecraft_world(&path.clone());
            match process_world_data(&path, game_type).await {
                Ok(data) => Ok(data),
                Err(e) => {
                    error!("Could not process world data: {:?}", e);
                    Err("Could not process world data".to_string())
                }
            }
        }
        Err(e) => {
            error!("Could not find world: {:?}", e);
            return Err("Could not find world".to_string());
        }
    }
}

pub fn is_minecraft_world(path: &PathBuf) -> GameType {
    let path = path.as_path();

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

pub async fn is_minecraft_folder(path: &Path) -> GameType {
    if path.is_dir() {
        if path.file_name().unwrap() == ".minecraft" {
            if !path.join("saves").exists() {
                fs::create_dir_all(path.join("saves"))
                    .await
                    .expect("Failed to create saves directory");
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

#[async_recursion]
pub async fn recursive_world_search(
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

    match is_minecraft_world(&path.to_path_buf()) {
        GameType::Java => {
            save_folders.push(path.parent().unwrap().to_path_buf());
        }
        GameType::Bedrock => {
            save_folders.push(path.parent().unwrap().to_path_buf());
        }
        GameType::None => match is_minecraft_folder(path).await {
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
                                )
                                .await?;
                            }
                        }
                    }
                }
            }
        },
    }

    Ok(())
}
