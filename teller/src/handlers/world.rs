use std::path::PathBuf;

use chrono::NaiveDateTime;
use commandblock::nbt::{read_from_file, Compression, Endian, NbtValue};
use log::{error, info};
use serde_json::Value;
use tokio::{fs, io::AsyncWriteExt};

use crate::{
    handlers::{
        player::get_player_data,
        search::worlds::{is_minecraft_world, world_path_from_id},
    },
    types::world::{GameRules, WorldData, WorldLevelData},
    utils::{calculate_dir_size, encode_image_to_base64},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameType {
    Java,
    Bedrock,
    None,
}

pub async fn create_vault_file(vault_data: Value, world_path: &PathBuf) -> Result<(), String> {
    info!("Creating vault file for: {:?}", world_path);

    let vault_file_path = world_path.join(".chunkvault");

    if vault_file_path.exists() {
        return Err("Vault file already exists".to_string());
    }

    let mut vault_file = match fs::File::create(&vault_file_path).await {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to create vault file: {e:?} {vault_file_path:?}");
            return Err(format!("Failed to create vault file: {:?}", e));
        }
    };

    match vault_file
        .write_all(vault_data.to_string().as_bytes())
        .await
    {
        Ok(_) => {}
        Err(e) => {
            return Err(format!("Failed to write vault file: {:?}", e));
        }
    }

    Ok(())
}

pub async fn get_vault_file(world_path: &PathBuf) -> Result<Value, String> {
    let vault_file_path = world_path.join(".chunkvault");

    if !vault_file_path.exists() {
        return Err("Vault file does not exist".to_string());
    }

    let vault_file = match fs::File::open(&vault_file_path).await {
        Ok(file) => file,
        Err(e) => {
            return Err(format!("Failed to open vault file: {e:?}"));
        }
    };

    let vault_data: Value = match serde_json::from_reader(vault_file.into_std().await) {
        Ok(data) => data,
        Err(e) => {
            return Err(format!("Failed to read vault file: {:?}", e));
        }
    };

    Ok(vault_data)
}

pub async fn update_vault_file(vault_data: Value, world_path: &PathBuf) -> Result<(), String> {
    let vault_file_path = world_path.join(".chunkvault");

    if !vault_file_path.exists() {
        return Err("Vault file does not exist".to_string());
    }

    let mut vault_file = match fs::File::create(&vault_file_path).await {
        Ok(file) => file,
        Err(e) => {
            return Err(format!("Failed to create vault file: {:?}", e));
        }
    };

    match vault_file
        .write_all(vault_data.to_string().as_bytes())
        .await
    {
        Ok(_) => {}
        Err(e) => {
            return Err(format!("Failed to write vault file: {:?}", e));
        }
    }

    Ok(())
}

