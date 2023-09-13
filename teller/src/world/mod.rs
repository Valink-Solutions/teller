use std::path::Path;

mod handler;
pub use handler::*;
use log::{error, info};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameType {
    Java,
    Bedrock,
    None,
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
