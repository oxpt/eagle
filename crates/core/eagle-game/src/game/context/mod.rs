mod private;
mod public;

use rand_chacha::ChaCha8Rng;

use crate::{
    clients::ClientsRef,
    eff_handler::EffHandler,
    game::Game,
    prelude::GameHandle, room::{command_history::CommandHistory, notify_history::NotifyHistory, game_instances::GameInstances},
};

pub struct GameContext<'a, 'client, T: Game> {
    game_handle: GameHandle<T>,
    clients: &'a mut ClientsRef<'client>,
    eff: &'a mut EffHandler,
    command_history: &'a mut CommandHistory,
    notify_history: &'a mut NotifyHistory<T>,
    game_instances: &'a mut GameInstances,
    rng: &'a mut ChaCha8Rng,
}

impl<T: Game> GameContext<'_, '_, T> {
    pub(crate) fn new<'a, 'clients>(
        game_handle: GameHandle<T>,
        clients: &'a mut ClientsRef<'clients>,
        eff: &'a mut EffHandler,
        command_history: &'a mut CommandHistory,
        notify_history: &'a mut NotifyHistory<T>,
        game_instances: &'a mut GameInstances,
        rng: &'a mut ChaCha8Rng,
    ) -> GameContext<'a, 'clients, T> {
        GameContext {
            game_handle,
            clients,
            eff,
            command_history,
            notify_history,
            game_instances,
            rng,
        }
    }
}
