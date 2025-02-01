use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Join { name: String },
    Chat { content: String },
    Leave ,
}
