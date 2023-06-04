use eagle_types::ids::PlayerId;
use rand_chacha::ChaCha8Rng;

use crate::{
    clients::Clients, event_history::EventHistory, game_instances::GameInstances, Context, Game,
    GameHandle,
};

use super::EffHandler;

impl<T: Game> Context<'_, '_, '_, T> {
    pub(crate) fn new<'a, 'clients, 'eff>(
        game_handle: GameHandle<T>,
        clients: &'a Clients<'clients>,
        eff: &'a mut EffHandler<'eff>,
        event_history: &'a mut EventHistory,
        game_instances: &'a mut GameInstances,
        rng: &'a mut ChaCha8Rng,
    ) -> Context<'a, 'clients, 'eff, T> {
        Context {
            game_handle,
            clients,
            eff,
            event_history,
            game_instances,
            rng,
        }
    }

    pub(crate) fn handle_conductor_event<S: Game>(
        &mut self,
        handle: GameHandle<S>,
        event: S::ConductorClientEvent,
    ) {
        self.event_history
            .log_conductor_client_event(handle, event.clone());
        let mut ctx = Context::new(
            handle,
            self.clients,
            self.eff,
            self.event_history,
            self.game_instances,
            self.rng,
        );
        let game = ctx.game_instances.get_game_instance_mut(handle);
        // This does not panic because cargo asserts that the game cannot be accessed elsewhere by self-reference or
        // cyclic-reference among games.
        game.borrow_mut().handle_conductor_event(&mut ctx, event);
    }

    pub(crate) fn handle_player_event<S: Game>(
        &mut self,
        handle: GameHandle<S>,
        player_id: PlayerId,
        event: S::PlayerClientEvent,
    ) {
        self.event_history
            .log_player_client_event(handle, player_id, event.clone());
        let mut ctx = Context::new(
            handle,
            self.clients,
            self.eff,
            self.event_history,
            self.game_instances,
            self.rng,
        );
        let game = ctx.game_instances.get_game_instance_mut(handle);
        // This does not panic because cargo asserts that the game cannot be accessed elsewhere by self-reference or
        // cyclic-reference among games.
        game.borrow_mut()
            .handle_player_event(&mut ctx, player_id, event);
    }
}
