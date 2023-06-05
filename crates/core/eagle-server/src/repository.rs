use eagle_game::{Game, GameHandle};
use eagle_types::{events::SystemEvent, ids::PlayerId};

use crate::EffectOutcomes;

pub trait Repository: Sized + 'static {
    fn store_conductor_event<T: Game>(
        &mut self,
        handle: GameHandle<T>,
        event: T::ConductorClientEvent,
        effect_outcomes: EffectOutcomes,
    );
    fn store_player_event<T: Game>(
        &mut self,
        handle: GameHandle<T>,
        player_id: PlayerId,
        event: T::PlayerClientEvent,
        effect_outcomes: EffectOutcomes,
    );
    fn store_system_event<T: Game>(
        &mut self,
        handle: GameHandle<T>,
        event: SystemEvent,
        offect_outcomes: EffectOutcomes,
    );
}
