use std::path::Path;

mod handler;
pub use handler::*;
use log::{error, info};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameType {
    Java,
    Bedrock,
    None,
}

pub fn is_minecraft_world(path: &Path) -> GameType {
    info!("Checking if {:?} is a minecraft world", path);

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
        error!(
            "Could not determine if path is a minecraft world: {:?}",
            path
        );

        return GameType::None;
    }
}

pub fn is_minecraft_folder(path: &Path) -> GameType {
    info!("Checking if {:?} is a minecraft folder", path);

    if path.is_dir() {
        if path.file_name().unwrap() == ".minecraft" || path.join("saves").exists() {
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
