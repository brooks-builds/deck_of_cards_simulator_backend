use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Command {
    None,
    CreateGame,
    JoinRoom,
    Chat,
    DrawCard,
}

impl Default for Command {
    fn default() -> Self {
        Self::None
    }
}
