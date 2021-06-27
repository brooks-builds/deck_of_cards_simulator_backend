use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use async_tungstenite::tungstenite::Message;
use eyre::{bail, Result};
use futures::channel::mpsc::UnboundedSender;
use rand::Rng;

use crate::{message::OutgoingMessage, room::Room};

pub type WrappedMainState = Arc<Mutex<MainState>>;

#[derive(Debug, Default)]
pub struct MainState {
    clients: HashMap<SocketAddr, UnboundedSender<Message>>,
    rooms: HashMap<String, Room>,
}

impl MainState {
    pub fn new_wrapped() -> WrappedMainState {
        let main_state = Self::default();
        Arc::new(Mutex::new(main_state))
    }

    pub fn create_room(&mut self, address: SocketAddr) -> Result<String> {
        let mut rng = rand::thread_rng();
        let code = rng.gen_range(1_000..10_000).to_string();
        if self.rooms.contains_key(&code) {
            bail!("room already exists");
        }
        self.rooms.insert(code.clone(), Room::new(address));
        Ok(code)
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

    pub fn join_room(&mut self, code: &str, address: SocketAddr) -> Result<()> {
        if let Some(room) = self.rooms.get_mut(code) {
            room.join(address);
            return Ok(());
        }

        bail!("room with code {} doesn't exist", code);
    }

    pub fn broadcast_to_room(&mut self, code: &str, message: &OutgoingMessage) -> Result<()> {
        let room = if let Some(room) = self.rooms.get(code) {
            room
        } else {
            bail!("Room {} not found", code);
        };

        for address in room.get_addresses() {
            let sender = self.clients.get_mut(address).unwrap();
            let stringified_message = Message::Text(serde_json::to_string(message)?);
            sender.unbounded_send(stringified_message)?;
        }
        Ok(())
    }

    pub fn get_draw_deck_size(&self, code: &str) -> Option<u8> {
        self.rooms.get(code).map(|room| room.get_draw_deck_size())
    }
}
