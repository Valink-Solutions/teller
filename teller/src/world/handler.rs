use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use commandblock::nbt::{read_from_file, Compression, Endian, NbtValue};
use log::error;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::utils::{
    player_handler::fetch_player_data_from_uuid, GameRules, Item, PlayerData, WorldLevelData,
};

use super::{is_minecraft_folder, is_minecraft_world, GameType};

pub fn create_vault_file(vault_data: Value, world_path: &PathBuf) -> Result<(), String> {
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

pub fn parse_dat_file(
    file_path: PathBuf,
    game_type: GameType,
) -> Result<NbtValue, Box<dyn std::error::Error>> {
    match game_type {
        GameType::Java => {
            let (_, dat_blob) = match read_from_file(file_path, Compression::Gzip, Endian::Big) {
                Ok(data) => data,
                Err(e) => return Err(format!("Failed to read level.dat: {e:?}").into()),
            };
            Ok(dat_blob)
        }
        GameType::Bedrock => {
            let (_, dat_blob) =
                match read_from_file(file_path, Compression::Uncompressed, Endian::Little) {
                    Ok(data) => data,
                    Err(e) => return Err(format!("Failed to read level.dat: {e:?}").into()),
                };
            Ok(dat_blob)
        }
        GameType::None => Err("Game type not specified".into()),
    }
}

pub fn parse_world_data(world_data: NbtValue, game_type: GameType) -> Result<Value, String> {
    let level_value: serde_json::Value = match serde_json::to_value(world_data) {
        Ok(value) => value,
        Err(e) => return Err(format!("Failed to parse level.dat JSON: {:?}", e)),
    };

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
    let level_dat = parse_dat_file(path.join("level.dat"), game_type)?;

    let level_value: serde_json::Value = serde_json::to_value(level_dat)?;

    match game_type {
        GameType::Bedrock => {
            let world_level_data = WorldLevelData {
                name: level_value["LevelName"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                difficulty: {
                    let difficulty = level_value["Difficulty"].as_i64().unwrap_or_default() as i32;
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
                    let naive_datetime = chrono::NaiveDateTime::from_timestamp_millis(last_played);
                    naive_datetime
                },
                players: Vec::new(),
                size_on_disk: calculate_dir_size(path)? as i64,
                game_rules: Some(parse_game_rules(&level_value, game_type)?),
            };

            return Ok(world_level_data);
        }
        GameType::Java => {
            let level_data = match level_value.get("Data") {
                Some(data) => data,
                None => return Err("Could not find Data in level.dat".into()),
            };

            let world_level_data = WorldLevelData {
                name: level_data["LevelName"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                difficulty: {
                    let difficulty = level_data["Difficulty"].as_i64().unwrap_or_default() as i32;
                    match difficulty {
                        0 => "Peaceful".to_string(),
                        1 => "Easy".to_string(),
                        2 => "Normal".to_string(),
                        3 => "Hard".to_string(),
                        _ => "Unknown".to_string(),
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
                size_on_disk: calculate_dir_size(path)? as i64,
                game_rules: Some(parse_game_rules(level_data, game_type)?),
            };

            return Ok(world_level_data);
        }
        GameType::None => return Err("Game type not specified".into()),
    }
}

pub fn get_player_data(
    path: &PathBuf,
    game_type: GameType,
) -> Result<Vec<PlayerData>, Box<dyn std::error::Error>> {
    match game_type {
        GameType::Bedrock => Ok(Vec::new()),
        GameType::Java => {
            let player_data_path = path.join("playerdata");

            if !player_data_path.exists() {
                return Err("Player data does not exist".into());
            }

            let player_data = match std::fs::read_dir(&player_data_path) {
                Ok(data) => data,
                Err(e) => {
                    return Err(format!("Failed to read player data: {:?}", e).into());
                }
            };

            let mut all_players: Vec<PlayerData> = Vec::new();

            for player in player_data {
                let player = match player {
                    Ok(player) => player,
                    Err(e) => {
                        return Err(format!("Failed to read player data: {:?}", e).into());
                    }
                };

                let player = player.path();

                if !player.is_file() || !player.extension().unwrap().eq("dat") {
                    continue;
                }

                let player_data = match commandblock::nbt::read_from_file(
                    player.clone(),
                    commandblock::nbt::Compression::Gzip,
                    commandblock::nbt::Endian::Big,
                ) {
                    Ok((_, data)) => serde_json::to_value(data)?,
                    Err(e) => {
                        return Err(format!("Failed to read player data: {:?}", e).into());
                    }
                };

                let player_uuid = Uuid::parse_str(player.file_stem().unwrap().to_str().unwrap())
                    .unwrap_or_default();

                // let player_meta = fetch_player_data_from_uuid(player_uuid)?;

                let player_data = PlayerData {
                    id: player_uuid,
                    health: player_data.get("Health").unwrap().as_f64().unwrap() as f32,
                    food: player_data.get("foodLevel").unwrap().as_i64().unwrap() as i32,
                    level: player_data.get("XpLevel").unwrap().as_i64().unwrap() as i32,
                    xp: player_data.get("XpTotal").unwrap().as_i64().unwrap() as i32,
                    inventory: player_data
                        .get("Inventory")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|item| Item {
                            id: item.get("id").unwrap().as_str().unwrap().to_string(),
                            slot: item.get("Slot").unwrap().as_i64().unwrap() as i32,
                            count: item.get("Count").unwrap().as_i64().unwrap() as i32,
                        })
                        .collect::<Vec<Item>>(),
                };
                all_players.push(player_data);
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
                    do_fire_tick: rules["doFireTick"].as_bool().unwrap_or_default(),
                    mob_loot: rules["doMobLoot"].as_bool().unwrap_or_default(),
                    keep_inventory: rules["keepInventory"].as_bool().unwrap_or_default(),
                    do_mob_spawning: rules["doMobSpawning"].as_bool().unwrap_or_default(),
                    do_tile_drops: rules["doTileDrops"].as_bool().unwrap_or_default(),
                    command_block_output: rules["commandBlockOutput"].as_bool().unwrap_or_default(),
                    natural_regeneration: rules["naturalRegeneration"]
                        .as_bool()
                        .unwrap_or_default(),
                    do_daylight_cycle: rules["doDaylightCycle"].as_bool().unwrap_or_default(),
                    do_weather_cycle: rules["doWeatherCycle"].as_bool().unwrap_or_default(),
                    do_immediate_respawn: rules["doImmediateRespawn"].as_bool().unwrap_or_default(),
                    drowning_damage: rules["drowningDamage"].as_bool().unwrap_or_default(),
                    fall_damage: rules["fallDamage"].as_bool().unwrap_or_default(),
                    fire_damage: rules["fireDamage"].as_bool().unwrap_or_default(),
                    do_insomnia: rules["doInsomnia"].as_bool().unwrap_or_default(),
                    invulnerable: rules["invulnerable"].as_bool().unwrap_or_default(),
                    max_command_chain_length: rules["maxCommandChainLength"]
                        .as_i64()
                        .unwrap_or_default() as i32,
                    random_tick_speed: rules["randomTickSpeed"].as_i64().unwrap_or_default() as i32,
                    reduced_debug_info: rules["reducedDebugInfo"].as_bool().unwrap_or_default(),
                    send_command_feedback: rules["sendCommandFeedback"]
                        .as_bool()
                        .unwrap_or_default(),
                    show_death_messages: rules["showDeathMessages"].as_bool().unwrap_or_default(),
                    spawn_radius: rules["spawnRadius"].as_i64().unwrap_or_default() as i32,
                    spectators_generate_chunks: rules["spectatorsGenerateChunks"]
                        .as_bool()
                        .unwrap_or_default(),
                },
                None => return Err("Could not find GameRules in level.dat".into()),
            };
            Ok(game_rules)
        }
        GameType::Bedrock => {
            let game_rules = GameRules {
                do_fire_tick: game_data["dofiretick"].as_bool().unwrap_or_default(),
                mob_loot: game_data["mobloot"].as_bool().unwrap_or_default(),
                keep_inventory: game_data["keepinventory"].as_bool().unwrap_or_default(),
                do_mob_spawning: game_data["domobspawning"].as_bool().unwrap_or_default(),
                do_tile_drops: game_data["dotiledrops"].as_bool().unwrap_or_default(),
                command_block_output: game_data["commandblockoutput"]
                    .as_bool()
                    .unwrap_or_default(),
                natural_regeneration: game_data["naturalregeneration"]
                    .as_bool()
                    .unwrap_or_default(),
                do_daylight_cycle: game_data["dodaylightcycle"].as_bool().unwrap_or_default(),
                do_weather_cycle: game_data["doweathercycle"].as_bool().unwrap_or_default(),
                do_immediate_respawn: game_data["doimmediaterespawn"]
                    .as_bool()
                    .unwrap_or_default(),
                drowning_damage: game_data["drowningdamage"].as_bool().unwrap_or_default(),
                fall_damage: game_data["falldamage"].as_bool().unwrap_or_default(),
                fire_damage: game_data["firedamage"].as_bool().unwrap_or_default(),
                do_insomnia: game_data["doinsomnia"].as_bool().unwrap_or_default(),
                invulnerable: game_data["invulnerable"].as_bool().unwrap_or_default(),
                max_command_chain_length: game_data["maxcommandchainlength"]
                    .as_i64()
                    .unwrap_or_default() as i32,
                random_tick_speed: game_data["randomtickspeed"].as_i64().unwrap_or_default() as i32,
                reduced_debug_info: game_data["reduceddebuginfo"].as_bool().unwrap_or_default(),
                send_command_feedback: game_data["sendcommandfeedback"]
                    .as_bool()
                    .unwrap_or_default(),
                show_death_messages: game_data["showdeathmessages"].as_bool().unwrap_or_default(),
                spawn_radius: game_data["spawnradius"].as_i64().unwrap_or_default() as i32,
                spectators_generate_chunks: game_data["spectatorsgeneratechunks"]
                    .as_bool()
                    .unwrap_or_default(),
            };
            Ok(game_rules)
        }
        GameType::None => Err("Game type not specified".into()),
    }
}

pub fn get_world_data(world_path: &PathBuf) -> Result<Value, String> {
    let game_type = is_minecraft_world(&world_path);

    let level_dat_path = world_path.join("level.dat");

    if !level_dat_path.exists() {
        return Err("level.dat does not exist".to_string());
    }

    let level_data = match parse_dat_file(level_dat_path, game_type) {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to parse level.dat: {:?}", e)),
    };

    let level_data = match parse_world_data(level_data, game_type) {
        Ok(data) => data,
        Err(e) => return Err(e),
    };

    Ok(level_data)
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

pub fn calculate_dir_size<P: AsRef<Path>>(path: P) -> std::io::Result<u64> {
    let mut size = 0;

    for entry in fs::read_dir(path)? {
        let dir = entry?;
        let metadata = dir.metadata()?;

        if metadata.is_dir() {
            size += calculate_dir_size(dir.path())?;
        } else {
            size += metadata.len();
        }
    }

    Ok(size)
}
