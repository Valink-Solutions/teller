use log::{error, info};
use std::collections::HashMap;
use std::io::{Read, Seek, Write};
use std::path::Path;
use std::{fs::File, path::PathBuf};
use zip::write::FileOptions;
use zip::ZipWriter;

use serde_json::json;

use crate::types::backup::BackupMetadata;

use super::config::backup::get_backup_config;
use super::search::worlds::grab_world_by_id;
use super::world::parse_world_entry_data;
use super::{
    config::get_config_folder, search::worlds::is_minecraft_world, world::process_world_data,
};

pub fn create_backup_from_id(
    world_id: &str,
    category: Option<&str>,
    vaults: Option<Vec<String>>,
) -> Result<String, String> {
    info!("Creating backup for world id: {}", world_id);
    match grab_world_by_id(world_id, Some(true), category) {
        Ok(value) => {
            let world_path = value.as_str().unwrap();
            let world_backup_path = match create_world_backup(PathBuf::from(world_path)) {
                Ok(backup_path) => backup_path,
                Err(e) => {
                    error!(
                        "Failed to create backup for world folder {}: {:?}",
                        world_path, e
                    );
                    return Err(format!(
                        "Failed to create backup for world folder {}: {:?}",
                        world_path, e
                    ));
                }
            };

            let backup_name = match world_backup_path.file_name() {
                Some(name) => name,
                None => {
                    error!(
                        "Could not get backup name from path: {:?}",
                        world_backup_path
                    );
                    return Err(format!(
                        "Could not get backup name from path: {:?}",
                        world_backup_path
                    ));
                }
            };

            if vaults.is_some() {
                let mut vault_locations = HashMap::new();

                let backup_settings = get_backup_config()?;

                for vault_id in vaults.unwrap() {
                    if let Some(vault) = backup_settings.vaults.get(&vault_id) {
                        vault_locations.insert(vault_id, vault);
                    }
                }

                info!("Copying backup to all {} vaults", vault_locations.len());

                for (vault_id, vault_path) in vault_locations {
                    let backup_location = vault_path.join(world_id);
                    if !backup_location.exists() {
                        match std::fs::create_dir_all(&backup_location) {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Failed to create vault folder {}: {:?}", vault_id, e);
                                continue;
                            }
                        };
                    }
                    match std::fs::copy(
                        &world_backup_path,
                        backup_location.join(backup_name.clone()),
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            error!(
                                "Failed to move backup to vault folder {}: {:?}",
                                vault_id, e
                            );
                            continue;
                        }
                    };
                }

                if let Err(e) = std::fs::remove_file(&world_backup_path) {
                    error!(
                        "Failed to remove backup file {}: {:?}",
                        world_backup_path.display(),
                        e
                    );
                    return Err(format!(
                        "Failed to remove backup file {}: {:?}",
                        world_backup_path.display(),
                        e
                    ));
                }
            } else {
                let default_vault = get_default_vault();
                match std::fs::rename(&world_backup_path, default_vault.join(backup_name)) {
                    Ok(_) => {}
                    Err(e) => {
                        error!(
                            "Failed to move backup to default vault {}: {:?}",
                            default_vault.display(),
                            e
                        );
                        return Err(format!(
                            "Failed to move backup to default vault {}: {:?}",
                            default_vault.display(),
                            e
                        ));
                    }
                };
            }
            Ok("Successfully Createad Backup.".to_string())
        }
        Err(e) => {
            error!("Failed to grab world by id {}: {:?}", world_id, e);
            return Err(format!("Failed to grab world by id {}: {:?}", world_id, e));
        }
    }
}

