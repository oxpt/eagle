use chrono::{DateTime, Utc};
use eagle_types::{
    client::{ClientState, User},
    ids::{GameInstanceId, PlayerId},
};

use crate::{context::Context, game::Game, game_handle::GameHandle};

impl<T: Game> Context<'_, '_, '_, T> {
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

    pub fn all_conductor_client_events(&self) -> impl Iterator<Item = &T::ConductorClientEvent> {
        self.event_history
            .get_conductor_client_events(self.game_handle)
    }
    pub fn all_player_client_events(
        &self,
        player_id: PlayerId,
    ) -> impl Iterator<Item = &T::PlayerClientEvent> {
        self.event_history
            .get_player_client_events(self.game_handle, player_id)
    }
    pub fn all_conductor_server_events(&self) -> impl Iterator<Item = &T::ConductorServerEvent> {
        self.event_history
            .get_conductor_server_events(self.game_handle)
    }
    pub fn all_player_server_events(
        &self,
        player_id: PlayerId,
    ) -> impl Iterator<Item = &T::PlayerServerEvent> {
        self.event_history
            .get_player_server_events(self.game_handle, player_id)
    }

    // game output

    pub fn push_conductor_server_event(&mut self, event: T::ConductorServerEvent) {
        let index = self
            .event_history
            .log_conductor_server_event(self.game_handle, event.clone());
        self.clients.send_server_event(
            User::Conductor,
            self.game_handle.game_instance_id,
            index,
            event,
        )
    }
    pub fn push_player_server_event(&mut self, player_id: PlayerId, event: T::PlayerServerEvent) {
        let index =
            self.event_history
                .log_player_server_event(self.game_handle, player_id, event.clone());
        self.clients.send_server_event(
            User::Player(player_id),
            self.game_handle.game_instance_id,
            index,
            event,
        )
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
        self.game_instances.get_game_instance(handle)
    }
    /// Trigger a conductor client event for a given game instance. The event must be handled immediately
    /// by the implementation. This means that the return values of other methods might be updated.
    pub fn trigger_conductor_client_event<G: Game>(
        &mut self,
        handle: GameHandle<G>,
        event: G::ConductorClientEvent,
    ) {
        self.handle_conductor_event(handle, event);
    }

    /// other side effects

    pub fn now(&mut self) -> DateTime<Utc> {
        self.eff.now.run(|| Utc::now())
    }
}
