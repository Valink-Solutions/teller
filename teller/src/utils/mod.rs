pub mod player_handler;

use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use base64::{engine::general_purpose, Engine as _};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldData {
    pub id: String,
    pub name: String,
    pub image: String,
    pub path: String,
    pub size: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct World {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub seed: i64,
    pub current_version: i32,
    pub edition: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorldVersion {
    pub id: String,
    pub world_id: Uuid,
    pub version: i32,
    pub backup_path: String,
    pub created_at: Option<NaiveDateTime>,
    pub difficulty: String,
    pub allow_cheats: bool,
    pub difficulty_locked: bool,
    pub spawn_x: i32,
    pub spawn_y: i32,
    pub spawn_z: i32,
    pub time: i64,
    pub weather: String,
    pub hardcore: bool,
    pub do_daylight_cycle: bool,
    pub do_mob_spawning: bool,
    pub do_weather_cycle: bool,
    pub keep_inventory: bool,
    pub size: i64,
    pub level_name: String,
    pub additional_data: Option<serde_json::Value>,
}

// Experimental data structures

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerData {
    pub id: String,
    // pub name: String,
    pub health: Option<f32>,
    pub food: Option<i32>,
    pub game_mode: Option<i32>,
    pub level: i32,
    pub xp: f32,
    pub inventory: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub slot: Option<i32>,
    pub count: i32,
    pub damage: Option<i32>,
    pub tag: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldLevelData {
    pub name: String,
    pub folder: Option<String>,
    pub icon: Option<String>,
    pub difficulty: String,
    pub game_engine: String,
    pub game_type: String,
    pub last_played: Option<NaiveDateTime>,
    pub size_on_disk: i64,
    pub players: Vec<Value>,
    pub game_rules: Option<GameRules>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameRules {
    pub do_fire_tick: bool,
    pub mob_loot: bool,
    pub keep_inventory: bool,
    pub do_mob_spawning: bool,
    pub do_tile_drops: bool,
    pub command_block_output: bool,
    pub natural_regeneration: bool,
    pub do_daylight_cycle: bool,
    pub do_weather_cycle: bool,
    pub do_immediate_respawn: bool,
    pub drowning_damage: bool,
    pub fall_damage: bool,
    pub fire_damage: bool,
    pub do_insomnia: bool,
    pub invulnerable: bool,
    pub max_command_chain_length: i64,
    pub random_tick_speed: i64,
    pub reduced_debug_info: bool,
    pub send_command_feedback: bool,
    pub show_death_messages: bool,
    pub spawn_radius: i64,
    pub spectators_generate_chunks: bool,
}

pub fn encode_image_to_base64(path: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = fs::File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let res_base64 = general_purpose::STANDARD_NO_PAD.encode(&buf);
    Ok(format!("data:image/png;base64,{}", res_base64))
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
