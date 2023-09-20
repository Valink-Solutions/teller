use std::path::PathBuf;

use log::{error, info};

use crate::handlers::config::{
    get_config_folder, get_local_directories_config, get_minecraft_save_location,
};

pub fn get_directory_by_name(dir_name: &str, category: Option<&str>) -> Option<PathBuf> {
    info!("Getting path for {}", dir_name);

    match dir_name == "default" {
        true => return get_minecraft_save_location(),
        false => (),
    }

    let config_dir = get_config_folder();

    let saves_config = match get_local_directories_config(&config_dir) {
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
