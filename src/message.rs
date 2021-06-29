use crate::{card::Card, command::Command};
use eyre::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomingMessage {
    pub command: Command,
    pub room_code: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct OutgoingMessage {
    room_code: Option<String>,
    error: Option<String>,
    message: Option<String>,
    chat_message: Option<String>,
    draw_deck_size: Option<u8>,
    card: Option<Card>,
    command: Command,
}

impl OutgoingMessage {
    pub fn new(command: Command) -> Result<Self> {
        let mut message = Self::default();
        if command == Command::None {
            bail!("Command needs to be a real command");
        }
        message.command = command;
        Ok(message)
    }

    pub fn set_room_code(mut self, code: String) -> Self {
        self.room_code = Some(code);
        self
    }

    pub fn set_draw_deck_size(mut self, draw_deck_size: Option<u8>) -> Self {
        self.draw_deck_size = draw_deck_size;
        self
    }

    pub fn set_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_owned());
        self
    }

    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    pub fn set_chat_message(&mut self, message: String) {
        self.chat_message = Some(message);
    }

    pub fn set_card(&mut self, card: Card) {
        self.card = Some(card);
    }

    pub fn remove_card(&mut self) {
        self.card = None;
    }

    pub fn set_command(&mut self, command: Command) {
        self.command = command;
    }
}
