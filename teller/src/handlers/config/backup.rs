use std::fs;

use log::{error, info};

use crate::{handlers::config::get_config_folder, types::backup::BackupSettings};

pub fn update_backup_config(settings_data: BackupSettings) -> Result<BackupSettings, String> {
    let config_dir = get_config_folder();

    let config_path = config_dir.join("backup_settings.json");

    info!("Updating backup config file at {:?}", config_path);

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
                "Could not load backup config file at {:?}: {:?}",
                config_path, e
            );
            return Err(format!(
                "Could not load backup config file at {:?}: {:?}",
                config_path, e
            ));
        }
    };

    let parsed_settings = match settings.try_deserialize::<BackupSettings>() {
        Ok(s) => s,
        Err(e) => {
            error!(
                "Could not parse backup config file at {:?}: {:?}",
                config_path, e
            );
            return Err(format!(
                "Could not parse backup config file at {:?}: {:?}",
                config_path, e
            ));
        }
    };

    match fs::write(&config_path, serde_json::to_string(&settings_data).unwrap()) {
        Ok(_) => (),
        Err(e) => {
            error!(
                "Could not write backup config file at {:?}: {:?}",
                config_path, e
            );
            return Err(format!(
                "Could not write backup config file at {:?}: {:?}",
                config_path, e
            ));
        }
    }

    info!("Updated backup config file at {:?}", config_path);

    Ok(parsed_settings)
}

pub fn get_backup_config() -> Result<BackupSettings, String> {
    let config_dir = get_config_folder();

    let config_path = config_dir.join("backup_settings.json");

    if !config_path.exists() {
        let default_settings = BackupSettings::default();
        match update_backup_config(default_settings) {
            Ok(settings) => return Ok(settings),
            Err(e) => {
                error!(
                    "Could not create backup config file at {:?}: {:?}",
                    config_path, e
                );
                return Err(format!(
                    "Could not create backup config file at {:?}: {:?}",
                    config_path, e
                ));
            }
        }
    }

    info!("Loading backup config file at {:?}", config_path);

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

    let parsed_settings = match settings.try_deserialize::<BackupSettings>() {
        Ok(s) => s,
        Err(e) => {
            error!(
                "Could not parse backup config file at {:?}: {:?}",
                config_path, e
            );
            return Err(format!(
                "Could not parse backup config file at {:?}: {:?}",
                config_path, e
            ));
        }
    };

    info!("Loaded backup config file at {:?}", config_path);

    Ok(parsed_settings)
}
