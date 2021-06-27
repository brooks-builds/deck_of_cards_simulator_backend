use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    CreateGame,
    JoinRoom,
    Chat,
    DrawCard,
}
