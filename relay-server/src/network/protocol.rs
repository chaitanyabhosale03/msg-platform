use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    #[serde(rename = "auth")]
    Auth,
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "ack")]
    Ack,
    #[serde(rename = "typing")]
    Typing,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub msg_type: MessageType,
    pub from: String,
    pub to: Option<String>,
    pub payload: serde_json::Value,
    pub timestamp: i64,
    pub signature: Option<String>,
}

impl Message {
    pub fn new(id: String, msg_type: MessageType, from: String, payload: serde_json::Value) -> Self {
        Self {
            id,
            msg_type,
            from,
            to: None,
            payload,
            timestamp: chrono::Utc::now().timestamp(),
            signature: None,
        }
    }
}
