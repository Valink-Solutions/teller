use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use chrono::NaiveDateTime;
use commandblock::nbt::{read_from_file, Compression, Endian, NbtValue};
use log::{error, info};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    handlers::player::fetch_player_data_from_uuid,
    types::world::{GameRules, WorldData, WorldLevelData},
    utils::{calculate_dir_size, encode_image_to_base64},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameType {
    Java,
    Bedrock,
    None,
}

pub fn create_vault_file(vault_data: Value, world_path: &PathBuf) -> Result<(), String> {
    info!("Creating vault file for: {:?}", world_path);

    let vault_file_path = world_path.join(".chunkvault");

    if vault_file_path.exists() {
        return Err("Vault file already exists".to_string());
    }

    let mut vault_file = match std::fs::File::create(&vault_file_path) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to create vault file: {e:?} {vault_file_path:?}");
            return Err(format!("Failed to create vault file: {:?}", e));
        }
    };

    match vault_file.write_all(vault_data.to_string().as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            return Err(format!("Failed to write vault file: {:?}", e));
        }
    }

    Ok(())
}

pub fn get_vault_file(world_path: &PathBuf) -> Result<Value, String> {
    let vault_file_path = world_path.join(".chunkvault");

    if !vault_file_path.exists() {
        return Err("Vault file does not exist".to_string());
    }

    let vault_file = match std::fs::File::open(&vault_file_path) {
        Ok(file) => file,
        Err(e) => {
            return Err(format!("Failed to open vault file: {e:?}"));
        }
    };

    let vault_data: Value = match serde_json::from_reader(vault_file) {
        Ok(data) => data,
        Err(e) => {
            return Err(format!("Failed to read vault file: {:?}", e));
        }
    };

    Ok(vault_data)
}

pub fn update_vault_file(vault_data: Value, world_path: &PathBuf) -> Result<(), String> {
    let vault_file_path = world_path.join(".chunkvault");

    if !vault_file_path.exists() {
        return Err("Vault file does not exist".to_string());
    }

    let mut vault_file = match std::fs::File::create(&vault_file_path) {
        Ok(file) => file,
        Err(e) => {
            return Err(format!("Failed to create vault file: {:?}", e));
        }
    };

    match vault_file.write_all(vault_data.to_string().as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            return Err(format!("Failed to write vault file: {:?}", e));
        }
    }

    Ok(())
}

pub fn get_vault_id(path: &PathBuf) -> Result<String, String> {
    let vault_data = match get_vault_file(path) {
        Ok(data) => data,
        Err(_) => {
            let new_vault_id = uuid::Uuid::new_v4().to_string();

            let new_vault_data = serde_json::json!({
                "id": new_vault_id
            });

            match create_vault_file(new_vault_data, path) {
                Ok(_) => {}
                Err(e) => return Err(e),
            };

            match get_vault_file(path) {
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
            match update_vault_file(vault_data, &path) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
            return Ok(new_vault_id);
        }
    };

    Ok(vault_id.to_string())
}

// Minecraft save finder

pub fn is_minecraft_world(path: &Path) -> GameType {
    if !path.is_dir() {
        return GameType::None;
    }

    let java_files = ["level.dat", "region", "data"];
    let bedrock_files = ["level.dat", "db"];

    let is_java = java_files.iter().all(|file| path.join(file).exists());
    let is_bedrock = bedrock_files.iter().all(|file| path.join(file).exists());

    if is_java {
        info!("Found java world at {:?}", path);
        return GameType::Java;
    } else if is_bedrock {
        info!("Found bedrock world at {:?}", path);
        return GameType::Bedrock;
    } else {
        error!(
            "Could not determine if path is a minecraft world: {:?}",
            path
        );

        return GameType::None;
    }
}

pub fn is_minecraft_folder(path: &Path) -> GameType {
    if path.is_dir() {
        if path.file_name().unwrap() == ".minecraft" {
            if !path.join("saves").exists() {
                fs::create_dir_all(path.join("saves")).expect("Failed to create saves directory");
            }
            return GameType::Java;
        } else if path.join("minecraftWorlds").exists() {
            return GameType::Bedrock;
        }
    }

    error!(
        "Could not determine if path is a minecraft folder: {:?}",
        path
    );

    GameType::None
}

pub fn recursive_world_search(
    path: &Path,
    depth: usize,
    max_depth: usize,
    save_folders: &mut Vec<PathBuf>,
) -> Result<(), String> {
    if depth > max_depth {
        return Ok(());
    }

    if !path.exists() {
        return Err(format!("Path {:?} does not exist", path));
    }

    match is_minecraft_world(path) {
        GameType::Java => {
            save_folders.push(path.parent().unwrap().to_path_buf());
        }
        GameType::Bedrock => {
            save_folders.push(path.parent().unwrap().to_path_buf());
        }
        GameType::None => match is_minecraft_folder(path) {
            GameType::Java => {
                save_folders.push(path.join("saves"));
            }
            GameType::Bedrock => {
                save_folders.push(path.join("minecraftWorlds"));
            }
            GameType::None => {
                if let Ok(entries) = path.read_dir() {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let entry_path = entry.path();
                            if entry_path.is_dir() {
                                recursive_world_search(
                                    &entry_path,
                                    depth + 1,
                                    max_depth,
                                    save_folders,
                                )?;
                            }
                        }
                    }
                }
            }
        },
    }

    Ok(())
}

