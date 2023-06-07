use eagle_types::ids::{GameInstanceId, PlayerId};
use serde::{Deserialize, Serialize};

use crate::{game::Game, model::Model};

#[derive(Clone, Serialize, Deserialize)]
/// Bubbled up command from a sub game to super game.
pub struct CommandBubble<T: Game> {
    pub(crate) game_instance_id: GameInstanceId,
    pub(crate) inner: InnerCommandBubble<T>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) enum InnerCommandBubble<T: Game> {
    ConductorCommand {
        command: T::ConductorCommand,
    },
    PlayerCommand {
        player_id: PlayerId,
        command: T::PlayerCommand,
    },
}

#[derive(Clone, Serialize, Deserialize)]
/// Bubbled up notify from a sub game to super game.
pub struct NotifyBubble<T: Model> {
    pub(crate) game_instance_id: GameInstanceId,
    pub(crate) notify: T::Notify,
}
