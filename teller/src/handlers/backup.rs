use log::{error, info};
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

use async_recursion::async_recursion;
use async_zip::error::ZipError;
use async_zip::tokio::read::seek::ZipFileReader;
use async_zip::tokio::write::ZipFileWriter;
use async_zip::{Compression, ZipEntryBuilder};
use tokio::fs::File;

use serde_json::json;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncSeek;
use tokio::io::AsyncWrite;

use crate::handlers::config::backup::get_backup_config;
use crate::handlers::search::worlds::get_world_path_by_id;
use crate::types::backup::BackupMetadata;

use super::config::get_config_folder;
use super::search::worlds::is_minecraft_world;
use super::world::{parse_world_entry_data, process_world_data};

async fn get_default_vault() -> PathBuf {
    let config_dir = get_config_folder();

    let vault_dir = config_dir.join("vault");

    match vault_dir.exists() {
        true => vault_dir,
        false => {
            let _ = tokio::fs::create_dir_all(vault_dir.clone()).await;
            vault_dir
        }
    }
}

pub async fn create_world_backup(world_path: PathBuf) -> Result<PathBuf, String> {
    let default_vault = get_default_vault().await;

    let temp_dir = match default_vault.join("temp").exists() {
        true => default_vault.join("temp"),
        false => {
            let _ = tokio::fs::create_dir_all(default_vault.join("temp")).await;
            default_vault.join("temp")
        }
    };

    let mut world_entry_data = parse_world_entry_data(world_path.clone()).await?;

    world_entry_data.path = "".to_string();

    let game_type = is_minecraft_world(&world_path);

    let world_data = match process_world_data(&world_path, game_type).await {
        Ok(data) => data,
        Err(e) => {
            return Err(format!("Could not process world data: {:?}", e));
        }
    };

    let metadata = json!({
        "entry": world_entry_data,
        "data": world_data,
    });

    info!("Creating backup for world {}", world_entry_data.id);

    let backup_id = format!("{}.chunkvault-snapshot", chrono::Utc::now().timestamp());
    let backup_path = temp_dir.join(backup_id);

    let mut zip = ZipFileWriter::with_tokio(File::create(backup_path.clone()).await.unwrap());

    let meta_builder = ZipEntryBuilder::new("metadata.json".into(), Compression::Stored);

    zip.write_entry_whole(meta_builder, metadata.to_string().as_bytes())
        .await
        .unwrap();

    let world_zip_path = temp_dir.join(format!("{}_data.zip", world_entry_data.id));
    let mut world_zip = ZipFileWriter::with_tokio(File::create(&world_zip_path).await.unwrap());
    add_directory_to_zip(&mut world_zip, &world_path, &world_path)
        .await
        .unwrap();
    world_zip.close().await.unwrap();

    let mut world_zip_file = File::open(&world_zip_path).await.unwrap();
    let mut buffer = Vec::new();
    world_zip_file.read_to_end(&mut buffer).await.unwrap();

    let world_zip_builder = ZipEntryBuilder::new(
        format!("{}_data.zip", world_entry_data.id).into(),
        Compression::Stored,
    );

    zip.write_entry_whole(world_zip_builder, &buffer)
        .await
        .unwrap();

    zip.close().await.unwrap();

    tokio::fs::remove_file(&world_zip_path).await.unwrap();

    Ok(backup_path)
}

#[async_recursion]
async fn add_directory_to_zip<W: AsyncWrite + AsyncSeek + Unpin + Send>(
    zip_writer: &mut ZipFileWriter<W>,
    directory: &Path,
    prefix: &Path,
) -> Result<(), ZipError> {
    let mut entries = tokio::fs::read_dir(directory).await?;

    while let Ok(entry) = entries.next_entry().await {
        let entry = match entry {
            Some(entry) => entry,
            None => {
                break;
            }
        };
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();
        if path.is_file() {
            let builder = ZipEntryBuilder::new(name.to_str().unwrap().into(), Compression::Zstd)
                .unix_permissions(0o755);

            let mut f = File::open(&path).await?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).await?;

            zip_writer
                .write_entry_whole(builder, &buffer)
                .await
                .unwrap();
        } else if path.is_dir() {
            add_directory_to_zip(zip_writer, &path, prefix).await?;
        }
    }
    Ok(())
}

