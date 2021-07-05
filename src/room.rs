use eyre::Result;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::{
    actions::Action::JoinRoom,
    card::{Card, Suite, Value},
    message::{CustomMessage, CustomMessageBuilder},
    player::Player,
};

#[derive(Debug)]
pub struct Room {
    pub id: u32,
    players: Vec<Player>,
    pub draw_deck: Vec<Card>,
}

impl Room {
    pub fn new(player: Player) -> Result<Self> {
        let player_name = player.name.clone();
        let mut rng = thread_rng();
        let id = rng.gen_range(1000..=9999);
        let player_id = player.id.clone();
        dbg!(&player);
        let players = vec![player];
        let draw_deck = vec![];
        let mut room = Self {
            id,
            players,
            draw_deck,
        };
        room.reset_draw_deck();
        let message = CustomMessageBuilder::new()
            .set_action(crate::actions::Action::CreateGame)
            .set_room_id(id)
            .set_player_name(&player_name)
            .set_draw_deck_size(room.draw_deck.len())
            .set_player_id(player_id)
            .build()?;
        room.broadcast_to_room(message)?;
        Ok(room)
    }

    pub fn broadcast_to_room(&mut self, message: CustomMessage) -> Result<()> {
        for player in &mut self.players {
            player.send(message.clone())?;
        }
        Ok(())
    }

    pub fn broadcast_to_everyone_else(
        &mut self,
        message: CustomMessage,
        player_id: &str,
    ) -> Result<()> {
        for player in &mut self.players {
            if player.id != player_id {
                player.send(message.clone())?;
            }
        }
        Ok(())
    }

    pub fn join(&mut self, mut player: Player) -> Result<()> {
        let message_to_player = CustomMessageBuilder::new()
            .set_action(JoinRoom)
            .set_room_id(self.id)
            .set_player_name(&player.name)
            .set_draw_deck_size(self.draw_deck.len())
            .set_player_id(player.id.clone())
            .build()?;
        player.send(message_to_player)?;
        self.players.push(player.clone());
        let message_to_everyone_else = CustomMessageBuilder::new()
            .set_action(crate::actions::Action::PlayerJoinedRoomInSession)
            .set_player_name(&player.name)
            .set_player_id(player.id.clone())
            .build()?;

        self.broadcast_to_everyone_else(message_to_everyone_else, &player.id)?;
        Ok(())
    }

    pub fn draw_card(&mut self, player_id: &str) -> Result<()> {
        let card = if let Some(card) = self.draw_deck.pop() {
            card
        } else {
            return Ok(());
        };

        let player = if let Some(player) = self
            .players
            .iter_mut()
            .find(|player| player.id == player_id)
        {
            player
        } else {
            return Ok(());
        };

        player.add_card(card);
        let message_to_player = CustomMessageBuilder::new()
            .set_action(crate::actions::Action::DrawCard)
            .set_card(card)
            .set_draw_deck_size(self.draw_deck.len())
            .build()?;
        player.send(message_to_player)?;
        Ok(())
    }

    fn reset_draw_deck(&mut self) {
        self.draw_deck.clear();
        // let ten_of_hearts = Card::new(crate::card::Suite::Heart, crate::card::Value::Ten);
        // self.draw_deck.push(ten_of_hearts);
        for suite in Suite::all().iter() {
            for value in Value::all().iter() {
                let card = Card::new(*suite, *value);
                self.draw_deck.push(card);
            }
        }
        self.shuffle_draw_deck();
    }

    fn shuffle_draw_deck(&mut self) {
        let mut rng = thread_rng();
        self.draw_deck.shuffle(&mut rng);
    }
}
