mod private;
mod public;

use rand_chacha::ChaCha8Rng;

use crate::{
    clients::Clients,event_history::EventHistory,
    game_instances::GameInstances, Game, GameHandle, EffHandler,
};

pub struct Context<'a, 'client, 'eff, T: Game> {
    game_handle: GameHandle<T>,
    clients: &'a Clients<'client>,
    eff: &'a mut EffHandler<'eff>,
    event_history: &'a mut EventHistory,
    game_instances: &'a mut GameInstances,
    rng: &'a mut ChaCha8Rng,
}
