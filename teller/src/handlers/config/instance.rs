use std::{
    env, fs,
    path::{Path, PathBuf},
};

use log::{error, info};

use crate::{handlers::config::get_config_folder, types::config::DirectorySettings};

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

pub fn get_local_directories_config<P: AsRef<Path>>(
    config_dir: P,
) -> Result<DirectorySettings, String> {
    let config_path = config_dir.as_ref().join("local-directories.json");

    info!("Loading config file at {:?}", config_path);

    let settings = match config::Config::builder()
        .add_source(config::File::from(config_path.clone()))
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

    Ok(parsed_settings)
}

pub fn create_local_directories_config(
    settings_data: DirectorySettings,
) -> Result<DirectorySettings, String> {
    let config_dir = get_config_folder();

    let config_path = config_dir.join("local-directories.json");

    info!("Creating directrories config file at {:?}", config_path);

    let settings = match config::Config::builder()
        .add_source(config::File::from_str(
            serde_json::to_string(&settings_data).unwrap().as_str(),
            config::FileFormat::Json,
        ))
        .build()
    {
        Ok(s) => s,
        Err(e) => {
            error!(
                "Could not load directories config file at {:?}: {:?}",
                config_path, e
            );
            return Err(format!(
                "Could not load directories config file at {:?}: {:?}",
                config_path, e
            ));
        }
    };

    let parsed_settings = match settings.try_deserialize::<DirectorySettings>() {
        Ok(s) => s,
        Err(e) => {
            error!(
                "Could not parse directories config file at {:?}: {:?}",
                config_path, e
            );
            return Err(format!(
                "Could not parse directories config file at {:?}: {:?}",
                config_path, e
            ));
        }
    };

    match fs::write(&config_path, serde_json::to_string(&settings_data).unwrap()) {
        Ok(_) => (),
        Err(e) => {
            error!(
                "Could not write directories config file at {:?}: {:?}",
                config_path, e
            );
            return Err(format!(
                "Could not write directories config file at {:?}: {:?}",
                config_path, e
            ));
        }
    }

    info!("Created directories config file at {:?}", config_path);

    Ok(parsed_settings)
}

pub fn update_local_directories_config(
    settings_data: DirectorySettings,
) -> Result<DirectorySettings, String> {
    let config_dir = get_config_folder();

    let config_path = config_dir.join("local-directories.json");

    info!("Updating directories config file at {:?}", config_path);

    let settings = match config::Config::builder()
        .add_source(config::File::from_str(
            serde_json::to_string(&settings_data).unwrap().as_str(),
            config::FileFormat::Json,
        ))
        .build()
    {
        Ok(s) => s,
        Err(e) => {
            error!(
                "Could not load directories config file at {:?}: {:?}",
                config_path, e
            );
            return Err(format!(
                "Could not load directories config file at {:?}: {:?}",
                config_path, e
            ));
        }
    };

    let parsed_settings = match settings.try_deserialize::<DirectorySettings>() {
        Ok(s) => s,
        Err(e) => {
            error!(
                "Could not parse directories config file at {:?}: {:?}",
                config_path, e
            );
            return Err(format!(
                "Could not parse directories config file at {:?}: {:?}",
                config_path, e
            ));
        }
    };

    match fs::write(&config_path, serde_json::to_string(&settings_data).unwrap()) {
        Ok(_) => (),
        Err(e) => {
            error!(
                "Could not write directories config file at {:?}: {:?}",
                config_path, e
            );
            return Err(format!(
                "Could not write directories config file at {:?}: {:?}",
                config_path, e
            ));
        }
    }

    info!("Updated directories config file at {:?}", config_path);

    Ok(parsed_settings)
}