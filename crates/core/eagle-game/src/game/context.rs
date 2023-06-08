use chrono::{DateTime, Utc};
use eagle_types::{client::{ClientState, User}, ids::{PlayerId, GameInstanceId}, events::SystemCommand};
use rand_chacha::ChaCha8Rng;

use crate::{
    clients::ClientsRef,
    eff_handler::EffHandler,
    game::Game,
    prelude::GameHandle, room::{command_history::CommandHistory, notify_history::NotifyHistory, game_instances::GameInstances}, events::GameCommand, bubble::{NotifyBubble, CommandBubble, InnerCommandBubble},
};

pub struct GameContextImpl<'a, 'client, T: Game> {
    game_handle: GameHandle<T>,
    clients: &'a mut ClientsRef<'client>,
    eff: &'a mut EffHandler,
    command_history: &'a mut CommandHistory,
    notify_history: &'a mut NotifyHistory<T>,
    game_instances: &'a mut GameInstances,
    rng: &'a mut ChaCha8Rng,
}

impl<T: Game> GameContextImpl<'_, '_, T> {
    pub(crate) fn new<'a, 'clients>(
        game_handle: GameHandle<T>,
        clients: &'a mut ClientsRef<'clients>,
        eff: &'a mut EffHandler,
        command_history: &'a mut CommandHistory,
        notify_history: &'a mut NotifyHistory<T>,
        game_instances: &'a mut GameInstances,
        rng: &'a mut ChaCha8Rng,
    ) -> GameContextImpl<'a, 'clients, T> {
        GameContextImpl {
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

pub trait GameContext<T: Game> {
    type CommandIterator<'a>: Iterator<Item = &'a GameCommand<T>> where Self: 'a;
    type GameRef<'a, G: Game>: std::ops::Deref<Target = G> + 'a where Self: 'a;

    // clients

    fn get_conductor_clients(&mut self) -> Vec<ClientState>; 
    fn get_player_clients(&mut self, player_id: PlayerId) -> Vec<ClientState>; 

    // history

    fn all_commands<'a>(&'a self) -> Self::CommandIterator<'a>;

    // game output

    fn push_conductor_notify(&mut self, notify: T::ConductorNotify); 
    fn push_player_notify(&mut self, player_id: PlayerId, notify: T::PlayerNotify); 

    // game management

    fn create_game_instance<G: Game>(&mut self, config: G::Config) -> GameHandle<G>; 
    /// Get the game state for a given game instance.
    fn get_game_state<'a, G: Game>(
        &'a self,
        handle: GameHandle<G>,
    ) -> Self::GameRef<'a, G>;

    fn mutate_game<G: Game>(
        &mut self,
        handle: GameHandle<G>,
        mutate: impl FnOnce(&mut GameContextImpl<G>, &mut G),
    ) where
        NotifyBubble<G::Conductor>: Into<T::ConductorNotify>,
        NotifyBubble<G::Player>: Into<T::PlayerNotify>,
;
    /// Trigger a conductor command for a given game instance. The event must be handled immediately
    /// by the implementation. This means that the return values of other methods might be updated.
    fn trigger_conductor_client_event<G: Game>(
        &mut self,
        handle: GameHandle<G>,
        event: G::ConductorCommand,
    ) where
        NotifyBubble<G::Conductor>: Into<T::ConductorNotify>,
        NotifyBubble<G::Player>: Into<T::PlayerNotify>,
;
    /// Trigger a player command for a given game instance. The event must be handled immediately
    /// by the implementation. This means that the return values of other methods might be updated.
    fn trigger_player_client_event<G: Game>(
        &mut self,
        handle: GameHandle<G>,
        player_id: PlayerId,
        event: G::PlayerCommand,
    ) where
        NotifyBubble<G::Conductor>: Into<T::ConductorNotify>,
        NotifyBubble<G::Player>: Into<T::PlayerNotify>,
;
    /// Trigger a system command for a given game instance. The event must be handled immediately
    /// by the implementation. This means that the return values of other methods might be updated.
    fn trigger_system_command<G: Game>(&mut self, handle: GameHandle<G>, event: SystemCommand)
    where
        NotifyBubble<G::Conductor>: Into<T::ConductorNotify>,
        NotifyBubble<G::Player>: Into<T::PlayerNotify>,
;
    fn propagate<G: Game>(&mut self, bubble: CommandBubble<G>)
    where
        NotifyBubble<G::Conductor>: Into<T::ConductorNotify>,
        NotifyBubble<G::Player>: Into<T::PlayerNotify>,
;
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

    // history

    fn all_commands(&self) -> Self::CommandIterator<'_> {
        self.command_history.all_commands(self.game_handle)
    }

    // game output

