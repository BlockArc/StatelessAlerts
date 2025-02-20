use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Alert {
    pub id: u32,
    pub message: String,
}
