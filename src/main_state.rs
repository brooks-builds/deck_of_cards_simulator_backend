use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use async_tungstenite::tungstenite::Message;
use eyre::{bail, Result};
use futures::channel::mpsc::UnboundedSender;
use rand::Rng;

use crate::{
    command::OutgoingEvent,
    message::{IncomingMessage, OutgoingMessage},
    room::Room,
};

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

    fn create_room(&mut self, address: SocketAddr) -> Result<String> {
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

    pub fn join_room(&mut self, event: IncomingMessage, address: SocketAddr) -> Result<()> {
        let room_code = if let Some(room_code) = event.room_code {
            room_code
        } else {
            bail!("Room code not provided");
        };
        if let Some(room) = self.rooms.get_mut(&room_code) {
            room.join(address);
        } else {
            bail!("room doesn't exist");
        }

        // bail!("room with code {} doesn't exist", code);
        let message = OutgoingMessage::new(OutgoingEvent::RoomJoined)?
            .set_draw_deck_size(self.get_draw_deck_size(&room_code))
            .set_room_code(room_code)
            .set_message("Joined room");
        self.send_message_to_address(&address, &message)?;

        Ok(())
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

    pub fn handle_draw_card(
        &mut self,
        incoming_message: IncomingMessage,
        address: SocketAddr,
    ) -> Result<()> {
        let room_code = if let Some(room_code) = incoming_message.room_code {
            room_code
        } else {
            let error_message =
                OutgoingMessage::new(OutgoingEvent::CardDrawn)?.set_error("room code missing");
            self.send_message_to_address(&address, &error_message)?;
            return Ok(());
        };

        if let Some(card) = self
            .rooms
            .get_mut(&room_code)
            .map(|room| room.draw(address))
        {
            let outgoing_message = OutgoingMessage::new(OutgoingEvent::DrawCard)?
                .set_room_code(room_code.clone())
                .set_card(card);

            self.send_message_to_address(&address, &outgoing_message)?;

            let outgoing_message = OutgoingMessage::new(OutgoingEvent::CardDrawn)?
                .set_room_code(room_code.clone())
                .set_draw_deck_size(self.get_draw_deck_size(&room_code));
            self.broadcast_to_room(&room_code, &outgoing_message)?;
        }

        Ok(())
    }

    pub fn create_game(
        &mut self,
        address: SocketAddr,
        incoming_message: IncomingMessage,
    ) -> Result<()> {
        let room_code = self.create_room(address)?;
        let message = OutgoingMessage::new(OutgoingEvent::GameCreated)?
            .set_draw_deck_size(self.get_draw_deck_size(&room_code))
            .set_room_code(room_code)
            .set_message("game created - invite others using the code");
        self.send_message_to_address(&address, &message)?;
        Ok(())
    }

    pub fn handle_chat(
        &mut self,
        incoming_message: IncomingMessage,
        address: SocketAddr,
    ) -> Result<()> {
        let room_code = if let Some(code) = incoming_message.room_code {
            code
        } else {
            let error_message =
                OutgoingMessage::new(OutgoingEvent::Chat)?.set_error("room code missing");
            self.send_message_to_address(&address, &error_message)?;
            return Ok(());
        };

        if let Some(message) = incoming_message.message {
            let outgoing_message = OutgoingMessage::new(OutgoingEvent::Chat)?.set_message(&message);
            self.broadcast_to_room(&room_code, &outgoing_message)?;
        } else {
            let error_message =
                OutgoingMessage::new(OutgoingEvent::Chat)?.set_error("message is missing");
            self.send_message_to_address(&address, &error_message)?;
        }

        Ok(())
    }
}
