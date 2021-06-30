use crate::{
    card::Card,
    command::{Command, OutgoingEvent},
};
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
    event: OutgoingEvent,
}

impl OutgoingMessage {
    pub fn new(event: OutgoingEvent) -> Result<Self> {
        let mut message = Self::default();
        if event == OutgoingEvent::None {
            bail!("OutgoingEvent needs to be a real event");
        }
        message.event = event;
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

    pub fn set_error(mut self, error: &str) -> Self {
        self.error = Some(error.to_owned());
        self
    }

    pub fn set_card(mut self, card: Option<Card>) -> Self {
        self.card = card;
        self
    }
}
