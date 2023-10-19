use log::info;
use serde_json::{json, Value};
use tokio::fs;
use uuid::Uuid;

use std::{collections::HashMap, path::PathBuf};

use crate::{
    handlers::{
        search::worlds::is_minecraft_world,
        world::{parse_world_data, read_dat_file, GameType},
    },
    types::player::{Item, PlayerData},
};

pub async fn fetch_player_data_from_uuid(
    client: reqwest::Client,
    player_uuid_str: String,
) -> Result<Value, String> {
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
    let response = client.get(&url).send().await;

    match response {
        Ok(data) => match data.json::<Value>().await {
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

pub async fn fetch_players_meta_data(
    player_data_list: Vec<PlayerData>,
) -> Result<HashMap<String, Value>, String> {
    let mut player_data_map: HashMap<String, Value> = HashMap::new();
    let client = reqwest::Client::new();

    for player_data in player_data_list {
        let player_uuid = player_data.id;
        let client = client.clone();
        match fetch_player_data_from_uuid(client, player_uuid.clone()).await {
            Ok(player) => {
                player_data_map.insert(player_uuid, player);
            }
            Err(_e) => continue,
        }
    }

    Ok(player_data_map)
}

pub fn grab_player_from_uuid(player_uuid: String, path: &PathBuf) -> Result<PlayerData, String> {
    info!("Grabbing player from UUID: {}", player_uuid);

    let game_type = is_minecraft_world(path);

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
            let player_data = serde_json::to_value(local_player_data.unwrap())
                .map_err(|e| format!("Failed to parse player data: {:?}", e))?;

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

                let level_data = read_dat_file(level_dat_path, game_type)
                    .map_err(|e| format!("Failed to read level.dat: {:?}", e))?;

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
                    Ok((_, data)) => serde_json::to_value(data)
                        .map_err(|e| format!("Failed to parse player data: {:?}", e))?,
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

pub async fn get_player_data(path: &PathBuf, game_type: GameType) -> Result<Vec<Value>, String> {
    match game_type {
        GameType::Bedrock => {
            info!("Fetching Bedrock player data");

            let player_uuid = "~local_player".to_string();
            let player_avatar = get_steve_image();

            let db_path = path.join("db").to_str().unwrap().to_string();

            let mut db_reader = commandblock::db::DbReader::new(&db_path, 0);

            let remote_player_data = db_reader.parse_remote_players();

            let mut players: Vec<Value> = Vec::new();

            if remote_player_data.is_some() {
                for (uuid, _) in remote_player_data.unwrap().iter() {
                    info!("Fetching player data for: {:?}", uuid);

                    let player_meta = json!({
                        "id": uuid.strip_prefix("player_server_").unwrap_or(uuid),
                        "avatar": player_avatar,
                    });

                    players.push(player_meta);
                }
            }

            let local_player_data = json!({
                "id": player_uuid,
                "avatar": player_avatar,
            });

            players.push(local_player_data);

            Ok(players)
        }
        GameType::Java => {
            info!("Fetching Java player data");

            let player_data_path = path.join("playerdata");

            if !player_data_path.exists() {
                let level_dat_path = path.join("level.dat");

                let level_data = read_dat_file(level_dat_path, game_type)?;

                let level_data = parse_world_data(level_data, game_type)?;

                let player_data = match level_data.get("Player") {
                    Some(data) => data,
                    None => return Err("Could not find Player in level.dat".into()),
                };

                let player_uuid = match player_data.get("UUID") {
                    Some(player_uuid_values) => {
                        let d1 = player_uuid_values[0].as_i64().unwrap_or_default() as u32; // Your most significant 32-bit value
                        let d2 = player_uuid_values[1].as_i64().unwrap_or_default() as u32; // Your second most significant 32-bit value
                        let d3 = player_uuid_values[2].as_i64().unwrap_or_default() as u32; // Your second least significant 32-bit value
                        let d4 = player_uuid_values[3].as_i64().unwrap_or_default() as u32; // Your least significant 32-bit value

                        // Concatenate the four integers into a single 128-bit value
                        let uuid_int = ((d1 as u128) << 96)
                            | ((d2 as u128) << 64)
                            | ((d3 as u128) << 32)
                            | d4 as u128;

                        // Create a UUID from the 128-bit value
                        let player_uuid = Uuid::from_u128(uuid_int).to_string();

                        player_uuid
                    }
                    None => "~local_player".to_string(),
                };

                let player_avatar = match player_uuid.contains("~local_player") {
                    true => get_steve_image(),
                    false => format!(
                        "https://crafthead.net/avatar/{}?scale=32&overlay=false",
                        player_uuid
                    ),
                };

                let player_meta = json!({
                    "id": player_uuid,
                    "avatar": player_avatar,
                });

                return Ok(vec![player_meta]);
            }

            let mut player_data = match fs::read_dir(&player_data_path).await {
                Ok(data) => data,
                Err(e) => {
                    return Err(format!("Failed to read player data: {:?}", e).into());
                }
            };

            let mut all_players: Vec<Value> = Vec::new();

            while let Some(player) = player_data
                .next_entry()
                .await
                .map_err(|e| format!("Failed to read player data: {:?}", e))?
            {
                let player = player.path();

                if !player.is_file()
                    || player.extension().and_then(std::ffi::OsStr::to_str) != Some("dat")
                {
                    continue;
                }

                let player_uuid = player.file_stem().unwrap().to_str().unwrap().to_string();

                let player_avatar = format!(
                    "https://crafthead.net/avatar/{}?scale=32&overlay=false",
                    player_uuid
                );

                let player_meta = json!({
                    "id": player_uuid,
                    "avatar": player_avatar,
                    "meta": {}
                });

                all_players.push(player_meta);
            }

            Ok(all_players)
        }
        GameType::None => Err("Game type not specified".into()),
    }
}

// Literally the base64 encoded image of Steve's face
pub fn get_steve_image() -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        "data:image/png;base64,",
        "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",
        "eM+9YbQAAAABJRU5ErkJggg=="
    )
}
