use crate::{
    actions::Action::{CreateGame, JoinRoom},
    message::CustomMessage,
    player::Player,
    room::Room,
};
use async_tungstenite::tungstenite::Message;
use eyre::Result;
use futures::channel::mpsc::UnboundedSender;
use std::sync::{Arc, Mutex};

pub type WrappedMainState = Arc<Mutex<MainState>>;

#[derive(Debug, Default)]
pub struct MainState {
    rooms: Vec<Room>,
}

impl MainState {
    pub fn new_wrapped() -> WrappedMainState {
        let main_state = Self::default();
        Arc::new(Mutex::new(main_state))
    }

    pub fn handle_incoming_message(
        &mut self,
        raw_message: Message,
        sender: UnboundedSender<Message>,
    ) -> Result<()> {
        let message: CustomMessage = serde_json::from_str(raw_message.to_text()?)?;
        // dbg!(message.clone());
        match message.action {
            CreateGame => self.handle_create_game(message, sender)?,
            JoinRoom => self.handle_join_room(message, sender)?,
        }
        Ok(())
    }

    fn handle_create_game(
        &mut self,
        message: CustomMessage,
        sender: UnboundedSender<Message>,
    ) -> Result<()> {
        let player_name = message.data.get_player_name()?;
        let player = Player::new(player_name, sender);
        self.create_room(player)?;
        Ok(())
    }

    fn handle_join_room(
        &mut self,
        message: CustomMessage,
        sender: UnboundedSender<Message>,
    ) -> Result<()> {
        let player = Player::new(message.data.get_player_name()?, sender);
        if let Some(room) = self
            .rooms
            .iter_mut()
            .find(|room| room.id == message.data.get_room_id().unwrap())
        {
            room.join(player)?;
        }
        Ok(())
    }

    fn create_room(&mut self, player: Player) -> Result<()> {
        let room = Room::new(player)?;
        self.rooms.push(room);
        Ok(())
    }
}
