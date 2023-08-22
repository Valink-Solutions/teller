use std::collections::HashMap;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct DirectorySettings {
    pub paths: HashMap<String, PathBuf>,
}
