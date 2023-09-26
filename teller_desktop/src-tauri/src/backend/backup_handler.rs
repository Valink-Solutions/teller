use std::path::PathBuf;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Wry,
};
use teller::types::world::WorldData;

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("backup_handler")
        .invoke_handler(tauri::generate_handler![
            create_backup_from_id,
            grab_local_backup_list
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
fn grab_local_backup_list(local_backups_path: PathBuf) -> Result<Vec<WorldData>, String> {
    teller::handlers::search::backups::fetch_backups_list(local_backups_path)
}
