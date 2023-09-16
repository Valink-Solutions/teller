use std::{env, fs, path::PathBuf};

use log::{error, info};
use teller::configuration::{get_config_folder, get_saves_config, DirectorySettings};

#[tauri::command]
pub async fn get_save_folders(handle: tauri::AppHandle) -> Result<DirectorySettings, String> {
    let config_dir = get_config_folder();

    // This simply opens the window and errors out allowing the user to configure the directories
    let saves_config = match get_saves_config(&config_dir) {
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
pub async fn load_saves_folders() -> Result<DirectorySettings, String> {
    let config_dir = get_config_folder();

    let saves_config = match get_saves_config(&config_dir) {
        Ok(s) => s,
        Err(e) => {
            return Err(format!("Could not get saves config: {:?}", e));
        }
    };

    Ok(saves_config)
}

#[tauri::command]
pub fn get_folder_path(dir_name: &str, category: Option<&str>) -> Option<PathBuf> {
    info!("Getting path for {}", dir_name);

    match dir_name == "default" {
        true => return get_minecraft_save_location(),
        false => (),
    }

    let config_dir = get_config_folder();

    let saves_config = match get_saves_config(&config_dir) {
        Ok(s) => s,
        Err(e) => {
            error!("Could not get saves config: {:?}", e);
            return None;
        }
    };

    match category {
        Some(category) => {
            if let Some(vault_entries) = saves_config.categories.get(category) {
                if let Some(path) = vault_entries.paths.get(dir_name) {
                    return Some(path.clone());
                }
            }
        }
        None => {
            for (_category, vault_entries) in saves_config.categories.iter() {
                if let Some(path) = vault_entries.paths.get(dir_name) {
                    return Some(path.clone());
                }
            }
        }
    }

    None
}

#[tauri::command]
pub fn create_saves_config(settings_data: DirectorySettings) -> Result<DirectorySettings, String> {
    let config_dir = get_config_folder();

    let config_path = config_dir.join("local-directories.json");

    info!("Creating config file at {:?}", config_path);

    let settings = match config::Config::builder()
        .add_source(config::File::from_str(
            serde_json::to_string(&settings_data).unwrap().as_str(),
            config::FileFormat::Json,
        ))
        .build()
    {
        Ok(s) => s,
        Err(e) => {
            error!("Could not load config file at {:?}: {:?}", config_path, e);
            return Err(format!(
                "Could not load config file at {:?}: {:?}",
                config_path, e
            ));
        }
    };

    let parsed_settings = match settings.try_deserialize::<DirectorySettings>() {
        Ok(s) => s,
        Err(e) => {
            error!("Could not parse config file at {:?}: {:?}", config_path, e);
            return Err(format!(
                "Could not parse config file at {:?}: {:?}",
                config_path, e
            ));
        }
    };

    match fs::write(&config_path, serde_json::to_string(&settings_data).unwrap()) {
        Ok(_) => (),
        Err(e) => {
            error!("Could not write config file at {:?}: {:?}", config_path, e);
            return Err(format!(
                "Could not write config file at {:?}: {:?}",
                config_path, e
            ));
        }
    }

    info!("Created config file at {:?}", config_path);

    Ok(parsed_settings)
}

#[tauri::command]
pub fn update_saves_config(settings_data: DirectorySettings) -> Result<DirectorySettings, String> {
    let config_dir = get_config_folder();

    let config_path = config_dir.join("local-directories.json");

    info!("Updating config file at {:?}", config_path);

    let settings = match config::Config::builder()
        .add_source(config::File::from_str(
            serde_json::to_string(&settings_data).unwrap().as_str(),
            config::FileFormat::Json,
        ))
        .build()
    {
        Ok(s) => s,
        Err(e) => {
            error!("Could not load config file at {:?}: {:?}", config_path, e);
            return Err(format!(
                "Could not load config file at {:?}: {:?}",
                config_path, e
            ));
        }
    };

    let parsed_settings = match settings.try_deserialize::<DirectorySettings>() {
        Ok(s) => s,
        Err(e) => {
            error!("Could not parse config file at {:?}: {:?}", config_path, e);
            return Err(format!(
                "Could not parse config file at {:?}: {:?}",
                config_path, e
            ));
        }
    };

    match fs::write(&config_path, serde_json::to_string(&settings_data).unwrap()) {
        Ok(_) => (),
        Err(e) => {
            error!("Could not write config file at {:?}: {:?}", config_path, e);
            return Err(format!(
                "Could not write config file at {:?}: {:?}",
                config_path, e
            ));
        }
    }

    info!("Updated config file at {:?}", config_path);

    Ok(parsed_settings)
}

#[tauri::command]
pub fn get_minecraft_save_location() -> Option<PathBuf> {
    let os = env::consts::OS;

    match os {
        "windows" => Some(PathBuf::from(format!(
            "{}\\.minecraft\\saves",
            env::var("APPDATA").unwrap()
        ))),
        "macos" => Some(PathBuf::from(format!(
            "{}/Library/Application Support/minecraft/saves",
            env::var("HOME").unwrap()
        ))),
        "linux" => Some(PathBuf::from(format!(
            "{}/.minecraft/saves",
            env::var("HOME").unwrap()
        ))),
        _ => None,
    }
}
