use std::{path::PathBuf, str::FromStr};

use log::info;

use crate::handlers::{
    backup::grab_backup_metadata, search::directories::get_directory_by_name, world::new_vault_id,
};

use super::{
    backup::extract_world_backup, config::backup::get_backup_config,
    search::worlds::world_path_from_id,
};

pub async fn snapshot_to_world(
    snapshot_id: &str,
    selected_vault: Option<&str>,
    world_id: &str,
    replace: bool,
    instances: Vec<String>,
) -> Result<(), String> {
    let backup_settings = get_backup_config()?;

    let world_path = if let Some(selected_vault) = selected_vault {
        if let Some(vault_path) = backup_settings.vaults.get(selected_vault) {
            vault_path.join(world_id).to_owned()
        } else {
            return Err(format!("Vault {} does not exist", selected_vault));
        }
    } else {
        return Err("No vault selected".to_string());
    };

    let backup_path = world_path.join(format!("{}.chunkvault-snapshot", snapshot_id));

    info!("Restoring backup from {:?}", backup_path);

    if backup_path.exists() {
        for instance in instances {
            let mut world_path = match world_path_from_id(world_id, None, Some(&instance)) {
                Ok(path) => path.to_owned(),
                Err(_) => {
                    let instance_path = get_directory_by_name(&instance, None).unwrap();

                    let metadata = grab_backup_metadata(backup_path.clone()).await?;

                    instance_path.join(&metadata.entry.name)
                }
            };

            if replace {
                if world_path.exists() {
                    tokio::fs::remove_dir_all(&world_path)
                        .await
                        .map_err(|e| e.to_string())?;
                }

                tokio::fs::create_dir_all(&world_path)
                    .await
                    .map_err(|e| e.to_string())?;

                extract_world_backup(backup_path.clone(), world_path).await?;
            } else {
                let mut copy_counter = 1;
                let original_world_path = world_path.clone();

                while world_path.exists() {
                    let new_world_path = format!(
                        "{}-copy({})",
                        original_world_path.to_str().unwrap(),
                        copy_counter
                    );
                    copy_counter += 1;
                    world_path = PathBuf::from_str(&new_world_path).unwrap();
                }

                tokio::fs::create_dir_all(&world_path)
                    .await
                    .map_err(|e| e.to_string())?;
                extract_world_backup(backup_path.clone(), world_path.clone().to_owned()).await?;

                new_vault_id(&world_path)?;
            }
        }
    } else {
        return Err("Backup does not exist".to_string());
    }

    Ok(())
}
