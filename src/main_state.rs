use crate::{actions::Action::CreateGame, message::CustomMessage, player::Player, room::Room};
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
        match message.action {
            CreateGame => self.handle_create_game(message, sender)?,
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

    fn create_room(&mut self, player: Player) -> Result<()> {
        let room = Room::new(player)?;
        self.rooms.push(room);
        Ok(())
    }
}
