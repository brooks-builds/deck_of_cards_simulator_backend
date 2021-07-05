use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Action {
    CreateGame,
    JoinRoom,
    Chat,
    DrawCard,
    DrawDeckUpdated,
    PlayerJoinedRoomInSession,
}
