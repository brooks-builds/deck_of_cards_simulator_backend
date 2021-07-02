use eyre::Result;
use rand::{thread_rng, Rng};

use crate::{
    message::{CustomMessage, CustomMessageBuilder},
    player::Player,
};

#[derive(Debug)]
pub struct Room {
    id: u32,
    players: Vec<Player>,
}

impl Room {
    pub fn new(player: Player) -> Result<Self> {
        let mut rng = thread_rng();
        let id = rng.gen_range(1000..=9999);
        let players = vec![player];
        let mut room = Self { id, players };
        let message = CustomMessageBuilder::new()
            .set_action(crate::actions::Action::CreateGame)
            .set_room_id(id)
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
}
