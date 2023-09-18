use serde::{Deserialize, Serialize};
use serde_json::Value;

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
