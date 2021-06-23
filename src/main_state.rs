use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use async_tungstenite::tungstenite::Message;
use eyre::Result;
use futures::channel::mpsc::UnboundedSender;
use rand::Rng;

use crate::message::OutgoingMessage;

pub type WrappedMainState = Arc<Mutex<MainState>>;

#[derive(Debug, Default)]
pub struct MainState {
    clients: HashMap<SocketAddr, UnboundedSender<Message>>,
}

impl MainState {
    pub fn new_wrapped() -> WrappedMainState {
        let main_state = Self::default();
        Arc::new(Mutex::new(main_state))
    }

    pub fn create_room(&mut self) -> String {
        let mut rng = rand::thread_rng();
        let code = rng.gen_range(1_000..10_000);
        code.to_string()
    }

    pub fn add_client(&mut self, address: SocketAddr, sender: UnboundedSender<Message>) {
        self.clients.insert(address, sender);
    }

    pub fn send_message_to_address(
        &mut self,
        address: &SocketAddr,
        message: &OutgoingMessage,
    ) -> Result<()> {
        if let Some(sender) = self.clients.get_mut(address) {
            sender.unbounded_send(Message::Text(serde_json::to_string(message)?))?;
        }
        Ok(())
    }
}
