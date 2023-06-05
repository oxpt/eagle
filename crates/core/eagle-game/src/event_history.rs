use std::{any::Any, collections::BTreeMap};

use crate::{game::Game, game_handle::GameHandle};
use eagle_types::{
    events::{ClientEventIndex, ServerEventIndex, SystemEvent},
    ids::{GameInstanceId, PlayerId}, client::User,
};

#[derive(Default)]
/// This stores server and client events in RON format.
pub(crate) struct EventHistory {
    // This Any is a GameEventHistory<T> where T is the game type.
    games: BTreeMap<GameInstanceId, Box<dyn Any>>,
    conductor_server_events: BTreeMap<User, Box<dyn Any>>,
}

struct EventLog<T: Game> {
    pub conductor_server_events: Vec<T::ConductorServerEvent>,
    pub player_server_events: BTreeMap<PlayerId, Vec<T::PlayerServerEvent>>,
    pub system_events: Vec<SystemEvent>,
}

impl<T: Game> EventLog<T> {
    fn new() -> Self {
        Self {
            conductor_server_events: Default::default(),
            player_server_events: Default::default(),
            system_events: Default::default(),
        }
    }
}

impl EventHistory {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn log_conductor_server_event<T: Game>(
        &mut self,
        game_handle: GameHandle<T>,
        event: T::ConductorServerEvent,
    ) -> ServerEventIndex {
        let vec = &mut self
            .games
            .entry(game_handle.game_instance_id)
            .or_insert_with(|| Box::new(EventLog::<T>::new()))
            .downcast_mut::<EventLog<T>>()
            .unwrap()
            .conductor_server_events;
        vec.push(event);
        ServerEventIndex::from_len(vec.len())
    }

    pub fn log_conductor_client_event<T: Game>(
        &mut self,
        game_handle: GameHandle<T>,
        event: T::ConductorClientEvent,
    ) {
        let vec = &mut self
            .games
            .entry(game_handle.game_instance_id)
            .or_insert_with(|| Box::new(EventLog::<T>::new()))
            .downcast_mut::<EventLog<T>>()
            .unwrap()
            .conductor_client_events;
        vec.push(event);
    }

    pub fn log_player_server_event<T: Game>(
        &mut self,
        game_handle: GameHandle<T>,
        player_id: PlayerId,
        event: T::PlayerServerEvent,
    ) -> ServerEventIndex {
        let vec = &mut self
            .games
            .entry(game_handle.game_instance_id)
            .or_insert_with(|| Box::new(EventLog::<T>::new()))
            .downcast_mut::<EventLog<T>>()
            .unwrap()
            .player_server_events
            .entry(player_id)
            .or_insert_with(Vec::new);
        vec.push(event);
        ServerEventIndex::from_len(vec.len())
    }

    pub fn log_player_client_event<T: Game>(
        &mut self,
        game_handle: GameHandle<T>,
        player_id: PlayerId,
        event: T::PlayerClientEvent,
    ) {
        let vec = &mut self
            .games
            .entry(game_handle.game_instance_id)
            .or_insert_with(|| Box::new(EventLog::<T>::new()))
            .downcast_mut::<EventLog<T>>()
            .unwrap()
            .player_client_events
            .entry(player_id)
            .or_insert_with(Vec::new);
        vec.push(event);
    }

    pub fn log_system_event<T: Game>(
        &mut self,
        game_handle: GameHandle<T>,
        event: SystemEvent,
    ) -> ServerEventIndex {
        let vec = &mut self
            .games
            .entry(game_handle.game_instance_id)
            .or_insert_with(|| Box::new(EventLog::<T>::new()))
            .downcast_mut::<EventLog<T>>()
            .unwrap()
            .system_events;
        vec.push(event);
        ServerEventIndex::from_len(vec.len())
    }

    pub fn get_conductor_server_events<T: Game>(
        &self,
        game_handle: GameHandle<T>,
    ) -> impl Iterator<Item = &T::ConductorServerEvent> {
        self.games
            .get(&game_handle.game_instance_id)
            .map(|any| {
                any.downcast_ref::<EventLog<T>>()
                    .unwrap()
                    .conductor_server_events
                    .iter()
            })
            .unwrap_or_else(|| [].iter())
    }

    pub fn get_conductor_client_events<T: Game>(
        &self,
        game_handle: GameHandle<T>,
    ) -> impl Iterator<Item = &T::ConductorClientEvent> {
        self.games
            .get(&game_handle.game_instance_id)
            .map(|any| {
                any.downcast_ref::<EventLog<T>>()
                    .unwrap()
                    .conductor_client_events
                    .iter()
            })
            .unwrap_or_else(|| [].iter())
    }

    pub fn get_player_server_events<'a, T: Game>(
        &'a self,
        game_handle: GameHandle<T>,
        player_id: PlayerId,
    ) -> impl Iterator<Item = &'a T::PlayerServerEvent> {
        self.games
            .get(&game_handle.game_instance_id)
            .and_then(|any| {
                any.downcast_ref::<EventLog<T>>()
                    .unwrap()
                    .player_server_events
                    .get(&player_id)
                    .map(|v| v.iter())
            })
            .unwrap_or_else(|| [].iter())
    }

    pub fn get_player_client_events<'a, T: Game>(
        &'a self,
        game_handle: GameHandle<T>,
        player_id: PlayerId,
    ) -> impl Iterator<Item = &'a T::PlayerClientEvent> {
        self.games
            .get(&game_handle.game_instance_id)
            .and_then(|any| {
                any.downcast_ref::<EventLog<T>>()
                    .unwrap()
                    .player_client_events
                    .get(&player_id)
                    .map(|v| v.iter())
            })
            .unwrap_or_else(|| [].iter())
    }

    pub fn get_system_events<T: Game>(
        &self,
        game_handle: GameHandle<T>,
    ) -> impl Iterator<Item = &SystemEvent> {
        self.games
            .get(&game_handle.game_instance_id)
            .map(|any| {
                any.downcast_ref::<EventLog<T>>()
                    .unwrap()
                    .system_events
                    .iter()
            })
            .unwrap_or_else(|| [].iter())
    }

    pub fn current_conductor_client_event_index<T: Game>(
        &self,
        game_handle: GameHandle<T>,
    ) -> ClientEventIndex {
        ClientEventIndex::from_len_after_insert(
            self.games
                .get(&game_handle.game_instance_id)
                .map(|any| {
                    any.downcast_ref::<EventLog<T>>()
                        .unwrap()
                        .conductor_client_events
                        .len()
                })
                .unwrap_or(0),
        )
    }

    pub fn current_player_client_event_index<T: Game>(
        &self,
        game_handle: GameHandle<T>,
        player_id: PlayerId,
    ) -> ClientEventIndex {
        ClientEventIndex::from_len_after_insert(
            self.games
                .get(&game_handle.game_instance_id)
                .and_then(|any| {
                    any.downcast_ref::<EventLog<T>>()
                        .unwrap()
                        .player_client_events
                        .get(&player_id)
                        .map(|v| v.len())
                })
                .unwrap_or(0),
        )
    }
}
