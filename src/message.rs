use async_tungstenite::tungstenite::Message;
use eyre::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::{
    actions::Action,
    card::{Card, CardData},
    player::PlayerData,
};

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
    draw_deck_size: Option<usize>,
    player_id: Option<String>,
    card: Option<Card>,
    other_players: Option<Vec<PlayerData>>,
    discard_pile: Option<Vec<Card>>,
    hand: Option<Vec<CardData>>,
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

    pub fn get_player_id(&self) -> Result<&str> {
        if let Some(player_id) = &self.player_id {
            Ok(player_id)
        } else {
            bail!("Player id doesn't exist");
        }
    }

    pub fn get_card(&self) -> Result<&Card> {
        if let Some(card) = &self.card {
            Ok(card)
        } else {
            bail!("Card doesn't exist");
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

    pub fn set_draw_deck_size(mut self, draw_deck_size: usize) -> Self {
        self.data.draw_deck_size = Some(draw_deck_size);
        self
    }

    pub fn set_player_id(mut self, player_id: String) -> Self {
        self.data.player_id = Some(player_id);
        self
    }

    pub fn set_card(mut self, card: Card) -> Self {
        self.data.card = Some(card);
        self
    }

    pub fn set_other_players(mut self, other_players: Vec<PlayerData>) -> Self {
        self.data.other_players = Some(other_players);
        self
    }

    pub fn set_discard_pile(mut self, discard_pile: Vec<Card>) -> Self {
        self.data.discard_pile = Some(discard_pile);
        self
    }

    pub fn set_hand(mut self, hand: Vec<Card>) -> Self {
        let sanitized_hand = hand.iter().map(|card| card.card_data()).collect();
        self.data.hand = Some(sanitized_hand);

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
