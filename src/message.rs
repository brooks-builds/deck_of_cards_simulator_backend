use async_tungstenite::tungstenite::Message;
use eyre::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::actions::Action;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CustomMessage {
    pub action: Action,
    pub data: MessageData,
}

#[allow(clippy::clippy::from_over_into)]
impl Into<Message> for CustomMessage {
    fn into(self) -> Message {
        Message::Text(serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct MessageData {
    player_name: Option<String>,
    room_id: Option<u32>,
    message: Option<String>,
}

impl MessageData {
    pub fn get_player_name(&self) -> Result<&str> {
        if let Some(player_name) = &self.player_name {
            Ok(player_name)
        } else {
            bail!("Player name doesn't exist");
        }
    }

    pub fn get_room_id(&self) -> Result<u32> {
        if let Some(room_id) = self.room_id {
            Ok(room_id)
        } else {
            bail!("Room id doesn't exist");
        }
    }

    pub fn get_message(&self) -> Result<&str> {
        if let Some(message) = &self.message {
            Ok(message)
        } else {
            bail!("Message doesn't exist");
        }
    }
}

#[derive(Debug, Default)]
pub struct CustomMessageBuilder {
    action: Option<Action>,
    data: MessageData,
}

impl CustomMessageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }

    pub fn set_room_id(mut self, room_id: u32) -> Self {
        self.data.room_id = Some(room_id);
        self
    }

    pub fn set_player_name(mut self, player_name: &str) -> Self {
        self.data.player_name = Some(player_name.to_owned());
        self
    }

    pub fn set_message(mut self, message: &str) -> Self {
        self.data.message = Some(message.to_owned());
        self
    }

    pub fn build(self) -> Result<CustomMessage> {
        let action = if let Some(action) = self.action {
            action
        } else {
            bail!("Action is not set");
        };
        Ok(CustomMessage {
            action,
            data: self.data,
        })
    }
}
