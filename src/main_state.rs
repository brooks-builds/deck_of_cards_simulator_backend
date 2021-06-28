use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use async_tungstenite::tungstenite::Message;
use eyre::{bail, Result};
use futures::{channel::mpsc::UnboundedSender, SinkExt};
use rand::Rng;

use crate::{card::Card, message::OutgoingMessage, room::Room};

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

    pub fn join_room(&mut self, code: &str, address: SocketAddr) -> Result<u8> {
        if let Some(room) = self.rooms.get_mut(code) {
            room.join(address);
            return Ok(room.get_draw_deck_size());
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

    pub fn handle_draw_card(&mut self, code: &str, address: SocketAddr) -> Option<Card> {
        if let Some(drawn_card) = self.rooms.get_mut(code).map(|room| room.draw(address)) {
            drawn_card
        } else {
            None
        }
    }

    pub fn broadcast_to_everyone_else(
        &mut self,
        room_code: &str,
        address: &SocketAddr,
        message: &OutgoingMessage,
    ) -> Result<()> {
        let room = if let Some(room) = self.rooms.get(room_code) {
            room
        } else {
            bail!("Room {} not found", room_code);
        };

        for room_address in room.get_addresses() {
            if room_address == address {
                continue;
            };

            let sender = self.clients.get_mut(room_address).unwrap();
            let message_text = Message::Text(serde_json::to_string(message)?);
            sender.unbounded_send(message_text)?;
        }

        Ok(())
    }
}
