use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde_json::Value;
use teller::{
    handlers::player::grab_player_from_uuid,
    types::{player::PlayerData, world::WorldLevelData},
};

use tauri::{
    plugin::{Builder, TauriPlugin},
    Wry,
};

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("world_handler")
        .invoke_handler(tauri::generate_handler![
            get_world_by_id,
            get_world_path_by_id,
            get_player_meta_from_uuids,
            get_player_meta_from_uuid,
            get_player_from_uuid,
            delete_world_by_id,
        ])
        .build()
}

#[tauri::command]
pub async fn get_world_by_id(
    world_id: &str,
    category: Option<&str>,
    instance: Option<&str>,
) -> Result<WorldLevelData, String> {
    teller::handlers::search::worlds::get_world_by_id(world_id, category, instance).await
}

#[tauri::command]
async fn get_world_path_by_id(
    world_id: &str,
    category: Option<&str>,
    instance: Option<&str>,
) -> Result<PathBuf, String> {
    teller::handlers::search::worlds::get_world_path_by_id(world_id, category, instance).await
}

#[tauri::command]
async fn get_player_meta_from_uuids(
    player_data_list: Vec<PlayerData>,
) -> Result<HashMap<String, Value>, String> {
    teller::handlers::player::get_players_meta_from_uuids(player_data_list).await
}

#[tauri::command]
async fn get_player_meta_from_uuid(player_uuid: String) -> Result<Value, String> {
    teller::handlers::player::get_player_meta_from_uuid(reqwest::Client::new(), player_uuid).await
}

#[tauri::command]
fn get_player_from_uuid(player_uuid: String, path: &Path) -> Result<PlayerData, String> {
    match grab_player_from_uuid(player_uuid, &path.to_path_buf()) {
        Ok(player) => Ok(player),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn delete_world_by_id(
    world_id: &str,
    category: Option<&str>,
    instance: Option<&str>,
) -> Result<(), String> {
    teller::handlers::world::delete_world_by_id(world_id, category, instance).await
}
