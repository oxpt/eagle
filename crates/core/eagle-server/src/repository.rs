use eagle_game::{events::GameCommand, game::Game};
use serde::{Deserialize, Serialize};

use crate::effect_outcomes::EffectOutcomes;

pub trait Repository<T: Game>: Sized + 'static {
    fn store_command(&mut self, entry: RepositoryLogEntry<T>);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RepositoryLogEntry<T: Game> {
    pub command: GameCommand<T>,
    pub effect_outcomes: EffectOutcomes,
}
