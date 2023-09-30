use tauri::{
    plugin::{Builder, TauriPlugin},
    Wry,
};
use teller::types::{
    backup::{BackupMetadata, SnapshotInfo},
    world::WorldData,
};

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("backup_handler")
        .invoke_handler(tauri::generate_handler![
            create_backup_from_id,
            grab_local_backup_list,
            grab_world_metadata,
            grab_world_backups,
            grab_backup_metadata,
            delete_backup_from_id,
            delete_world_backups
        ])
        .build()
}

#[tauri::command]
async fn create_backup_from_id(
    world_id: String,
    category: Option<String>,
    vaults: Option<Vec<String>>,
) -> String {
    let result = tauri::async_runtime::spawn_blocking(move || {
        teller::handlers::backup::create_backup_from_id(&world_id, category.as_deref(), vaults)
    })
    .await;

    match result {
        Ok(Ok(path)) => path,
        Ok(Err(e)) => e.to_string(),
        Err(e) => e.to_string(),
    }
}

#[tauri::command]
fn grab_local_backup_list(vault: &str) -> Result<Vec<WorldData>, String> {
    teller::handlers::search::backups::fetch_backups_list(vault)
}

#[tauri::command]
fn grab_world_metadata(
    world_id: &str,
    selected_vault: Option<&str>,
) -> Result<BackupMetadata, String> {
    teller::handlers::search::backups::fetch_metadata_for_world(world_id, selected_vault)
}

#[tauri::command]
fn grab_world_backups(
    world_id: &str,
    selected_vault: Option<&str>,
) -> Result<Vec<SnapshotInfo>, String> {
    teller::handlers::search::backups::fetch_backups_for_world(world_id, selected_vault)
}

#[tauri::command]
fn grab_backup_metadata(
    world_id: &str,
    selected_vault: Option<&str>,
    backup_id: &str,
) -> Result<BackupMetadata, String> {
    teller::handlers::search::backups::fetch_metadata_for_backup(
        world_id,
        selected_vault,
        backup_id,
    )
}

#[tauri::command]
fn delete_backup_from_id(
    world_id: &str,
    selected_vault: Option<&str>,
    backup_id: &str,
) -> Result<(), String> {
    teller::handlers::backup::delete_backup(world_id, selected_vault, backup_id)
}

#[tauri::command]
fn delete_world_backups(world_id: &str, selected_vault: Option<&str>) -> Result<(), String> {
    teller::handlers::backup::delete_all_backups(world_id, selected_vault)
}
