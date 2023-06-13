use serde::{Deserialize, Serialize};

use crate::ids::PlayerId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemCommand {
    JoinPlayer(PlayerId),
}
