use eyre::Result;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::player::PlayerData;
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
    pub discard_deck: Vec<Card>,
}

impl Room {
    pub fn new(player: Player) -> Result<Self> {
        let player_name = player.name.clone();
        let mut rng = thread_rng();
        let id = rng.gen_range(1000..=9999);
        let player_id = player.id.clone();
        let players = vec![player];
        let draw_deck = vec![];
        let discard_deck = vec![];
        let mut room = Self {
            id,
            players,
            draw_deck,
            discard_deck,
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
        let other_players: Vec<PlayerData> = self
            .players
            .iter()
            .map(|player| player.player_data())
            .collect();
        let message_to_player = CustomMessageBuilder::new()
            .set_action(JoinRoom)
            .set_room_id(self.id)
            .set_player_name(&player.name)
            .set_draw_deck_size(self.draw_deck.len())
            .set_player_id(player.id.clone())
            .set_other_players(other_players)
            .set_discard_pile(self.discard_deck.clone())
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

    pub fn toggle_visibility_of_card(&mut self, player_id: &str, card: &Card) -> Result<()> {
        let player = if let Some(player) = self
            .players
            .iter_mut()
            .find(|player| player.id == player_id)
        {
            player
        } else {
            return Ok(());
        };

        if let Some(card) = player.toggle_visibility_of_card(card) {
            let message_to_all_players = CustomMessageBuilder::new()
                .set_action(crate::actions::Action::ToggleVisibilityOfCard)
                .set_card(card)
                .set_player_id(player.id.clone())
                .build()?;
            self.broadcast_to_room(message_to_all_players)?;
        }

        Ok(())
    }

    pub fn discard_card(&mut self, player_id: &str, card: &Card) -> Result<()> {
        let player = if let Some(player) = self
            .players
            .iter_mut()
            .find(|player| player.id == player_id)
        {
            player
        } else {
            return Ok(());
        };
        if let Some(discarded_card) = player.discard_card(card) {
            self.discard_deck.push(discarded_card);
            let message_to_all_players = CustomMessageBuilder::new()
                .set_action(crate::actions::Action::DiscardCard)
                .set_card(discarded_card)
                .set_player_id(player.id.clone())
                .set_hand(player.hand.clone())
                .build()?;
            self.broadcast_to_room(message_to_all_players)?;
        }
        Ok(())
    }

    pub fn reset_deck(&mut self) -> Result<()> {
        self.reset_draw_deck();
        self.shuffle_draw_deck();
        self.discard_deck.clear();
        for player in &mut self.players {
            player.empty_hand();
        }
        let message_to_all_players = CustomMessageBuilder::new()
            .set_action(crate::actions::Action::ResetDeck)
            .set_discard_pile(vec![])
            .set_draw_deck_size(self.draw_deck.len())
            .set_message("Deck reset and shuffled")
            .build()?;
        self.broadcast_to_room(message_to_all_players)?;
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

    pub fn remove_player_by_id(&mut self, player_id: &str) -> Result<()> {
        let mut player_index = None;
        let mut player_name = None;
        for (index, player) in self.players.iter_mut().enumerate() {
            if player.id != player_id {
                continue;
            }

            player_index = Some(index);
            self.discard_deck.append(&mut player.hand);
            break;
        }
        if let Some(player_index) = player_index {
            player_name = Some(self.players[player_index].name.clone());
            self.players.remove(player_index);
        }
        let text_message = format!("{} left the room", player_name.unwrap());
        let message_to_all_players = CustomMessageBuilder::new()
            .set_action(crate::actions::Action::Quit)
            .set_player_id(player_id.to_owned())
            .set_discard_pile(self.discard_deck.clone())
            .set_message(&text_message)
            .build()?;
        self.broadcast_to_room(message_to_all_players)?;
        Ok(())
    }
}
