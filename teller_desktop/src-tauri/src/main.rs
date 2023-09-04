#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;
use std::path::PathBuf;

use commandblock::nbt::NbtValue;
use log::error;
use log::info;
use tauri_plugin_log::LogTarget;
use teller::configuration::get_config_folder;
use teller::utils::encode_image_to_base64;
use teller::utils::WorldData;
use teller::world::{
    calculate_dir_size, get_vault_id, is_minecraft_world, parse_dat_file, GameType,
};

fn get_level_name(
    level_dat_blob: NbtValue,
    game_type: GameType,
) -> Result<String, Box<dyn std::error::Error>> {
    let level_value: serde_json::Value = serde_json::to_value(level_dat_blob)?;

    match game_type {
        GameType::Java => {
            let level_data = match level_value.get("Data") {
                Some(data) => data,
                None => return Err("Could not find Data in level.dat".into()),
            };

            let level_name = match level_data.get("LevelName") {
                Some(name) => name.to_string(),
                None => return Err("Could not find LevelName in level.dat".into()),
            };

            let parsed_level_name = level_name[1..level_name.len() - 1].to_string();

            Ok(parsed_level_name)
        }
        GameType::Bedrock => {
            let level_name = match level_value.get("LevelName") {
                Some(name) => name.to_string(),
                None => return Err("Could not find levelName in level.dat".into()),
            };

            let parsed_level_name = level_name[1..level_name.len() - 1].to_string();

            Ok(parsed_level_name)
        }
        GameType::None => Err("Could not find game type".into()),
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
            let level_dat_blob = match parse_dat_file(level_dat_path, game_type) {
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

fn main() {
    let config_dir = get_config_folder();
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    LogTarget::Folder(config_dir),
                    LogTarget::Stdout,
                    LogTarget::Webview,
                ])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            grab_local_worlds_list,
            teller_desktop::backend::folder_handler::check_path_for_save_folders,
            teller_desktop::backend::folder_handler::open_world_in_explorer,
            teller_desktop::config::get_save_folders,
            teller_desktop::config::get_minecraft_save_location,
            teller_desktop::config::get_folder_path,
            teller_desktop::config::create_saves_config,
            teller_desktop::backend::world_handler::get_world_by_id,
            teller_desktop::backend::world_handler::grab_player_meta_from_uuids,
            teller_desktop::backend::world_handler::grab_player_meta_from_uuid,
            teller_desktop::backend::world_handler::grab_player_from_uuid,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
