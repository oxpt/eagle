use eagle_game::{Game, GameCommand};

use crate::EffectOutcomes;

pub trait Repository<T: Game>: Sized + 'static {
    fn store_command(
        &mut self,
        command: GameCommand<T>,
        effect_outcomes: EffectOutcomes,
    );
}
