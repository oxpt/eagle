use eagle_types::ids::{GameInstanceId, PlayerId};
use serde::{Deserialize, Serialize};

use crate::Game;

#[derive(Clone, Serialize, Deserialize)]
pub struct BubbledCommand<T: Game> {
    pub(crate) game_instance_id: GameInstanceId,
    pub(crate) inner: Inner<T>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) enum Inner<T: Game> {
    ConductorCommand {
        command: T::ConductorCommand,
    },
    PlayerCommand {
        player_id: PlayerId,
        command: T::PlayerCommand,
    },
}
