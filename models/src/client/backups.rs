use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Backup {
    pub uuid: String,
    pub name: String,
    pub ignored_files: Vec<String>,
    pub bytes: u32,
    pub checksum: Option<String>,
    pub is_successful: bool,
    pub is_locked: bool,
    pub created_at: String,
    pub completed_at: Option<String>,
}
