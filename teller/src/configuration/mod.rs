use std::fs;
use std::path::Path;

use log::{error, info};

use std::collections::HashMap;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct DirectorySettings {
    pub categories: HashMap<String, VaultEntries>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct VaultEntries {
    pub paths: HashMap<String, PathBuf>,
}

pub fn get_saves_config<P: AsRef<Path>>(config_dir: P) -> Result<DirectorySettings, String> {
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

pub fn get_config_folder() -> PathBuf {
    let config_dir = directories::ProjectDirs::from("io", "valink", "teller");

    let config_dir = config_dir.unwrap().config_dir().to_path_buf();

    // check if config directory exists
    if !config_dir.exists() {
        info!("Creating config folder at {:?}", config_dir);

        match fs::create_dir_all(&config_dir) {
            Ok(_) => (),
            Err(e) => {
                error!(
                    "Could not create config directory at {:?}: {:?}",
                    config_dir, e
                );
                return config_dir;
            }
        }
    }

    config_dir
}
