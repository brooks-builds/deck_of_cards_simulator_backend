use crate::{
    actions::Action::{
        Chat, CreateGame, DrawCard, DrawDeckUpdated, JoinRoom, ToggleVisibilityOfCard,
    },
    message::{CustomMessage, CustomMessageBuilder},
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
        match message.action {
            CreateGame => self.handle_create_game(message, sender)?,
            JoinRoom => self.handle_join_room(message, sender)?,
            Chat => self.handle_chat(message)?,
            DrawCard => self.handle_draw_card(message)?,
            ToggleVisibilityOfCard => self.handle_toggle_visibility_of_card(message)?,
            _ => {}
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

    fn handle_chat(&mut self, message: CustomMessage) -> Result<()> {
        if let Some(room) = self
            .rooms
            .iter_mut()
            .find(|room| room.id == message.data.get_room_id().unwrap())
        {
            let outgoing_message = CustomMessageBuilder::new()
                .set_action(message.action)
                .set_player_name(message.data.get_player_name()?)
                .set_room_id(message.data.get_room_id().unwrap())
                .set_message(message.data.get_message()?)
                .build()?;
            room.broadcast_to_room(outgoing_message).unwrap();
        }
        Ok(())
    }

    fn handle_draw_card(&mut self, message: CustomMessage) -> Result<()> {
        if let Some(room) = self
            .rooms
            .iter_mut()
            .find(|room| room.id == message.data.get_room_id().unwrap())
        {
            room.draw_card(message.data.get_player_id()?)?;
            let message = CustomMessageBuilder::new()
                .set_action(DrawDeckUpdated)
                .set_draw_deck_size(room.draw_deck.len())
                .set_player_id(message.data.get_player_id()?.to_owned())
                .build()?;
            room.broadcast_to_room(message).unwrap();
        }
        Ok(())
    }

    fn handle_toggle_visibility_of_card(&mut self, message: CustomMessage) -> Result<()> {
        if let Some(room) = self
            .rooms
            .iter_mut()
            .find(|room| room.id == message.data.get_room_id().unwrap())
        {
            room.toggle_visibility_of_card(
                message.data.get_player_id()?,
                message.data.get_card()?,
            )?;
        }

        Ok(())
    }

    fn create_room(&mut self, player: Player) -> Result<()> {
        let room = Room::new(player)?;
        self.rooms.push(room);
        Ok(())
    }
}