pub fn create_world_backup(world_path: PathBuf) -> Result<PathBuf, String> {
    let default_vault = get_default_vault();

    let temp_dir = match default_vault.join("temp").exists() {
        true => default_vault.join("temp"),
        false => {
            let _ = std::fs::create_dir_all(default_vault.join("temp"));
            default_vault.join("temp")
        }
    };

    let mut world_entry_data = parse_world_entry_data(world_path.clone())?;

    world_entry_data.path = "".to_string();

    let game_type = is_minecraft_world(&world_path);

    let world_data = match process_world_data(&world_path, game_type) {
        Ok(data) => data,
        Err(e) => {
            return Err(format!("Could not process world data: {:?}", e));
        }
    };

    let metadata = json!({
        "entry": world_entry_data,
        "data": world_data,
    });

    let backup_id = format!("{}.chunkvault-snapshot", chrono::Utc::now().timestamp());
    let backup_path = temp_dir.join(backup_id);

    let world_zip_path = temp_dir.join(format!("{}_data.zip", world_entry_data.id));
    let mut world_zip = zip::ZipWriter::new(std::fs::File::create(&world_zip_path).unwrap());
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);
    add_directory_to_zip(&mut world_zip, &world_path, &world_path, &options).unwrap();
    world_zip.finish().unwrap();

    let mut zip = zip::ZipWriter::new(std::fs::File::create(backup_path.clone()).unwrap());
    zip.start_file(
        "metadata.json",
        FileOptions::default().compression_method(zip::CompressionMethod::Stored),
    )
    .unwrap();
    zip.write_all(metadata.to_string().as_bytes()).unwrap();

    let mut world_zip_file = File::open(&world_zip_path).unwrap();
    let mut buffer = Vec::new();
    world_zip_file.read_to_end(&mut buffer).unwrap();
    zip.start_file(
        format!("{}_data.zip", world_entry_data.id),
        FileOptions::default().compression_method(zip::CompressionMethod::Stored),
    )
    .unwrap();
    zip.write_all(&buffer).unwrap();

    zip.finish().unwrap();

    std::fs::remove_file(&world_zip_path).unwrap();

    Ok(backup_path)
}

fn add_directory_to_zip<W: Write + Seek>(
    zip_writer: &mut ZipWriter<W>,
    directory: &Path,
    prefix: &Path,
    options: &FileOptions,
) -> zip::result::ZipResult<()> {
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();
        if path.is_file() {
            zip_writer.start_file(name.to_string_lossy().as_ref(), *options)?;
            let mut f = File::open(&path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip_writer.write_all(&buffer)?;
        } else if path.is_dir() {
            add_directory_to_zip(zip_writer, &path, prefix, options)?;
        }
    }
    Ok(())
}

fn get_default_vault() -> PathBuf {
    let config_dir = get_config_folder();

    let vault_dir = config_dir.join("vault");

    match vault_dir.exists() {
        true => vault_dir,
        false => {
            let _ = std::fs::create_dir_all(vault_dir.clone());
            vault_dir
        }
    }
}

pub fn grab_backup_metadata(backup_path: PathBuf) -> Result<BackupMetadata, String> {
    let mut zip = match zip::ZipArchive::new(File::open(backup_path.clone()).unwrap()) {
        Ok(zip) => zip,
        Err(e) => {
            return Err(format!(
                "Failed to open backup file {}: {:?}",
                backup_path.display(),
                e
            ));
        }
    };

    let mut metadata = String::new();

    let mut metadata_file = match zip.by_name("metadata.json") {
        Ok(file) => file,
        Err(e) => {
            return Err(format!(
                "Failed to open metadata file in backup {}: {:?}",
                backup_path.display(),
                e
            ));
        }
    };

    match metadata_file.read_to_string(&mut metadata) {
        Ok(_) => {}
        Err(e) => {
            return Err(format!(
                "Failed to read metadata file in backup {}: {:?}",
                backup_path.display(),
                e
            ));
        }
    };

    let metadata: BackupMetadata = match serde_json::from_str(&metadata) {
        Ok(metadata) => metadata,
        Err(e) => {
            return Err(format!(
                "Failed to parse metadata file in backup {}: {:?}",
                backup_path.display(),
                e
            ));
        }
    };

    Ok(metadata)
}
