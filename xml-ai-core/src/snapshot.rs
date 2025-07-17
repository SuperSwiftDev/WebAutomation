use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSnapshot {
    pub message_payload: ai_client::request::Message,
    pub evaluation_point: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSnapshot {
    pub messages: Vec<MessageSnapshot>,
}
