use eagle_game::prelude::PlayerId;
use serde::{Deserialize, Serialize};

use crate::{
    events::{UltimatumConductorCommand, UltimatumPlayerCommand},
    phase::Phase,
};

#[derive(Debug, thiserror::Error, Serialize, Deserialize, Clone)]
pub enum UltimatumError {
    #[error("unexpected conductor command: {command:?} in phase: {phase:?}")]
    UnexpectedConductorCommand {
        phase: Phase,
        command: UltimatumConductorCommand,
    },
    #[error("unexpected player command: {command:?} in phase: {phase:?}")]
    UnexpectedPlayerCommand {
        phase: Phase,
        player_id: PlayerId,
        command: UltimatumPlayerCommand,
    },
}