pub async fn get_vault_id(path: &PathBuf) -> Result<String, String> {
    let vault_data = match get_vault_file(path).await {
        Ok(data) => data,
        Err(_) => {
            let new_vault_id = uuid::Uuid::new_v4().to_string();

            let new_vault_data = serde_json::json!({
                "id": new_vault_id
            });

            match create_vault_file(new_vault_data, path).await {
                Ok(_) => {}
                Err(e) => return Err(e),
            };

            match get_vault_file(path).await {
                Ok(data) => data,
                Err(e) => return Err(e),
            }
        }
    };

    let vault_id = match vault_data["id"].as_str() {
        Some(id) => id,
        None => {
            let new_vault_id = uuid::Uuid::new_v4().to_string();
            let mut vault_data = vault_data;
            vault_data["id"] = serde_json::Value::String(new_vault_id.clone());
            match update_vault_file(vault_data, &path).await {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
            return Ok(new_vault_id);
        }
    };

    Ok(vault_id.to_string())
}

pub async fn new_vault_id(world_path: &PathBuf) -> Result<(), String> {
    let mut vault_info = get_vault_file(world_path).await?;
    if let Some(id_pointer) = vault_info.pointer_mut("/id") {
        *id_pointer = serde_json::Value::String(uuid::Uuid::new_v4().to_string());
    }
    update_vault_file(vault_info, world_path).await?;

    Ok(())
}

// Data parsing & processing

pub fn read_dat_file(file_path: PathBuf, game_type: GameType) -> Result<NbtValue, String> {
    info!("Parsing {:?} dat file: {:?}", game_type, file_path);

    match game_type {
        GameType::Java => {
            let (_, dat_blob) =
                match read_from_file(file_path, Compression::Gzip, Endian::Big, false) {
                    Ok(data) => data,
                    Err(e) => return Err(format!("Failed to read level.dat: {e:?}").into()),
                };
            Ok(dat_blob)
        }
        GameType::Bedrock => {
            let (_, dat_blob) =
                match read_from_file(file_path, Compression::Uncompressed, Endian::Little, true) {
                    Ok(data) => data,
                    Err(e) => return Err(format!("Failed to read level.dat: {e:?}").into()),
                };
            Ok(dat_blob)
        }
        GameType::None => Err("Game type not specified".into()),
    }
}

pub fn get_world_data(world_path: &PathBuf) -> Result<Value, String> {
    info!("Getting world data for {:?}", world_path);

    let game_type = is_minecraft_world(&world_path);

    let level_dat_path = world_path.join("level.dat");

    if !level_dat_path.exists() {
        return Err("level.dat does not exist".to_string());
    }

    let level_data = match read_dat_file(level_dat_path, game_type) {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to parse level.dat: {:?}", e)),
    };

    let level_data = match parse_world_data(level_data, game_type) {
        Ok(data) => data,
        Err(e) => return Err(e),
    };

    Ok(level_data)
}

pub fn parse_world_data(world_data: NbtValue, game_type: GameType) -> Result<Value, String> {
    let level_value: serde_json::Value = match serde_json::to_value(world_data) {
        Ok(value) => value,
        Err(e) => return Err(format!("Failed to parse level.dat JSON: {:?}", e)),
    };

    info!("Parsing world data for {:?}", game_type);

    match game_type {
        GameType::Bedrock => Ok(level_value),
        GameType::Java => {
            let level_data = match level_value.get("Data") {
                Some(data) => data,
                None => return Err("Could not find Data in level.dat".into()),
            };
            Ok(level_data.to_owned())
        }
        GameType::None => Err("Game type not specified".into()),
    }
}

pub async fn process_world_data(
    path: &PathBuf,
    game_type: GameType,
) -> Result<WorldLevelData, String> {
    info!("Processing world data for: {:?}", path);

    let level_dat = read_dat_file(path.join("level.dat"), game_type).map_err(|e| e.to_string())?;

    let level_value: serde_json::Value =
        serde_json::to_value(level_dat).map_err(|e| e.to_string())?;

    match game_type {
        GameType::Bedrock => {
            let world_level_data = WorldLevelData {
                game_engine: "Bedrock".to_string(),
                name: level_value["LevelName"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                folder: Some(path.to_str().unwrap().to_string()),
                icon: match encode_image_to_base64(path.clone().join("world_icon.jpeg")).await {
                    Ok(data) => Some(data),
                    Err(_) => None,
                },
                difficulty: {
                    let difficulty = level_value["Difficulty"].as_i64().unwrap_or(2) as i32;
                    match difficulty {
                        0 => "Peaceful".to_string(),
                        1 => "Easy".to_string(),
                        2 => "Normal".to_string(),
                        3 => "Hard".to_string(),
                        _ => "Unknown".to_string(),
                    }
                },
                game_type: {
                    let game_type = level_value["GameType"].as_i64().unwrap_or_default() as i32;
                    match game_type {
                        0 => "Survival".to_string(),
                        1 => "Creative".to_string(),
                        2 => "Adventure".to_string(),
                        3 => "Spectator".to_string(),
                        _ => "Unknown".to_string(),
                    }
                },
                last_played: {
                    let last_played = level_value["LastPlayed"].as_i64().unwrap_or_default();
                    let naive_datetime = chrono::NaiveDateTime::from_timestamp_opt(last_played, 0);
                    naive_datetime
                },
                players: get_player_data(path, game_type)
                    .await
                    .map_err(|e| e.to_string())?,
                size_on_disk: {
                    info!("Calculating directory size for: {:?}", path);
                    calculate_dir_size(path.clone().to_owned())
                        .await
                        .map_err(|e| e.to_string())? as i64
                },
                game_rules: match parse_game_rules(&level_value, game_type) {
                    Ok(rules) => Some(rules),
                    Err(_) => None,
                },
            };

            return Ok(world_level_data);
        }
        GameType::Java => {
            let level_data = match level_value.get("Data") {
                Some(data) => data,
                None => return Err("Could not find Data in level.dat".into()),
            };

            let world_level_data = WorldLevelData {
                game_engine: "Java".to_string(),
                name: level_data["LevelName"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                folder: Some(path.to_str().unwrap().to_string()),
                icon: match encode_image_to_base64(path.clone().join("icon.png")).await {
                    Ok(data) => Some(data),
                    Err(_) => None,
                },
                difficulty: {
                    let difficulty = level_data["Difficulty"].as_i64().unwrap_or(2) as i32;
                    let hardcore = level_data["hardcore"].as_bool().unwrap_or_default();

                    if hardcore {
                        "Hardcore".to_string()
                    } else {
                        match difficulty {
                            0 => "Peaceful".to_string(),
                            1 => "Easy".to_string(),
                            2 => "Normal".to_string(),
                            3 => "Hard".to_string(),
                            _ => "Unknown".to_string(),
                        }
                    }
                },
                game_type: {
                    let game_type = level_data["GameType"].as_i64().unwrap_or_default() as i32;
                    match game_type {
                        0 => "Survival".to_string(),
                        1 => "Creative".to_string(),
                        2 => "Adventure".to_string(),
                        3 => "Spectator".to_string(),
                        _ => "Unknown".to_string(),
                    }
                },
                last_played: {
                    let last_played = level_data["LastPlayed"].as_i64().unwrap_or_default();
                    let naive_datetime = chrono::NaiveDateTime::from_timestamp_millis(last_played);
                    naive_datetime
                },
                players: get_player_data(path, game_type)
                    .await
                    .map_err(|e| e.to_string())?,
                size_on_disk: {
                    info!("Calculating directory size for: {:?}", path);
                    calculate_dir_size(path.clone().to_owned())
                        .await
                        .map_err(|e| e.to_string())? as i64
                },
                game_rules: match parse_game_rules(&level_data, game_type) {
                    Ok(rules) => Some(rules),
                    Err(_) => None,
                },
            };

            return Ok(world_level_data);
        }
        GameType::None => return Err("Game type not specified".into()),
    }
}

pub fn parse_game_rules(
    game_data: &serde_json::Value,
    game_type: GameType,
) -> Result<GameRules, String> {
    match game_type {
        GameType::Java => {
            let game_rules = match game_data.get("GameRules") {
                Some(rules) => GameRules {
                    do_fire_tick: rules
                        .get("doFireTick")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    mob_loot: rules
                        .get("doMobLoot")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    keep_inventory: rules
                        .get("keepInventory")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    do_mob_spawning: rules
                        .get("doMobSpawning")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    do_tile_drops: rules
                        .get("doTileDrops")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    command_block_output: rules
                        .get("commandBlockOutput")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    natural_regeneration: rules
                        .get("naturalRegeneration")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    do_daylight_cycle: rules
                        .get("doDaylightCycle")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    do_weather_cycle: rules
                        .get("doWeatherCycle")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    do_immediate_respawn: rules
                        .get("doImmediateRespawn")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    drowning_damage: rules
                        .get("drowningDamage")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    fall_damage: rules
                        .get("fallDamage")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    fire_damage: rules
                        .get("fireDamage")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    do_insomnia: rules
                        .get("doInsomnia")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    invulnerable: rules
                        .get("invulnerable")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    max_command_chain_length: rules
                        .get("maxCommandChainLength")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .parse::<i64>()
                        .unwrap_or_default(),
                    random_tick_speed: rules
                        .get("randomTickSpeed")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .parse::<i64>()
                        .unwrap_or_default(),
                    reduced_debug_info: rules
                        .get("reducedDebugInfo")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    send_command_feedback: rules
                        .get("sendCommandFeedback")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    show_death_messages: rules
                        .get("showDeathMessages")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                    spawn_radius: rules
                        .get("spawnRadius")
                        .and_then(|v| v.as_str())
                        .unwrap_or("0")
                        .parse::<i64>()
                        .unwrap_or_default(),
                    spectators_generate_chunks: rules
                        .get("spectatorsGenerateChunks")
                        .and_then(|v| v.as_str())
                        .unwrap_or("false")
                        == "true",
                },
                None => return Err("Could not find GameRules in level.dat".into()),
            };
            Ok(game_rules)
        }
        GameType::Bedrock => {
            let game_rules = GameRules {
                do_fire_tick: game_data
                    .get("dofiretick")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                mob_loot: game_data
                    .get("mobloot")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                keep_inventory: game_data
                    .get("keepinventory")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                do_mob_spawning: game_data
                    .get("domobspawning")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                do_tile_drops: game_data
                    .get("dotiledrops")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                command_block_output: game_data
                    .get("commandblockoutput")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                natural_regeneration: game_data
                    .get("naturalregeneration")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                do_daylight_cycle: game_data
                    .get("dodaylightcycle")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                do_weather_cycle: game_data
                    .get("doweathercycle")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                do_immediate_respawn: game_data
                    .get("doimmediaterespawn")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                drowning_damage: game_data
                    .get("drowningdamage")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                fall_damage: game_data
                    .get("falldamage")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                fire_damage: game_data
                    .get("firedamage")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                do_insomnia: game_data
                    .get("doinsomnia")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                invulnerable: game_data
                    .get("invulnerable")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                max_command_chain_length: game_data
                    .get("maxcommandchainlength")
                    .and_then(|v| v.as_str())
                    .unwrap_or("0")
                    .parse::<i64>()
                    .unwrap_or_default(),
                random_tick_speed: game_data
                    .get("randomtickspeed")
                    .and_then(|v| v.as_str())
                    .unwrap_or("0")
                    .parse::<i64>()
                    .unwrap_or_default(),
                reduced_debug_info: game_data
                    .get("reduceddebuginfo")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                send_command_feedback: game_data
                    .get("sendcommandfeedback")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                show_death_messages: game_data
                    .get("showdeathmessages")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
                spawn_radius: game_data
                    .get("spawnradius")
                    .and_then(|v| v.as_str())
                    .unwrap_or("0")
                    .parse::<i64>()
                    .unwrap_or_default(),
                spectators_generate_chunks: game_data
                    .get("spectatorsgeneratechunks")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
                    == 1,
            };
            Ok(game_rules)
        }
        GameType::None => Err("Game type not specified".into()),
    }
}

pub fn get_level_info(
    level_dat_blob: NbtValue,
    game_type: GameType,
) -> Result<(String, Option<NaiveDateTime>), Box<dyn std::error::Error>> {
    let level_value: serde_json::Value = serde_json::to_value(level_dat_blob)?;

    match game_type {
        GameType::Java => {
            let level_data = match level_value.get("Data") {
                Some(data) => data,
                None => return Err("Could not find Data in level.dat".into()),
            };

            let level_name = match level_data.get("LevelName") {
                Some(name) => name.to_string(),
                None => return Err("Could not find LevelName in level.dat".into()),
            };

            let last_played = match level_data.get("LastPlayed") {
                Some(time) => time.as_i64().unwrap_or_default(),
                None => return Err("Could not find LastPlayed in level.dat".into()),
            };

            let parsed_level_name = level_name[1..level_name.len() - 1].to_string();

            let parsed_last_played = chrono::NaiveDateTime::from_timestamp_millis(last_played);

            Ok((parsed_level_name, parsed_last_played))
        }
        GameType::Bedrock => {
            let level_name = match level_value.get("LevelName") {
                Some(name) => name.to_string(),
                None => return Err("Could not find levelName in level.dat".into()),
            };

            let last_played = match level_value.get("LastPlayed") {
                Some(time) => time.as_i64().unwrap_or_default(),
                None => return Err("Could not find LastPlayed in level.dat".into()),
            };

            let parsed_level_name = level_name[1..level_name.len() - 1].to_string();

            let parsed_last_played = chrono::NaiveDateTime::from_timestamp_opt(last_played, 0);

            Ok((parsed_level_name, parsed_last_played))
        }
        GameType::None => Err("Could not find game type".into()),
    }
}

pub async fn parse_world_entry_data(path: PathBuf) -> Result<WorldData, String> {
    let game_type = is_minecraft_world(&path);

    let level_dat_path = path.join("level.dat");
    let level_dat_blob = match read_dat_file(level_dat_path, game_type) {
        Ok(blob) => blob,
        Err(e) => {
            error!("Could not parse level.dat at {:?}: {:?}", path, e);
            return Err(format!("Could not parse level.dat at {:?}: {:?}", path, e));
        }
    };

    let (level_name, last_played) = match get_level_info(level_dat_blob, game_type) {
        Ok((name, time)) => (name, time),
        Err(e) => {
            error!("Could not get level name at {:?}: {:?}", path, e);
            return Err(format!("Could not get level name at {:?}: {:?}", path, e));
        }
    };

    let world_size = match calculate_dir_size(path.clone()).await {
        Ok(size) => size,
        Err(_) => 0,
    };

    let vault_id = match get_vault_id(&path).await {
        Ok(id) => id,
        Err(e) => {
            error!("Could not get vault id at {:?}: {:?}", path, e);
            return Err(format!("Could not get vault id at {:?}: {:?}", path, e));
        }
    };

    let world_data = WorldData {
        id: vault_id,
        name: level_name,
        image: match game_type {
            GameType::Java => encode_image_to_base64(path.join("icon.png"))
                .await
                .unwrap_or("".to_string()),
            GameType::Bedrock => encode_image_to_base64(path.join("world_icon.jpeg"))
                .await
                .unwrap_or("".to_string()),
            GameType::None => "".to_string(),
        },
        path: path.to_string_lossy().into_owned(),
        size: world_size,
        last_played: last_played,
    };

    Ok(world_data)
}

pub async fn delete_world(
    world_id: &str,
    category: Option<&str>,
    instance: Option<&str>,
) -> Result<(), String> {
    let world_path = world_path_from_id(world_id, category, instance).await?;

    if !world_path.exists() {
        error!("World does not exist: {:?}", world_path);
        return Err("World does not exist".into());
    }

    info!("Deleting world at {:?}", world_path);

    if let Err(e) = fs::remove_dir_all(world_path).await {
        return Err(format!("Failed to delete world: {:?}", e));
    }

    Ok(())
}
