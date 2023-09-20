use std::path::PathBuf;

use log::error;
use teller::{
    handlers::{
        config::{
            create_local_directories_config, get_config_folder, get_local_directories_config,
            update_local_directories_config,
        },
        search::directories::get_directory_by_name,
    },
    types::config::DirectorySettings,
};

use tauri::{
    plugin::{Builder, TauriPlugin},
    Wry,
};

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("config")
        .invoke_handler(tauri::generate_handler![
            get_save_folders,
            load_saves_folders,
            get_folder_path,
            create_saves_config,
            update_saves_config,
            get_minecraft_save_location
        ])
        .build()
}

#[tauri::command]
async fn get_save_folders(handle: tauri::AppHandle) -> Result<DirectorySettings, String> {
    let config_dir = get_config_folder();

    // This simply opens the window and errors out allowing the user to configure the directories
    let saves_config = match get_local_directories_config(&config_dir) {
        Ok(s) => s,
        Err(e) => {
            let _config_saves_window = tauri::WindowBuilder::new(
                &handle,
                "configure-saves-directories",
                tauri::WindowUrl::App("config/setDirs".into()),
            )
            .build()
            .unwrap();

            error!("Could not get saves config: {:?}", e);
            return Err(format!("Could not get saves config: {:?}", e));
        }
    };

    Ok(saves_config)
}

#[tauri::command]
async fn load_saves_folders() -> Result<DirectorySettings, String> {
    let config_dir = get_config_folder();

    let saves_config = match get_local_directories_config(&config_dir) {
        Ok(s) => s,
        Err(e) => {
            return Err(format!("Could not get saves config: {:?}", e));
        }
    };

    Ok(saves_config)
}

#[tauri::command]
fn get_folder_path(dir_name: &str, category: Option<&str>) -> Option<PathBuf> {
    get_directory_by_name(dir_name, category)
}

#[tauri::command]
fn create_saves_config(settings_data: DirectorySettings) -> Result<DirectorySettings, String> {
    create_local_directories_config(settings_data)
}

#[tauri::command]
fn update_saves_config(settings_data: DirectorySettings) -> Result<DirectorySettings, String> {
    update_local_directories_config(settings_data)
}

#[tauri::command]
pub fn get_minecraft_save_location() -> Option<PathBuf> {
    teller::handlers::config::get_minecraft_save_location()
}