    fn push_conductor_notify(&mut self, notify: T::ConductorNotify) {
        let index = self.notify_history.log_conductor_notify(notify.clone());
        self.clients.send_notify(User::Conductor, index, notify);
    }
    fn push_player_notify(&mut self, player_id: PlayerId, notify: T::PlayerNotify) {
        let index = self
            .notify_history
            .log_player_notify(player_id, notify.clone());
        self.clients
            .send_notify(User::Player(player_id), index, notify);
    }

    // game management

    fn create_game_instance<G: Game>(&mut self, config: G::Config) -> GameHandle<G> {
        let id = self.eff.new_game.run(|| GameInstanceId::new());
        let handle = GameHandle::new(id);
        let game = G::new(config);
        self.game_instances.insert_game_instance(handle, game);
        handle
    }
    /// Get the game state for a given game instance.
    fn get_game_state<G: Game>(
        &self,
        handle: GameHandle<G>,
    ) -> Self::GameRef<'_, G> {
        self.game_instances.get_game_instance(handle).borrow()
    }

    fn mutate_game<G: Game>(
        &mut self,
        handle: GameHandle<G>,
        mutate: impl FnOnce(&mut GameContextImpl<G>, &mut G),
    ) where
        NotifyBubble<G::Conductor>: Into<T::ConductorNotify>,
        NotifyBubble<G::Player>: Into<T::PlayerNotify>,
    {
        let mut notifies = NotifyHistory::new();
        let mut ctx = GameContextImpl::new(
            handle,
            self.clients,
            self.eff,
            self.command_history,
            &mut notifies,
            self.game_instances,
            self.rng,
        );
        let game = ctx.game_instances.get_game_instance_mut(handle);
        mutate(&mut ctx, &mut game.borrow_mut());
        let NotifyHistory { conductor, players } = notifies;
        for notify in conductor {
            let bubble = NotifyBubble::<G::Conductor> {
                game_instance_id: handle.game_instance_id,
                notify,
            };
            self.push_conductor_notify(bubble.into());
        }
        for (player_id, notifies) in players {
            for notify in notifies {
                let bubble = NotifyBubble::<G::Player> {
                    game_instance_id: handle.game_instance_id,
                    notify,
                };
                self.push_player_notify(player_id, bubble.into());
            }
        }
    }

    /// Trigger a conductor command for a given game instance. The event must be handled immediately
    /// by the implementation. This means that the return values of other methods might be updated.
    fn trigger_conductor_client_event<G: Game>(
        &mut self,
        handle: GameHandle<G>,
        event: G::ConductorCommand,
    ) where
        NotifyBubble<G::Conductor>: Into<T::ConductorNotify>,
        NotifyBubble<G::Player>: Into<T::PlayerNotify>,
    {
        self.mutate_game(handle, |ctx, game| {
            ctx.handle_conductor_command(handle, game, event)
        });
    }

    /// Trigger a player command for a given game instance. The event must be handled immediately
    /// by the implementation. This means that the return values of other methods might be updated.
    fn trigger_player_client_event<G: Game>(
        &mut self,
        handle: GameHandle<G>,
        player_id: PlayerId,
        event: G::PlayerCommand,
    ) where
        NotifyBubble<G::Conductor>: Into<T::ConductorNotify>,
        NotifyBubble<G::Player>: Into<T::PlayerNotify>,
    {
        self.mutate_game(handle, |ctx, game| {
            ctx.handle_player_command(handle, game, player_id, event)
        });
    }

    /// Trigger a system command for a given game instance. The event must be handled immediately
    /// by the implementation. This means that the return values of other methods might be updated.
    fn trigger_system_command<G: Game>(&mut self, handle: GameHandle<G>, event: SystemCommand)
    where
        NotifyBubble<G::Conductor>: Into<T::ConductorNotify>,
        NotifyBubble<G::Player>: Into<T::PlayerNotify>,
    {
        self.mutate_game(handle, |ctx, game| {
            ctx.handle_system_command(handle, game, event)
        });
    }

    fn propagate<G: Game>(&mut self, bubble: CommandBubble<G>)
    where
        NotifyBubble<G::Conductor>: Into<T::ConductorNotify>,
        NotifyBubble<G::Player>: Into<T::PlayerNotify>,
    {
        let handle = GameHandle::<G>::new(bubble.game_instance_id);
        self.mutate_game(handle, |ctx, game| match bubble.inner {
            InnerCommandBubble::ConductorCommand { command } => {
                ctx.handle_conductor_command(handle, game, command)
            }
            InnerCommandBubble::PlayerCommand { player_id, command } => {
                ctx.command_history
                    .log_player_command(handle, player_id, command.clone());
                game.handle_player_command(ctx, player_id, command);
            }
        });
    }

    /// other side effects

    fn now(&mut self) -> DateTime<Utc> {
        self.eff.now.run(|| Utc::now())
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
