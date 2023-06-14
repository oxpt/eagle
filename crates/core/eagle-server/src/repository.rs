use eagle_game::game::Game;
use eagle_types::{
    events::SystemCommand,
    ids::{ClientId, PlayerId},
    messages::ClientCommandIndex,
};
use serde::{Deserialize, Serialize};

use crate::effect_outcomes::EffectOutcomes;

pub trait Repository<T: Game>: Sized + 'static {
    fn store_entry(&mut self, entry: RepositoryLogEntry<T>);
    fn is_command_handled(&self, client_id: ClientId, index: ClientCommandIndex) -> bool;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RepositoryLogEntry<T: Game> {
    ConductorCommand {
        index: ClientCommandIndex,
        entry: CommandLogEntry<T::ConductorCommand>,
    },
    PlayerCommand {
        player_id: PlayerId,
        index: ClientCommandIndex,
        entry: CommandLogEntry<T::PlayerCommand>,
    },
    SystemCommand(CommandLogEntry<SystemCommand>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandLogEntry<T> {
    pub command: T,
    pub effect_outcomes: EffectOutcomes,
}
