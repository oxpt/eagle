mod public;
mod private;

use rand_chacha::ChaCha8Rng;

use crate::{
    clients::ClientsRef, command_history::CommandHistory, game_instances::GameInstances, notify_history::NotifyHistory, prelude::GameHandle, eff_handler::EffHandler, game::Game,
};

pub struct Context<'a, 'client, T: Game> {
    game_handle: GameHandle<T>,
    clients: &'a mut ClientsRef<'client>,
    eff: &'a mut EffHandler,
    command_history: &'a mut CommandHistory,
    notify_history: &'a mut NotifyHistory<T>,
    game_instances: &'a mut GameInstances,
    rng: &'a mut ChaCha8Rng,
}

impl<T: Game> Context<'_, '_, T> {
    pub(crate) fn new<'a, 'clients>(
        game_handle: GameHandle<T>,
        clients: &'a mut ClientsRef<'clients>,
        eff: &'a mut EffHandler,
        command_history: &'a mut CommandHistory,
        notify_history: &'a mut NotifyHistory<T>,
        game_instances: &'a mut GameInstances,
        rng: &'a mut ChaCha8Rng,
    ) -> Context<'a, 'clients, T> {
        Context {
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
