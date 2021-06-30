use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Command {
    CreateGame,
    JoinRoom,
    Chat,
    DrawCard,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum OutgoingEvent {
    None,
    CardDrawn,
    RoomJoined,
    GameCreated,
    Chat,
    DrawCard,
}

impl Default for OutgoingEvent {
    fn default() -> Self {
        Self::None
    }
}
