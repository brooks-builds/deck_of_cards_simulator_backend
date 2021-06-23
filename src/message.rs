use crate::command::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IncommingMessage {
    pub command: Command,
    pub room_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutgoingMessage {
    room_code: Option<String>,
}

impl OutgoingMessage {
    pub fn new(room_code: Option<String>) -> Self {
        Self { room_code }
    }
}
