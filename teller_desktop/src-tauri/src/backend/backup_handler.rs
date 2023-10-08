use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Wry,
};
use teller::types::{
    backup::{BackupMetadata, SnapshotInfo},
    world::WorldData,
};

use crate::types::events::ToastEvent;

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("backup_handler")
        .invoke_handler(tauri::generate_handler![
            create_backup_from_id,
            grab_local_backup_list,
            grab_world_metadata,
            grab_world_backups,
            grab_backup_metadata,
            delete_backup_from_id,
            delete_world_backups,
            restore_snapshot_to_world
        ])
        .build()
}

#[tauri::command]
async fn create_backup_from_id(
    app: tauri::AppHandle,
    world_id: String,
    category: Option<String>,
    instance: Option<String>,
    vaults: Option<Vec<String>>,
) -> String {
    let world_id_clone = world_id.clone();

    let result = teller::handlers::backup::create_backup_from_id(
        &world_id,
        category.as_deref(),
        instance.as_deref(),
        vaults,
    )
    .await;

    match result {
        Ok(path) => {
            let _ = app.emit_all("world_backup_list_updated", &world_id_clone);

            let _ = app.emit_all("backup_list_updated", ());

            let _ = app.emit_all(
                "toast",
                ToastEvent {
                    message: "Backup created successfully".to_string(),
                },
            );
            path
        }
        Err(e) => {
            let _ = app.emit_all(
                "error",
                ToastEvent {
                    message: format!("Error creating backup: {}", e),
                },
            );
            e
        }
    }
}

#[tauri::command]
async fn grab_local_backup_list(vault: &str) -> Result<Vec<WorldData>, String> {
    teller::handlers::search::backups::fetch_backups_list(vault).await
}

#[tauri::command]
async fn grab_world_metadata(
    world_id: &str,
    selected_vault: Option<&str>,
) -> Result<BackupMetadata, String> {
    teller::handlers::search::backups::fetch_metadata_for_world(world_id, selected_vault).await
}

#[tauri::command]
async fn grab_world_backups(
    world_id: &str,
    selected_vault: Option<&str>,
) -> Result<Vec<SnapshotInfo>, String> {
    teller::handlers::search::backups::fetch_backups_for_world(world_id, selected_vault).await
}

#[tauri::command]
async fn grab_backup_metadata(
    world_id: &str,
    selected_vault: Option<&str>,
    backup_id: &str,
) -> Result<BackupMetadata, String> {
    teller::handlers::search::backups::fetch_metadata_for_backup(
        world_id,
        selected_vault,
        backup_id,
    )
    .await
}

#[tauri::command]
async fn delete_backup_from_id(
    world_id: &str,
    selected_vault: Option<&str>,
    backup_id: &str,
) -> Result<(), String> {
    teller::handlers::backup::delete_backup(world_id, selected_vault, backup_id).await
}

#[tauri::command]
async fn delete_world_backups(world_id: &str, selected_vault: Option<&str>) -> Result<(), String> {
    teller::handlers::backup::delete_all_backups(world_id, selected_vault).await
}

#[tauri::command]
async fn restore_snapshot_to_world(
    app: tauri::AppHandle,
    snapshot_id: &str,
    selected_vault: Option<&str>,
    world_id: &str,
    replace: bool,
    instances: Vec<String>,
) -> Result<(), String> {
    match teller::handlers::snapshot::snapshot_to_world(
        snapshot_id,
        selected_vault,
        world_id,
        replace,
        instances,
    )
    .await
    {
        Ok(_) => {
            let _ = app.emit_all(
                "toast",
                ToastEvent {
                    message: "Succesfully restored backup to isntance.".to_string(),
                },
            );

            Ok(())
        }
        Err(e) => {
            let _ = app.emit_all(
                "error",
                ToastEvent {
                    message: format!("Error restoring backup: {}", e),
                },
            );
            Err(e)
        }
    }
}
