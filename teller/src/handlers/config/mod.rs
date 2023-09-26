pub mod backup;
pub mod instance;

use std::{fs, path::PathBuf};

use log::{error, info};

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
