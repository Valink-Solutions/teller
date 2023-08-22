use std::path::{Path, PathBuf};

use log::info;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameType {
    Java,
    Bedrock,
    None,
}

#[tauri::command]
pub fn check_path_for_save_folders(path: PathBuf) -> Result<Vec<PathBuf>, String> {
    let mut save_folders = Vec::new();
    let max_depth = 6;

    recursive_world_search(&path, 0, max_depth, &mut save_folders)?;

    save_folders.sort();
    save_folders.dedup();

    info!("Found Minecraft worlds: {:?}", save_folders);

    Ok(save_folders)
}

fn recursive_world_search(
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

pub fn is_minecraft_world(path: &Path) -> GameType {
    if !path.is_dir() {
        return GameType::None;
    }

    let java_files = ["level.dat", "region", "data"];
    let bedrock_files = ["level.dat", "db"];

    let is_java = java_files.iter().all(|file| path.join(file).exists());
    let is_bedrock = bedrock_files.iter().all(|file| path.join(file).exists());

    if is_java {
        return GameType::Java;
    } else if is_bedrock {
        return GameType::Bedrock;
    } else {
        return GameType::None;
    }
}

pub fn is_minecraft_folder(path: &Path) -> GameType {
    if path.is_dir() {
        if path.file_name().unwrap() == ".minecraft" || path.join("saves").exists() {
            return GameType::Java;
        } else if path.join("minecraftWorlds").exists() {
            return GameType::Bedrock;
        }
    }
    GameType::None
}

pub fn create_worlds_database() -> Result<(), String> {
    Ok(())
}
