use eagle_types::{
    client::{ClientState, User},
    ids::{ClientId, PlayerId},
};

use crate::{
    channels::ChannelsRef,
    game::{Game, GameHandle},
    room_state::RoomState,
    serialized_event::SerializedEvent,
};

pub struct RoomContext<'a> {
    state: RoomState,
    channels: ChannelsRef<'a>,
}

impl RoomContext<'_> {
    // clients

    pub fn get_conductor_clients<'a>(&'a self) -> impl Iterator<Item = ClientState> + 'a {
        self.state
            .client_attachments
            .iter()
            .filter(|(user, _)| **user == User::Conductor)
            .flat_map(|(_, clients)| clients.iter().copied())
            .map(|client_id| self.channels.get_client_state(client_id))
            .flatten()
    }
    pub fn get_player_clients<'a>(&'a self, player_id: PlayerId) -> impl Iterator<Item = ClientState> + 'a {
        self.state
            .client_attachments
            .iter()
            .filter(move |(user, _)| **user == User::Player(player_id))
            .flat_map(|(_, clients)| clients.iter().copied())
            .map(|client_id| self.channels.get_client_state(client_id))
            .flatten()
    }
    pub fn attach_client_to(&mut self, user: User, client_id: ClientId) {
        self.state
            .client_attachments
            .entry(user)
            .or_default()
            .insert(client_id);
    }
    pub fn detach_client(&mut self, user: User, client_id: ClientId) {
        if let Some(clients) = self.state.client_attachments.get_mut(&user) {
            clients.remove(&client_id);
        }
    }
    pub fn get_attached_player_id(&self, client_id: ClientId) -> Option<User> {
        self.state
            .client_attachments
            .iter()
            .find(|(_, clients)| clients.contains(&client_id))
            .map(|(user, _)| *user)
    }
    pub fn get_all_client_attachments<'a>(
        &'a self,
    ) -> impl Iterator<Item = (User, impl Iterator<Item = ClientId> + 'a)> {
        self.state
            .client_attachments
            .iter()
            .map(|(user, clients)| (*user, clients.iter().copied()))
    }

    // history

    pub fn handled_conductor_client_events<T: Game>(
        &self,
        handle: &GameHandle<T>,
    ) -> impl Iterator<Item = SerializedEvent<T::ConductorClientEvent>> {
        self.state.event_history.conductor_client_events(handle)
    }
    pub fn handled_player_client_events<T: Game>(
        &self,
        handle: &GameHandle<T>,
        player_id: PlayerId,
    ) -> impl Iterator<Item = SerializedEvent<T::PlayerClientEvent>> {
        self.state
            .event_history
            .player_client_events(handle, player_id)
    }
    pub fn sent_conductor_server_events<T: Game>(
        &self,
        handle: &GameHandle<T>,
    ) -> impl Iterator<Item = SerializedEvent<T::ConductorServerEvent>> {
        self.state.event_history.conductor_server_events(handle)
    }
    pub fn sent_player_server_events<T: Game>(
        &self,
        handle: &GameHandle<T>,
        player_id: PlayerId,
    ) -> impl Iterator<Item = SerializedEvent<T::PlayerServerEvent>> {
        self.state
            .event_history
            .player_server_events(handle, player_id)
    }

    // game output

    pub fn push_server_event_to_conductor<T: Game>(
        &mut self,
        handle: &GameHandle<T>,
        event: T::ConductorServerEvent,
    ) {
        todo!()
    }
    pub fn push_server_event_to_player<T: Game>(
        &mut self,
        handle: &GameHandle<T>,
        player_id: PlayerId,
        event: T::PlayerServerEvent,
    ) {
        todo!()
    }

    // game management
    pub fn create_game_instance<T: Game>(&mut self, game: T) -> GameHandle<T> {
        todo!()
    }
    /// Get the game state for a given game instance.
    pub fn get_game_state<T: Game>(&self, handle: GameHandle<T>) -> &T {
        todo!()
    }
    /// Trigger a conductor client event for a given game instance. The event must be handled immediately
    /// by the implementation. This means that the return values of other methods might be updated.
    pub fn trigger_conductor_client_event<T: Game>(
        &mut self,
        handle: GameHandle<T>,
        event: T::ConductorClientEvent,
    ) {
        todo!()
    }
}
