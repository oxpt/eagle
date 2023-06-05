mod private;
mod public;

use rand_chacha::ChaCha8Rng;

use crate::{
    clients::Clients, event_history::EventHistory, game_instances::GameInstances, EffHandler, Game,
    GameHandle,
};

pub struct Context<'a, 'client, T: Game> {
    game_handle: GameHandle<T>,
    clients: &'a mut Clients<'client>,
    eff: &'a mut EffHandler,
    event_history: &'a mut EventHistory,
    game_instances: &'a mut GameInstances,
    rng: &'a mut ChaCha8Rng,
}
