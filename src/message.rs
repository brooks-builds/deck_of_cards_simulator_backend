use crate::command::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IncommingMessage {
    pub command: Command,
    pub room_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OutgoingMessage {
    room_code: Option<String>,
    error: Option<String>,
    message: Option<String>,
}

impl OutgoingMessage {
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    pub fn set_room_code(&mut self, code: String) {
        self.room_code = Some(code);
    }

    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }
}
