use eyre::Result;
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
    draw_deck: Vec<Card>,
}

impl Room {
    pub fn new(player: Player) -> Result<Self> {
        let player_name = player.name.clone();
        let mut rng = thread_rng();
        let id = rng.gen_range(1000..=9999);
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

    pub fn join(&mut self, player: Player) -> Result<()> {
        let message = CustomMessageBuilder::new()
            .set_action(JoinRoom)
            .set_room_id(self.id)
            .set_player_name(&player.name)
            .set_draw_deck_size(self.draw_deck.len())
            .build()?;
        self.players.push(player);
        self.broadcast_to_room(message)?;
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
    }
}
