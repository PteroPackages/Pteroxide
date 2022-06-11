use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    pub name: String,
    pub mode: String,
    pub mode_bits: String,
    pub size: u64,
    pub is_file: bool,
    pub is_symlink: bool,
    pub mimetype: String,
    pub created_at: String,
    pub modified_at: String,
}

impl File {
    pub fn mode_as_int(&self) -> u64 {
        self.mode_bits.parse::<u64>().unwrap()
    }
}
