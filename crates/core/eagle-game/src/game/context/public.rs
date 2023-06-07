use chrono::{DateTime, Utc};
use eagle_types::{
    client::{ClientState, User},
    events::SystemCommand,
    ids::{GameInstanceId, PlayerId},
};

use crate::{
    bubble::{CommandBubble, InnerCommandBubble, NotifyBubble},
    events::GameCommand,
    game::Game,
    game::handle::GameHandle,
    room::notify_history::NotifyHistory,
};
use super::GameContext;

impl<T: Game> GameContext<'_, '_, T> {
    // clients

    pub fn get_conductor_clients(&mut self) -> Vec<ClientState> {
        self.eff
            .client_states
            .run(|| self.clients.get_client_states(User::Conductor))
    }
    pub fn get_player_clients(&mut self, player_id: PlayerId) -> Vec<ClientState> {
        self.eff
            .client_states
            .run(|| self.clients.get_client_states(User::Player(player_id)))
    }

    // history

    pub fn all_commands(&self) -> impl Iterator<Item = &GameCommand<T>> {
        self.command_history.all_commands(self.game_handle)
    }

    // game output

    pub fn push_conductor_notify(&mut self, notify: T::ConductorNotify) {
        let index = self.notify_history.log_conductor_notify(notify.clone());
        self.clients.send_notify(User::Conductor, index, notify);
    }
    pub fn push_player_notify(&mut self, player_id: PlayerId, notify: T::PlayerNotify) {
        let index = self
            .notify_history
            .log_player_notify(player_id, notify.clone());
        self.clients
            .send_notify(User::Player(player_id), index, notify);
    }

    // game management

    pub fn create_game_instance<G: Game>(&mut self, config: G::Config) -> GameHandle<G> {
        let id = self.eff.new_game.run(|| GameInstanceId::new());
        let handle = GameHandle::new(id);
        let game = G::new(config);
        self.game_instances.insert_game_instance(handle, game);
        handle
    }
    /// Get the game state for a given game instance.
    pub fn get_game_state<G: Game>(
        &self,
        handle: GameHandle<G>,
    ) -> impl std::ops::Deref<Target = G> + '_ {
        self.game_instances.get_game_instance(handle).borrow()
    }

    fn mutate_game<G: Game>(
        &mut self,
        handle: GameHandle<G>,
        mutate: impl FnOnce(&mut GameContext<G>, &mut G),
    ) where
        NotifyBubble<G::Conductor>: Into<T::ConductorNotify>,
        NotifyBubble<G::Player>: Into<T::PlayerNotify>,
    {
        let mut notifies = NotifyHistory::new();
        let mut ctx = GameContext::new(
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
    pub fn trigger_conductor_client_event<G: Game>(
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
    pub fn trigger_player_client_event<G: Game>(
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
    pub fn trigger_system_command<G: Game>(&mut self, handle: GameHandle<G>, event: SystemCommand)
    where
        NotifyBubble<G::Conductor>: Into<T::ConductorNotify>,
        NotifyBubble<G::Player>: Into<T::PlayerNotify>,
    {
        self.mutate_game(handle, |ctx, game| {
            ctx.handle_system_command(handle, game, event)
        });
    }

    pub fn propagate<G: Game>(&mut self, bubble: CommandBubble<G>)
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

    pub fn now(&mut self) -> DateTime<Utc> {
        self.eff.now.run(|| Utc::now())
    }
}
