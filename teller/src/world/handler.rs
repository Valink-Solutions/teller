use std::{
    io::Write,
    path::{Path, PathBuf},
};

use log::error;
use serde_json::Value;

use super::{is_minecraft_folder, is_minecraft_world, GameType};

// use crate::{GameType, is_minecraft_world};

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
) -> Result<commandblock::NbtValue, Box<dyn std::error::Error>> {
    match game_type {
        GameType::Java => {
            let dat_blob = match commandblock::read_from_file(
                file_path,
                commandblock::Compression::Gzip,
                commandblock::Endian::Big,
            ) {
                Ok(data) => data,
                Err(e) => return Err(format!("Failed to read level.dat: {e:?}").into()),
            };
            Ok(dat_blob)
        }
        GameType::Bedrock => {
            let dat_blob = match commandblock::read_from_file(
                file_path,
                commandblock::Compression::Uncompressed,
                commandblock::Endian::Little,
            ) {
                Ok(data) => data,
                Err(e) => return Err(format!("Failed to read level.dat: {e:?}").into()),
            };
            Ok(dat_blob)
        }
        GameType::None => Err("Game type not specified".into()),
    }
}

pub fn parse_world_data(
    world_data: commandblock::NbtValue,
    game_type: GameType,
) -> Result<Value, String> {
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

pub fn get_world_player_data(world_path: PathBuf) -> Result<Vec<Value>, String> {
    let game_type = is_minecraft_world(&world_path);

    let player_data_path = world_path.join("playerdata");

    if !player_data_path.exists() {
        return Err("Player data does not exist".to_string());
    }

    let player_data = match std::fs::read_dir(&player_data_path) {
        Ok(data) => data,
        Err(e) => {
            return Err(format!("Failed to read player data: {:?}", e));
        }
    };

    let mut all_players: Vec<Value> = Vec::new();

    for player in player_data {
        let player = match player {
            Ok(player) => player,
            Err(e) => {
                return Err(format!("Failed to read player data: {:?}", e));
            }
        };

        let player = player.path();

        if !player.is_file() {
            continue;
        }

        // let player_data = match commandblock::read_from_file(player, compression, endian_style)

        match game_type {
            GameType::Java => {
                let player_data = match commandblock::read_from_file(
                    player,
                    commandblock::Compression::Gzip,
                    commandblock::Endian::Big,
                ) {
                    Ok(data) => data,
                    Err(e) => {
                        return Err(format!("Failed to read player data: {:?}", e));
                    }
                };

                // println!("{:?}", player_data);

                let player_json = serde_json::to_value(player_data).unwrap();
                all_players.push(player_json);
            }
            GameType::Bedrock => {
                let player_data = match commandblock::read_from_file(
                    player,
                    commandblock::Compression::Uncompressed,
                    commandblock::Endian::Little,
                ) {
                    Ok(data) => data,
                    Err(e) => {
                        return Err(format!("Failed to read player data: {:?}", e));
                    }
                };

                // println!("{:?}", player_data);

                let player_json = serde_json::to_value(player_data).unwrap();
                all_players.push(player_json);
            }
            GameType::None => {}
        }

        // let mut player_file = match std::fs::File::open(player.path()) {
        //     Ok(file) => file,
        //     Err(e) => {
        //         return Err(format!("Failed to open player data: {:?}", e));
        //     }
        // };

        // let player_data = match quartz_nbt::io::read_nbt(&mut player_file, Flavor::GzCompressed) {
        //     Ok((data, _)) => data,
        //     Err(e) => {
        //         return Err(format!("Failed to read player data: {:?}", e));
        //     }
        // };
    }

    Ok(all_players)
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
