use crate::command::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IncommingMessage {
    pub command: Command,
    pub room_code: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OutgoingMessage {
    room_code: Option<String>,
    error: Option<String>,
    message: Option<String>,
    chat_message: Option<String>,
    draw_deck_size: Option<u8>,
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

    pub fn set_chat_message(&mut self, message: String) {
        self.chat_message = Some(message);
    }

    pub fn set_draw_deck_size(&mut self, size: u8) {
        self.draw_deck_size = Some(size);
    }
}
