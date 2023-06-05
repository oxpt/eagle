use eagle_game::{EffHandler, Game, GameHandle, Room};
use eagle_types::{
    client::{ClientParams, User},
    errors::EagleError,
    events::{ClientEventIndex, IsNextOf, SystemEvent},
    ids::{ClientId, PlayerId},
};

use crate::{channel::Channel, clients::Clients, repository::Repository, EffectOutcomes};

pub struct GameServer<C: Channel> {
    clients: Clients<C>,
    room: Room,
}

impl<C: Channel> GameServer<C> {
    pub fn new(room: Room) -> Self {
        Self {
            clients: Clients::new(),
            room,
        }
    }

    pub fn add_conductor_client<T: Game>(
        &mut self,
        handle: GameHandle<T>,
        params: ClientParams,
        channel: C,
    ) {
        self.clients.add_client(User::Conductor, channel);
        if let Some(mut latest_index) = params.latest_received_server_event {
            let events = self.room.get_conductor_server_events(handle);
            for event in events.skip(latest_index.skip()) {
                latest_index = latest_index.next();
                self.clients.send_server_event(
                    User::Conductor,
                    latest_index,
                    handle.game_instance_id,
                    event,
                );
            }
        }
    }

    pub fn add_player_client<T: Game>(
        &mut self,
        handle: GameHandle<T>,
        player_id: PlayerId,
        params: ClientParams,
        channel: C,
    ) {
        self.clients.add_client(User::Player(player_id), channel);
        if let Some(mut latest_index) = params.latest_received_server_event {
            let events = self.room.get_player_server_events(handle, player_id);
            for event in events.skip(latest_index.skip()) {
                latest_index = latest_index.next();
                self.clients.send_server_event(
                    User::Player(player_id),
                    latest_index,
                    handle.game_instance_id,
                    event,
                );
            }
        }
    }

    pub fn remove_client(&mut self, user: User, client_id: ClientId) {
        self.clients.remove_channel(user, client_id);
    }

    pub fn handle_conductor_event<T: Game>(
        &mut self,
        repository: &mut impl Repository,
        client_id: ClientId,
        handle: GameHandle<T>,
        index: ClientEventIndex,
        event: T::ConductorClientEvent,
    ) {
        let server_side_index = self.room.current_conductor_client_event_index(handle);
        match index.is_next_of(server_side_index) {
            IsNextOf::Yes => (),
            IsNextOf::No => return,
            IsNextOf::TooFarAhead => {
                self.room.log_error(
                    handle,
                    EagleError::ClientSendsClientEventWithTooAheadIndex {
                        client_side_index: index,
                        server_side_index,
                        user: User::Conductor,
                    }
                    .into(),
                );
                return;
            }
        }
        let mut eff = EffHandler::default();
        self.room.handle_conductor_event(
            &mut self.clients.to_ref(),
            &mut eff,
            handle,
            event.clone(),
        );
        let effect_outcomes = EffectOutcomes::from(eff);
        self.clients
            .update_last_successful_communication(User::Conductor, client_id);
        repository.store_conductor_event(handle, event, effect_outcomes)
    }

    pub fn handle_player_event<T: Game>(
        &mut self,
        repository: &mut impl Repository,
        client_id: ClientId,
        handle: GameHandle<T>,
        player_id: PlayerId,
        index: ClientEventIndex,
        event: T::PlayerClientEvent,
    ) {
        let server_side_index = self
            .room
            .current_player_client_event_index(handle, player_id);
        match index.is_next_of(server_side_index) {
            IsNextOf::Yes => (),
            IsNextOf::No => return,
            IsNextOf::TooFarAhead => {
                self.room.log_error(
                    handle,
                    EagleError::ClientSendsClientEventWithTooAheadIndex {
                        client_side_index: index,
                        server_side_index,
                        user: User::Player(player_id),
                    }
                    .into(),
                );
            }
        }
        let mut eff = EffHandler::default();
        self.room.handle_player_event(
            &mut self.clients.to_ref(),
            &mut eff,
            handle,
            player_id,
            event.clone(),
        );
        let effect_outcomes = EffectOutcomes::from(eff);
        self.clients
            .update_last_successful_communication(User::Player(player_id), client_id);
        repository.store_player_event(handle, player_id, event, effect_outcomes)
    }

    pub fn handle_system_event<T: Game>(
        &mut self,
        repository: &mut impl Repository,
        handle: GameHandle<T>,
        event: SystemEvent,
    ) {
        let mut eff = EffHandler::default();
        self.room
            .handle_system_event(&mut self.clients.to_ref(), &mut eff, handle, event.clone());
        let effect_outcomes = EffectOutcomes::from(eff);
        repository.store_system_event(handle, event, effect_outcomes)
    }

    pub fn replay_conductor_event<T: Game>(
        &mut self,
        handle: GameHandle<T>,
        event: T::ConductorClientEvent,
        effect_outcomes: EffectOutcomes,
    ) {
        let mut eff = effect_outcomes.into();
        self.room
            .handle_conductor_event(&mut self.clients.to_ref(), &mut eff, handle, event);
    }

    pub fn replay_player_event<T: Game>(
        &mut self,
        handle: GameHandle<T>,
        player_id: PlayerId,
        event: T::PlayerClientEvent,
        effect_outcomes: EffectOutcomes,
    ) {
        let mut eff = effect_outcomes.into();
        self.room.handle_player_event(
            &mut self.clients.to_ref(),
            &mut eff,
            handle,
            player_id,
            event,
        );
    }

    pub fn replay_system_event<T: Game>(
        &mut self,
        handle: GameHandle<T>,
        event: SystemEvent,
        effect_outcomes: EffectOutcomes,
    ) {
        let mut eff = effect_outcomes.into();
        self.room
            .handle_system_event(&mut self.clients.to_ref(), &mut eff, handle, event);
    }
}