// Data parsing & processing

pub fn read_dat_file(
    file_path: PathBuf,
    game_type: GameType,
) -> Result<NbtValue, Box<dyn std::error::Error>> {
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

pub fn process_world_data(
    path: &PathBuf,
    game_type: GameType,
) -> Result<WorldLevelData, Box<dyn std::error::Error>> {
    info!("Processing world data for: {:?}", path);

    let level_dat = read_dat_file(path.join("level.dat"), game_type)?;

    let level_value: serde_json::Value = serde_json::to_value(level_dat)?;

    match game_type {
        GameType::Bedrock => {
            let world_level_data = WorldLevelData {
                game_engine: "Bedrock".to_string(),
                name: level_value["LevelName"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                folder: Some(path.to_str().unwrap().to_string()),
                icon: match encode_image_to_base64(path.join("world_icon.jpeg")) {
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
                players: get_player_data(path, game_type)?,
                size_on_disk: {
                    info!("Calculating directory size for: {:?}", path);
                    calculate_dir_size(path)? as i64
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
                icon: match encode_image_to_base64(path.join("icon.png")) {
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
                players: get_player_data(path, game_type)?,
                size_on_disk: {
                    info!("Calculating directory size for: {:?}", path);
                    calculate_dir_size(path)? as i64
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

pub fn get_player_data(
    path: &PathBuf,
    game_type: GameType,
) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    match game_type {
        GameType::Bedrock => {
            info!("Fetching Bedrock player data");

            let player_uuid = "~local_player".to_string();
            let player_avatar = "https://crafthead.net/avatar/8667ba71b85a4004af54457a9734eed7?scale=32&overlay=false";

            let db_path = path.join("db").to_str().unwrap().to_string();

            let mut db_reader = commandblock::db::DbReader::new(&db_path, 0);

            let remote_player_data = db_reader.parse_remote_players();

            let mut players: Vec<Value> = Vec::new();

            if remote_player_data.is_some() {
                for (uuid, _) in remote_player_data.unwrap().iter() {
                    info!("Fetching player data for: {:?}", uuid);

                    let player_meta = json!({
                        "username": "Remote Player",
                        "id": uuid.strip_prefix("player_server_").unwrap_or(uuid),
                        "avatar": player_avatar,
                        "meta": {}
                    });

                    players.push(player_meta);
                }
            }

            let local_player_data = json!({
                "username": "Local Player",
                "id": player_uuid,
                "avatar": player_avatar,
                "meta": {}
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
                let player_avatar = "https://crafthead.net/avatar/8667ba71b85a4004af54457a9734eed7?scale=32&overlay=false";

                let player_meta = json!({
                    "username": "Local Player",
                    "id": player_uuid,
                    "avatar": player_avatar,
                    "meta": {}
                });

                return Ok(vec![player_meta]);
            }

            let player_data = match std::fs::read_dir(&player_data_path) {
                Ok(data) => data,
                Err(e) => {
                    return Err(format!("Failed to read player data: {:?}", e).into());
                }
            };

            let mut all_players: Vec<Value> = Vec::new();

            for player in player_data {
                let player = match player {
                    Ok(player) => player,
                    Err(e) => {
                        return Err(format!("Failed to read player data: {:?}", e).into());
                    }
                };

                let player = player.path();

                if !player.is_file()
                    || player.extension().and_then(std::ffi::OsStr::to_str) != Some("dat")
                {
                    continue;
                }

                let player_uuid = player.file_stem().unwrap().to_str().unwrap().to_string();

                let player_meta = match fetch_player_data_from_uuid(player_uuid) {
                    Ok(data) => data,
                    Err(e) => {
                        return Err(format!("Failed to fetch player data: {:?}", e).into());
                    }
                };

                all_players.push(player_meta);
            }

            Ok(all_players)
        }
        GameType::None => Err("Game type not specified".into()),
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

pub fn parse_world_entry_data(path: PathBuf) -> Result<WorldData, String> {
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

    let world_size = match calculate_dir_size(&path) {
        Ok(size) => size,
        Err(_) => 0,
    };

    let vault_id = match get_vault_id(&path) {
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
            GameType::Java => {
                encode_image_to_base64(path.join("icon.png")).unwrap_or("".to_string())
            }
            GameType::Bedrock => {
                encode_image_to_base64(path.join("world_icon.jpeg")).unwrap_or("".to_string())
            }
            GameType::None => "".to_string(),
        },
        path: path.to_string_lossy().into_owned(),
        size: world_size,
        last_played: last_played,
    };

    Ok(world_data)
}
