use crate::{
    card::{Card, CardData},
    message::CustomMessage,
};
use async_tungstenite::tungstenite::Message;
use eyre::Result;
use futures::channel::mpsc::UnboundedSender;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    sender: UnboundedSender<Message>,
    pub id: String,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(name: &str, sender: UnboundedSender<Message>) -> Self {
        Self {
            name: name.to_owned(),
            sender,
            id: Uuid::new_v4().to_string(),
            hand: vec![],
        }
    }

    pub fn send(&mut self, message: CustomMessage) -> Result<()> {
        self.sender.unbounded_send(message.into())?;
        Ok(())
    }

    pub fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn find_card(&mut self, message_card: &Card) -> Option<&mut Card> {
        self.hand
            .iter_mut()
            .find(|card| card.suite == message_card.suite && card.value == message_card.value)
    }

    pub fn toggle_visibility_of_card(&mut self, message_card: &Card) -> Option<Card> {
        if let Some(card) = self.find_card(message_card) {
            card.toggle_visibility();
            Some(*card)
        } else {
            None
        }
    }

    pub fn player_data(&self) -> PlayerData {
        PlayerData {
            name: self.name.clone(),
            id: self.id.clone(),
            hand: self.hand.iter().map(|card| card.card_data()).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerData {
    name: String,
    id: String,
    hand: Vec<CardData>,
}
