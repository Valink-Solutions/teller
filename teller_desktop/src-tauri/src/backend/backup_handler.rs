use tauri::{
    plugin::{Builder, TauriPlugin},
    Wry,
};

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("backup_handler")
        .invoke_handler(tauri::generate_handler![create_backup_from_id])
        .build()
}

#[tauri::command]
async fn create_backup_from_id(world_id: String, category: Option<String>) -> String {
    let result = tauri::async_runtime::spawn_blocking(move || {
        teller::handlers::backup::create_backup_from_id(&world_id, category.as_deref())
    })
    .await;

    match result {
        Ok(Ok(path)) => path,
        Ok(Err(e)) => e.to_string(),
        Err(e) => e.to_string(),
    }
}
