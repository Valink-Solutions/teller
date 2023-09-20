use std::collections::HashMap;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct DirectorySettings {
    pub categories: HashMap<String, VaultEntries>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct VaultEntries {
    pub paths: HashMap<String, PathBuf>,
}
