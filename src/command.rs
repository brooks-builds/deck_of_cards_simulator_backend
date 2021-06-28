use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
