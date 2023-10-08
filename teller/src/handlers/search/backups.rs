use std::path::PathBuf;

use tokio::fs;

use crate::{
    handlers::{backup::grab_backup_metadata, config::backup::get_backup_config},
    types::{
        backup::{BackupMetadata, SnapshotInfo},
        world::WorldData,
    },
};

async fn get_backups_from_path(directory_path: &str) -> Result<Vec<fs::DirEntry>, std::io::Error> {
    let mut entries = fs::read_dir(directory_path).await?;
    let mut files: Vec<fs::DirEntry> = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        files.push(entry);
    }

    Ok(files)
}

fn find_newest_backup(files: &[fs::DirEntry]) -> Option<PathBuf> {
    let mut newest_file: Option<PathBuf> = None;
    let mut newest_time = i64::MIN;

    for file in files {
        if let Some(file_name) = file.file_name().to_str() {
            let file_name = file_name.replace(".chunkvault-snapshot", "");

            if let Ok(time) = file_name.parse::<i64>() {
                if time > newest_time {
                    newest_time = time;
                    newest_file = Some(file.path());
                }
            }
        }
    }

    newest_file
}

pub async fn fetch_backups_list(vault: &str) -> Result<Vec<WorldData>, String> {
    let backup_settings = get_backup_config().await?;

    let local_backups_path = if let Some(vault_path) = backup_settings.vaults.get(vault) {
        vault_path
    } else {
        return Err(format!("Vault {} does not exist", vault));
    };

    let mut backup_entries = fs::read_dir(local_backups_path)
        .await
        .map_err(|e| format!("Failed to read backups directory: {}", e))?;

    let mut backups: Vec<WorldData> = Vec::new();

    while let Some(entry) = backup_entries
        .next_entry()
        .await
        .map_err(|e| format!("Failed to read backups directory: {}", e))?
    {
        let path = entry.path();

        if path.is_dir() {
            let all_backups = get_backups_from_path(path.to_str().unwrap())
                .await
                .map_err(|e| format!("Failed to read backups directory: {}", e))?;

            let newest_backup = find_newest_backup(&all_backups);

            if let Some(newest_backup) = newest_backup {
                let metadata = grab_backup_metadata(newest_backup).await;
                if metadata.is_ok() {
                    let world_data = metadata.unwrap();
                    backups.push(world_data.entry);
                } else {
                    continue;
                }
            }
        }
    }

    Ok(backups)
}

pub async fn fetch_backups_for_world(
    world_id: &str,
    selected_vault: Option<&str>,
) -> Result<Vec<SnapshotInfo>, String> {
    let backup_settings = get_backup_config().await?;

    let world_path = if let Some(selected_vault) = selected_vault {
        if let Some(vault_path) = backup_settings.vaults.get(selected_vault) {
            vault_path.join(world_id)
        } else {
            return Err(format!("Vault {} does not exist", selected_vault));
        }
    } else {
        return Err("No vault selected".to_string());
    };

    let files = get_backups_from_path(world_path.to_str().unwrap())
        .await
        .map_err(|e| format!("Failed to read backups directory: {}", e))?;

    let mut backups = Vec::new();

    for entry in files {
        if entry
            .file_name()
            .to_str()
            .unwrap()
            .contains(".chunkvault-snapshot")
        {
            let path = entry.path();

            let created = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(".chunkvault-snapshot", "");
            let created = created.parse::<i64>().unwrap();

            let metadata = fs::metadata(&path).await.unwrap();
            let size = metadata.len();

            let data = SnapshotInfo {
                created,
                size,
                path,
            };

            backups.push(data);
        }
    }

    Ok(backups)
}

pub async fn fetch_metadata_for_world(
    world_id: &str,
    selected_vault: Option<&str>,
) -> Result<BackupMetadata, String> {
    let backup_settings = get_backup_config().await?;

    let world_path = if let Some(selected_vault) = selected_vault {
        if let Some(vault_path) = backup_settings.vaults.get(selected_vault) {
            vault_path.join(world_id).to_owned()
        } else {
            return Err(format!("Vault {} does not exist", selected_vault));
        }
    } else {
        return Err("No vault selected".to_string());
    };

    let files = get_backups_from_path(world_path.to_str().unwrap())
        .await
        .map_err(|e| format!("Failed to read backups directory: {}", e))?;

    match find_newest_backup(&files) {
        Some(newest_backup) => return grab_backup_metadata(newest_backup).await,
        None => Err("No backups found".to_string()),
    }
}

pub async fn fetch_metadata_for_backup(
    world_id: &str,
    selected_vault: Option<&str>,
    backup_id: &str,
) -> Result<BackupMetadata, String> {
    let backup_settings = get_backup_config().await?;

    let world_path = if let Some(selected_vault) = selected_vault {
        if let Some(vault_path) = backup_settings.vaults.get(selected_vault) {
            vault_path.join(world_id).to_owned()
        } else {
            return Err(format!("Vault {} does not exist", selected_vault));
        }
    } else {
        return Err("No vault selected".to_string());
    };

    let backup_path = world_path.join(format!("{}.chunkvault-snapshot", backup_id));

    if backup_path.exists() {
        return grab_backup_metadata(backup_path).await;
    } else {
        return Err("Backup does not exist".to_string());
    }
}
