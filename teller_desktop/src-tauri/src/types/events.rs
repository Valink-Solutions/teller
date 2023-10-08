#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackupEvent {
    pub world_id: String,
    pub category: Option<String>,
    pub vaults: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToastEvent {
    pub message: String,
}
