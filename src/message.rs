use crate::command::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IncommingMessage {
    command: Command,
}
