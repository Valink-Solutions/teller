use std::path::Path;

mod handler;
pub use handler::*;

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
