use std::path::PathBuf;

use log::{error, info};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Wry,
};
use teller::{
    handlers::world::{
        get_level_name, get_vault_id, is_minecraft_world, read_dat_file, recursive_world_search,
        GameType,
    },
    types::world::WorldData,
    utils::{calculate_dir_size, encode_image_to_base64},
};

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("folder_handler")
        .invoke_handler(tauri::generate_handler![
            check_path_for_save_folders,
            grab_local_worlds_list,
            open_world_in_explorer,
        ])
        .build()
}

#[tauri::command]
fn check_path_for_save_folders(path: PathBuf) -> Result<Vec<PathBuf>, String> {
    info!("Checking path for saves folder: {}", path.to_string_lossy());

    let mut save_folders = Vec::new();
    let max_depth = 6;

    recursive_world_search(&path, 0, max_depth, &mut save_folders)?;

    save_folders.sort();
    save_folders.dedup();

    Ok(save_folders)
}

#[tauri::command]
fn open_world_in_explorer(
    handle: tauri::AppHandle,
    world_id: &str,
    category: Option<&str>,
) -> Result<(), String> {
    let path_str = super::get_world_by_id(world_id, Some(true), category)?.to_string();
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

#[tauri::command]
fn grab_local_worlds_list(local_saves_path: PathBuf) -> Result<Vec<WorldData>, String> {
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
