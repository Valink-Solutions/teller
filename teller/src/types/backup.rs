use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::world::{WorldData, WorldLevelData};

#[derive(Deserialize, Serialize, Clone)]
pub struct RemoteBackup {
    pub remote_url: String,
    pub api_key: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BackupSettings {
    pub schedule: String,
    pub auto_backup: bool,
    pub default_vaults: Option<Vec<String>>,
    pub vaults: HashMap<String, PathBuf>,
    pub remote_vaults: HashMap<String, RemoteBackup>,
}

impl Default for BackupSettings {
    fn default() -> Self {
        Self {
            schedule: "0 0 * * * * *".to_string(),
            auto_backup: false,
            default_vaults: Some(Vec::new()),
            vaults: HashMap::new(),
            remote_vaults: HashMap::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BackupMetadata {
    pub entry: WorldData,
    pub data: WorldLevelData,
}
