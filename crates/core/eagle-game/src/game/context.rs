use chrono::{DateTime, Utc};
use eagle_types::{
    client::{ClientState, User},
    events::SystemCommand,
    ids::{GameInstanceId, PlayerId},
};
use rand_chacha::ChaCha8Rng;

use crate::{
    clients::ClientsRef,
    eff_handler::EffHandler,
    events::GameCommand,
    game::Game,
    prelude::GameHandle,
    room::{command_history::CommandHistory, game_instances::GameInstances},
};

pub(crate) struct GameContextImpl<'a, 'client, T: Game> {
    game_handle: GameHandle<T>,
    clients: &'a ClientsRef<'client>,
    eff: &'a mut EffHandler,
    command_history: &'a mut CommandHistory,
    game_instances: &'a mut GameInstances,
    rng: &'a mut ChaCha8Rng,
}

impl<T: Game> GameContextImpl<'_, '_, T> {
    pub(crate) fn new<'a, 'clients>(
        game_handle: GameHandle<T>,
        clients: &'a ClientsRef<'clients>,
        eff: &'a mut EffHandler,
        command_history: &'a mut CommandHistory,
        game_instances: &'a mut GameInstances,
        rng: &'a mut ChaCha8Rng,
    ) -> GameContextImpl<'a, 'clients, T> {
        GameContextImpl {
            game_handle,
            clients,
            eff,
            command_history,
            game_instances,
            rng,
        }
    }
}

pub trait GameContext<T: Game> {
    type CommandIterator<'a>: Iterator<Item = &'a GameCommand<T>>
    where
        Self: 'a;
    type GameRef<'a, G: Game>: std::ops::Deref<Target = G> + 'a
    where
        Self: 'a;

    // clients

    fn get_conductor_clients(&mut self) -> Vec<ClientState>;
    fn get_player_clients(&mut self, player_id: PlayerId) -> Vec<ClientState>;

    // history

    fn all_commands(&self) -> Self::CommandIterator<'_>;

    // game management

    fn create_game_instance<G: Game>(&mut self, config: G::Config) -> GameHandle<G>;

    /// Get the game state for a given game instance.
    fn get_game_state<'a, G: Game>(&'a self, handle: GameHandle<G>) -> Self::GameRef<'a, G>;

    /// Trigger a conductor command for a given game instance. The event must be handled immediately
    /// by the implementation. This means that the return values of other methods might be updated.
    fn trigger_conductor_command<G: Game>(
        &mut self,
        handle: GameHandle<G>,
        event: G::ConductorCommand,
    );
    /// Trigger a player command for a given game instance. The event must be handled immediately
    /// by the implementation. This means that the return values of other methods might be updated.
    fn trigger_player_command<G: Game>(
        &mut self,
        handle: GameHandle<G>,
        player_id: PlayerId,
        event: G::PlayerCommand,
    );
    /// Trigger a system command for a given game instance. The event must be handled immediately
    /// by the implementation. This means that the return values of other methods might be updated.
    fn trigger_system_command<G: Game>(&mut self, handle: GameHandle<G>, event: SystemCommand);

    /// other side effects

    fn now(&mut self) -> DateTime<Utc>;
}

impl<T: Game> GameContext<T> for GameContextImpl<'_, '_, T> {
    type CommandIterator<'a> = std::slice::Iter<'a, GameCommand<T>> where Self: 'a;
    type GameRef<'a, G: Game> = std::cell::Ref<'a, G> where Self: 'a;

    // clients

    fn get_conductor_clients(&mut self) -> Vec<ClientState> {
        self.eff
            .client_states
            .run(|| self.clients.get_client_states(User::Conductor))
    }
    fn get_player_clients(&mut self, player_id: PlayerId) -> Vec<ClientState> {
        self.eff
            .client_states
            .run(|| self.clients.get_client_states(User::Player(player_id)))
    }

    fn all_commands(&self) -> Self::CommandIterator<'_> {
        self.command_history.all_commands(self.game_handle)
    }

    // game management

    fn create_game_instance<G: Game>(&mut self, config: G::Config) -> GameHandle<G> {
        let id = self.eff.new_game.run(GameInstanceId::gen);
        let handle = GameHandle::new(id);
        let game = G::new(config);
        self.game_instances.insert_game_instance(handle, game);
        handle
    }

    fn get_game_state<G: Game>(&self, handle: GameHandle<G>) -> Self::GameRef<'_, G> {
        self.game_instances.get_game_instance_ref(handle).borrow()
    }

    fn trigger_conductor_command<G: Game>(
        &mut self,
        handle: GameHandle<G>,
        event: G::ConductorCommand,
    ) {
        self.mutate_game(handle, |ctx, game| {
            ctx.handle_conductor_command(handle, game, event)
        });
    }

    /// Trigger a player command for a given game instance. The event must be handled immediately
    /// by the implementation. This means that the return values of other methods might be updated.
    fn trigger_player_command<G: Game>(
        &mut self,
        handle: GameHandle<G>,
        player_id: PlayerId,
        event: G::PlayerCommand,
    ) {
        self.mutate_game(handle, |ctx, game| {
            ctx.handle_player_command(handle, game, player_id, event)
        });
    }

    /// Trigger a system command for a given game instance. The event must be handled immediately
    /// by the implementation. This means that the return values of other methods might be updated.
    fn trigger_system_command<G: Game>(&mut self, handle: GameHandle<G>, event: SystemCommand) {
        self.mutate_game(handle, |ctx, game| {
            ctx.handle_system_command(handle, game, event)
        });
    }

    /// other side effects

    fn now(&mut self) -> DateTime<Utc> {
        self.eff.now.run(Utc::now)
    }
}

impl<'a, 'client, T: Game> GameContextImpl<'a, 'client, T> {
    pub(crate) fn handle_conductor_command(
        &mut self,
        handle: GameHandle<T>,
        game: &mut T,
        command: T::ConductorCommand,
    ) {
        self.command_history
            .log_conductor_command(handle, command.clone());
        game.handle_conductor_command(self, command);
    }

    fn mutate_game<G: Game>(
        &mut self,
        handle: GameHandle<G>,
        mutate: impl FnOnce(&mut GameContextImpl<G>, &mut G),
    ) {
        let mut ctx = GameContextImpl::new(
            handle,
            self.clients,
            self.eff,
            self.command_history,
            self.game_instances,
            self.rng,
        );
        let game = ctx.game_instances.get_game_instance(handle);
        mutate(&mut ctx, &mut game.borrow_mut());
    }

    pub(crate) fn handle_player_command(
        &mut self,
        handle: GameHandle<T>,
        game: &mut T,
        player_id: PlayerId,
        command: T::PlayerCommand,
    ) {
        self.command_history
            .log_player_command(handle, player_id, command.clone());
        game.handle_player_command(self, player_id, command);
    }

    pub(crate) fn handle_system_command(
        &mut self,
        handle: GameHandle<T>,
        game: &mut T,
        command: SystemCommand,
    ) {
        self.command_history
            .log_system_command(handle, command.clone());
        game.handle_system_command(self, command);
    }
}
