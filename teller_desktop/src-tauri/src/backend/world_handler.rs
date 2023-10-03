use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde_json::Value;
use teller::{
    handlers::{
        player::{fetch_player_data_from_uuid, fetch_players_meta_data, grab_player_from_uuid},
        search::worlds::{grab_world_by_id, world_path_from_id},
    },
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
) -> Result<WorldLevelData, String> {
    grab_world_by_id(world_id, category).await
}

#[tauri::command]
fn get_world_path_by_id(world_id: &str, category: Option<&str>) -> Result<PathBuf, String> {
    world_path_from_id(world_id, category)
}

#[tauri::command]
async fn get_player_meta_from_uuids(
    player_data_list: Vec<PlayerData>,
) -> Result<HashMap<String, Value>, String> {
    fetch_players_meta_data(player_data_list).await
}

#[tauri::command]
async fn get_player_meta_from_uuid(player_uuid: String) -> Result<Value, String> {
    fetch_player_data_from_uuid(reqwest::Client::new(), player_uuid).await
}

#[tauri::command]
fn get_player_from_uuid(player_uuid: String, path: &Path) -> Result<PlayerData, String> {
    match grab_player_from_uuid(player_uuid, &path.to_path_buf()) {
        Ok(player) => Ok(player),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn delete_world_by_id(world_id: &str, category: Option<&str>) -> Result<(), String> {
    teller::handlers::world::delete_world(world_id, category)
}
