use eagle_types::{ids::PlayerId, events::SystemCommand};
use serde::{Serialize, Deserialize};

use crate::game::Game;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameCommand<T: Game> {
    ConductorCommand(T::ConductorCommand),
    PlayerCommand(PlayerId, T::PlayerCommand),
    SystemCommand(SystemCommand),
}
