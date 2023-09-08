use log::info;
// use log::info;
// use log::info;
use serde_json::{json, Value};
use uuid::Uuid;

use std::{collections::HashMap, path::PathBuf};

use crate::world::{parse_world_data, read_dat_file, GameType};

use super::{Item, PlayerData};

pub fn fetch_player_data_from_uuid(player_uuid_str: String) -> Result<Value, String> {
    if player_uuid_str == "~local_player".to_string() {
        let player_avatar =
            "https://crafthead.net/avatar/8667ba71b85a4004af54457a9734eed7?scale=32&overlay=false";

        let player_meta = json!({
            "username": "Local Player",
            "id": player_uuid_str,
            "avatar": player_avatar,
            "meta": {}
        });

        return Ok(player_meta);
    }

    let player_uuid = match Uuid::parse_str(&player_uuid_str) {
        Ok(uuid) => uuid,
        Err(_) => return Err("Error parsing UUID".to_string()),
    };

    let url = format!("https://playerdb.co/api/player/minecraft/{}", player_uuid);
    let response = reqwest::blocking::get(&url);

    match response {
        Ok(data) => match data.json::<Value>() {
            Ok(mut json) => {
                // info!("Fetched player data from playerdb.co: {:?}", json);
                if json
                    .get("success")
                    .unwrap_or(&json!(false))
                    .as_bool()
                    .unwrap_or(false)
                {
                    if let Some(player) =
                        json.get_mut("data").and_then(|data| data.get_mut("player"))
                    {
                        Ok(player.take())
                    } else {
                        Err("Player data not found in response".to_string())
                    }
                } else {
                    let player_avatar = "https://crafthead.net/avatar/8667ba71b85a4004af54457a9734eed7?scale=32&overlay=false";

                    let player_meta = json!({
                        "username": "Player",
                        "id": player_uuid,
                        "avatar": player_avatar,
                        "meta": {}
                    });

                    return Ok(player_meta);
                }
            }
            Err(_) => Err("Error parsing player data".to_string()),
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

pub fn grab_player_from_uuid(
    player_uuid: String,
    path: &PathBuf,
    game_type: GameType,
) -> Result<PlayerData, Box<dyn std::error::Error>> {
    info!("Grabbing player from UUID: {}", player_uuid);

    match game_type {
        GameType::Bedrock => {
            let db_path = path.join("db").to_str().unwrap().to_string();

            let mut db_reader = commandblock::db::DbReader::new(&db_path, 0);
            let (player_uuid, local_player_data) = if player_uuid == "~local_player" {
                (
                    "~local_player".to_string(),
                    db_reader.get("~local_player".as_bytes()),
                )
            } else {
                (
                    player_uuid.clone(),
                    db_reader.get(format!("player_server_{player_uuid}").as_bytes()),
                )
            };

            if local_player_data.is_none() {
                return Err("Failed to read player data".into());
            }
            let player_data = serde_json::to_value(local_player_data.unwrap())?;

            let player_data = PlayerData {
                id: player_uuid.to_owned(),
                health: None,
                food: None,
                game_mode: Some(player_data.get("PlayerGameMode").unwrap().as_i64().unwrap() as i32),
                level: player_data.get("PlayerLevel").unwrap().as_i64().unwrap() as i32,
                xp: player_data
                    .get("PlayerLevelProgress")
                    .unwrap()
                    .as_f64()
                    .unwrap() as f32,
                inventory: player_data
                    .get("Inventory")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|item| Item {
                        id: item.get("Name").unwrap().as_str().unwrap().to_string(),
                        slot: Some(item.get("Slot").unwrap().as_i64().unwrap() as i32),
                        damage: item
                            .get("tag")
                            .and_then(|tag| tag.get("Damage"))
                            .and_then(|damage| damage.as_i64())
                            .map(|damage| damage as i32),
                        count: item.get("Count").unwrap().as_i64().unwrap() as i32,
                        tag: Some(item.clone()),
                    })
                    .collect::<Vec<Item>>(),
            };

            Ok(player_data)
        }
        GameType::Java => {
            let player_data = if player_uuid == "~local_player" {
                let level_dat_path = path.join("level.dat");

                let level_data = read_dat_file(level_dat_path, game_type)?;

                let level_data = parse_world_data(level_data, game_type)?;

                match level_data.get("Player") {
                    Some(data) => data.to_owned(),
                    None => return Err("Could not find Player in level.dat".into()),
                }
            } else {
                let player = path.join("playerdata").join(format!("{}.dat", player_uuid));

                match commandblock::nbt::read_from_file(
                    player.clone(),
                    commandblock::nbt::Compression::Gzip,
                    commandblock::nbt::Endian::Big,
                    false,
                ) {
                    Ok((_, data)) => serde_json::to_value(data)?,
                    Err(e) => {
                        return Err(format!("Failed to read player data: {:?}", e).into());
                    }
                }
            };

            let player_data = PlayerData {
                id: player_uuid.to_string(),
                health: Some(player_data.get("Health").unwrap().as_f64().unwrap() as f32),
                food: Some(player_data.get("foodLevel").unwrap().as_i64().unwrap() as i32),
                game_mode: match player_data.get("playerGameType") {
                    Some(game_mode) => match game_mode.as_i64() {
                        Some(game_mode) => Some(game_mode as i32),
                        None => None,
                    },
                    None => None,
                },
                level: player_data.get("XpLevel").unwrap().as_i64().unwrap() as i32,
                xp: player_data.get("XpTotal").unwrap().as_f64().unwrap() as f32,
                inventory: player_data
                    .get("Inventory")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|item| Item {
                        id: match item.get("id").unwrap() {
                            Value::String(s) => s.clone(),
                            Value::Number(n) => n.to_string(),
                            _ => "".to_string(),
                        },
                        slot: Some(item.get("Slot").unwrap().as_i64().unwrap() as i32),
                        damage: item
                            .get("tag")
                            .and_then(|tag| tag.get("Damage"))
                            .and_then(|damage| damage.as_i64())
                            .map(|damage| damage as i32),
                        count: item.get("Count").unwrap().as_i64().unwrap() as i32,
                        tag: item.get("tag").cloned(),
                    })
                    .collect::<Vec<Item>>(),
            };

            Ok(player_data)
        }
        GameType::None => Err("Game type not found".into()),
    }
}
