use std::path::PathBuf;

use log::error;
use teller::{
    handlers::{
        config::{
            get_config_folder,
            instance::{
                create_local_directories_config, get_local_directories_config,
                update_local_directories_config,
            },
        },
        search::directories::get_directory_by_name,
    },
    types::{backup::BackupSettings, config::DirectorySettings},
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
            get_minecraft_save_location,
            get_backup_settings,
            update_backup_settings
        ])
        .build()
}

#[tauri::command]
fn get_save_folders(handle: tauri::AppHandle) -> Result<DirectorySettings, String> {
    let config_dir = get_config_folder();

    // This simply opens the window and errors out allowing the user to configure the directories
    let saves_config = match get_local_directories_config(config_dir) {
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
fn load_saves_folders() -> Result<DirectorySettings, String> {
    let config_dir = get_config_folder();

    let saves_config = match get_local_directories_config(config_dir) {
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
async fn create_saves_config(
    settings_data: DirectorySettings,
) -> Result<DirectorySettings, String> {
    create_local_directories_config(settings_data).await
}

#[tauri::command]
async fn update_saves_config(
    settings_data: DirectorySettings,
) -> Result<DirectorySettings, String> {
    update_local_directories_config(settings_data).await
}

#[tauri::command]
pub fn get_minecraft_save_location() -> Option<PathBuf> {
    teller::handlers::config::instance::get_minecraft_save_location()
}

#[tauri::command]
async fn get_backup_settings() -> Result<BackupSettings, String> {
    teller::handlers::config::backup::get_backup_config().await
}

#[tauri::command]
async fn update_backup_settings(settings_data: BackupSettings) -> Result<BackupSettings, String> {
    teller::handlers::config::backup::update_backup_config(settings_data).await
}
