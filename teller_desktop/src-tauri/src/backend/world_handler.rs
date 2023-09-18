use std::{collections::HashMap, path::Path};

use serde_json::Value;
use teller::{
    handlers::player::{fetch_player_data_from_uuid, fetch_players_meta_data},
    types::player::PlayerData,
};

use tauri::{
    plugin::{Builder, TauriPlugin},
    Wry,
};

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("world_handler")
        .invoke_handler(tauri::generate_handler![
            get_world_by_id,
            grab_player_meta_from_uuids,
            grab_player_meta_from_uuid,
            grab_player_from_uuid
        ])
        .build()
}

#[tauri::command]
pub fn get_world_by_id(
    world_id: &str,
    return_path: Option<bool>,
    category: Option<&str>,
) -> Result<Value, String> {
    Ok(teller::handlers::search::worlds::get_world_by_id(
        world_id,
        return_path,
        category,
    )?)
}

#[tauri::command]
fn grab_player_meta_from_uuids(
    player_data_list: Vec<PlayerData>,
) -> Result<HashMap<String, Value>, String> {
    Ok(fetch_players_meta_data(player_data_list)?)
}

#[tauri::command]
fn grab_player_meta_from_uuid(player_uuid: String) -> Result<Value, String> {
    Ok(fetch_player_data_from_uuid(player_uuid)?)
}

#[tauri::command]
fn grab_player_from_uuid(player_uuid: String, path: &Path) -> Result<PlayerData, String> {
    match teller::handlers::player::grab_player_from_uuid(player_uuid, &path.to_path_buf()) {
        Ok(player) => Ok(player),
        Err(e) => Err(e.to_string()),
    }
}
