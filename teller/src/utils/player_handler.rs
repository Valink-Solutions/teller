use log::info;
use serde_json::Value;
use uuid::Uuid;

use std::collections::HashMap;

use super::PlayerData;

pub fn fetch_player_data_from_uuid(player_uuid_str: String) -> Result<Value, String> {
    let player_uuid = match Uuid::parse_str(&player_uuid_str) {
        Ok(uuid) => uuid,
        Err(_) => return Err("Error parsing UUID".to_string()),
    };

    let url = format!("https://playerdb.co/api/player/minecraft/{}", player_uuid);
    let response = reqwest::blocking::get(&url);

    match response {
        Ok(data) => match data.json::<Value>() {
            Ok(mut json) => {
                info!("Fetched player data from playerdb.co: {:?}", json);
                if let Some(player) = json.get_mut("data").and_then(|data| data.get_mut("player")) {
                    Ok(player.take())
                } else {
                    Err("Player data not found in response".to_string())
                }
            }
            Err(_) => Err("Error parsing JSON".to_string()),
        },
        Err(_) => Err("Error fetching player data".to_string()),
    }
}

pub fn fetch_players_meta_data(
    player_data_list: Vec<PlayerData>,
) -> Result<HashMap<String, Value>, String> {
    let mut player_data_map: HashMap<String, Value> = HashMap::new();

    for player_data in player_data_list {
        let player_uuid = player_data.id;
        match fetch_player_data_from_uuid(player_uuid.clone()) {
            Ok(player) => {
                player_data_map.insert(player_uuid, player);
            }
            Err(_e) => continue,
        }
    }

    Ok(player_data_map)
}