pub async fn create_backup_from_id(
    world_id: &str,
    category: Option<&str>,
    instance: Option<&str>,
    vaults: Option<Vec<String>>,
) -> Result<String, String> {
    info!("Creating backup for world id: {}", world_id);
    match get_world_path_by_id(world_id, category, instance).await {
        Ok(world_path) => {
            let world_backup_path = match create_world_backup(world_path.clone()).await {
                Ok(backup_path) => backup_path,
                Err(e) => {
                    error!(
                        "Failed to create backup for world folder {}: {:?}",
                        world_path.display(),
                        e
                    );
                    return Err(format!(
                        "Failed to create backup for world folder {}: {:?}",
                        world_path.display(),
                        e
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

                let backup_settings = get_backup_config().await?;

                for vault_id in vaults.unwrap() {
                    if let Some(vault) = backup_settings.vaults.get(&vault_id) {
                        vault_locations.insert(vault_id, vault);
                    }
                }

                info!("Copying backup to all {} vaults", vault_locations.len());

                for (vault_id, vault_path) in vault_locations {
                    let backup_location = vault_path.join(world_id);
                    if !backup_location.exists() {
                        match tokio::fs::create_dir_all(&backup_location).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Failed to create vault folder {}: {:?}", vault_id, e);
                                continue;
                            }
                        };
                    }
                    match tokio::fs::copy(&world_backup_path, backup_location.join(backup_name))
                        .await
                    {
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

                if let Err(e) = tokio::fs::remove_file(&world_backup_path).await {
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
                let default_vault = get_default_vault().await;
                match tokio::fs::rename(&world_backup_path, default_vault.join(backup_name)).await {
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
            Err(format!("Failed to grab world by id {}: {:?}", world_id, e))
        }
    }
}

pub async fn get_backup_meta_from_path(backup_path: PathBuf) -> Result<BackupMetadata, String> {
    let mut zip =
        match ZipFileReader::with_tokio(File::open(backup_path.clone()).await.unwrap()).await {
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

    let mut reader = match zip.reader_with_entry(0).await {
        Ok(reader) => reader,
        Err(e) => {
            return Err(format!(
                "Failed to open metadata file in backup {}: {:?}",
                backup_path.display(),
                e
            ));
        }
    };

    match reader.read_to_string_checked(&mut metadata).await {
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

pub async fn extract_world_backup(
    backup_path: PathBuf,
    extract_path: PathBuf,
) -> Result<(), String> {
    let mut zip =
        match ZipFileReader::with_tokio(File::open(backup_path.clone()).await.unwrap()).await {
            Ok(zip) => zip,
            Err(e) => {
                return Err(format!(
                    "Failed to open backup file {}: {:?}",
                    backup_path.display(),
                    e
                ));
            }
        };

    let mut reader = match zip.reader_with_entry(1).await {
        Ok(reader) => reader,
        Err(e) => {
            return Err(format!(
                "Failed to open metadata file in backup {}: {:?}",
                backup_path.display(),
                e
            ));
        }
    };

    let mut world_data = Vec::new();

    match reader.read_to_end_checked(&mut world_data).await {
        Ok(_) => {}
        Err(e) => {
            return Err(format!(
                "Failed to read world data file in backup {}: {:?}",
                backup_path.display(),
                e
            ));
        }
    };

    let cursor = std::io::Cursor::new(world_data);

    let mut world_data_zip = match ZipFileReader::with_tokio(cursor).await {
        Ok(zip) => zip,
        Err(e) => {
            return Err(format!(
                "Failed to open world data zip in backup {}: {:?}",
                backup_path.display(),
                e
            ));
        }
    };

    let mut index = 0;
    while let Ok(mut zip_entry) = world_data_zip.reader_with_entry(index).await {
        let entry = zip_entry.entry();

        let path = extract_path.join(entry.filename().as_str().unwrap());

        if entry.dir().unwrap() {
            tokio::fs::create_dir_all(&path)
                .await
                .map_err(|e| e.to_string())?;
        } else {
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    tokio::fs::create_dir_all(&parent)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }

            let mut file = tokio::fs::File::create(&path)
                .await
                .map_err(|e| e.to_string())?;

            let mut buffer = Vec::new();
            zip_entry
                .read_to_end_checked(&mut buffer)
                .await
                .map_err(|e| e.to_string())?;
            file.write_all(&buffer).await.map_err(|e| e.to_string())?;
        }

        index += 1;
    }

    Ok(())
}

pub async fn delete_backup(
    world_id: &str,
    vault: Option<&str>,
    snapshot_id: &str,
) -> Result<(), String> {
    let backup_settings = get_backup_config().await?;

    let vault_path = match vault {
        Some(vault_id) => {
            if let Some(vault) = backup_settings.vaults.get(vault_id) {
                vault.to_owned()
            } else {
                return Err(format!("Vault {} does not exist.", vault_id));
            }
        }
        None => get_default_vault().await,
    };

    let backup_path = vault_path
        .join(world_id)
        .join(format!("{}.chunkvault-snapshot", snapshot_id));

    info!("Removing backup {} for {}", snapshot_id, world_id);

    match tokio::fs::remove_file(backup_path.clone()).await {
        Ok(_) => {}
        Err(e) => {
            return Err(format!(
                "Failed to remove backup file {}: {:?}",
                backup_path.display(),
                e
            ));
        }
    };
    Ok(())
}

pub async fn delete_world_backups(world_id: &str, vault: Option<&str>) -> Result<(), String> {
    let backup_settings = get_backup_config().await?;

    let vault_path = match vault {
        Some(vault_id) => {
            if let Some(vault) = backup_settings.vaults.get(vault_id) {
                vault.to_owned()
            } else {
                return Err(format!("Vault {} does not exist.", vault_id));
            }
        }
        None => get_default_vault().await,
    };

    let backups_path = vault_path.join(world_id);

    info!("Removing all backups for {}", world_id);

    match tokio::fs::remove_dir_all(backups_path.clone()).await {
        Ok(_) => {}
        Err(e) => {
            return Err(format!(
                "Failed to remove backup folder {}: {:?}",
                backups_path.display(),
                e
            ));
        }
    };

    Ok(())
}
