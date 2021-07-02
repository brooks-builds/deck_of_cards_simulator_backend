use async_tungstenite::tungstenite::Message;
use eyre::Result;
use futures::channel::mpsc::UnboundedSender;

use crate::message::CustomMessage;

#[derive(Debug)]
pub struct Player {
    name: String,
    sender: UnboundedSender<Message>,
}

impl Player {
    pub fn new(name: &str, sender: UnboundedSender<Message>) -> Self {
        Self {
            name: name.to_owned(),
            sender,
        }
    }

    pub fn send(&mut self, message: CustomMessage) -> Result<()> {
        self.sender.unbounded_send(message.into())?;
        Ok(())
    }
}
