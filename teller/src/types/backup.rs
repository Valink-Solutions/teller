use std::collections::HashMap;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct RemoteBackup {
    pub remote_url: String,
    pub api_key: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct BackupSettings {
    pub schedule: String,
    pub auto_backup: bool,
    pub enable_remote_backup: bool,
    pub default_vault: Option<String>,
    pub vaults: HashMap<String, PathBuf>,
    pub remote_vaults: HashMap<String, RemoteBackup>,
}

impl Default for BackupSettings {
    fn default() -> Self {
        Self {
            schedule: "0 0 * * * * *".to_string(),
            auto_backup: false,
            enable_remote_backup: false,
            default_vault: None,
            vaults: HashMap::new(),
            remote_vaults: HashMap::new(),
        }
    }
}
