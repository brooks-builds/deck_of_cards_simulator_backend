use eyre::Result;
use rand::{thread_rng, Rng};

use crate::{
    actions::Action::JoinRoom,
    message::{CustomMessage, CustomMessageBuilder},
    player::Player,
};

#[derive(Debug)]
pub struct Room {
    pub id: u32,
    players: Vec<Player>,
}

impl Room {
    pub fn new(player: Player) -> Result<Self> {
        let player_name = player.name.clone();
        let mut rng = thread_rng();
        let id = rng.gen_range(1000..=9999);
        let players = vec![player];
        let mut room = Self { id, players };
        let message = CustomMessageBuilder::new()
            .set_action(crate::actions::Action::CreateGame)
            .set_room_id(id)
            .set_player_name(&player_name)
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
            .build()?;
        self.players.push(player);
        self.broadcast_to_room(message)?;
        Ok(())
    }
}
