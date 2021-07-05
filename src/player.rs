use async_tungstenite::tungstenite::Message;
use eyre::Result;
use futures::channel::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::{card::Card, message::CustomMessage};

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
}
