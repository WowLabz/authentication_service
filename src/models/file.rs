use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct FileData {
    pub name: String,
    pub url: String,
    pub size: u64,
    pub size_unit: String
}