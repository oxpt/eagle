use std::{any::Any, collections::BTreeMap};

use crate::{game::Game, game_handle::GameHandle};
use eagle_types::ids::{GameInstanceId, PlayerId};

#[derive(Default)]
/// This stores server and client events in RON format.
pub(crate) struct EventHistory {
    // This Any is a GameEventHistory<T> where T is the game type.
    games: BTreeMap<GameInstanceId, Box<dyn Any>>,
}

struct EventLog<T: Game> {
    pub conductor_server_events: Vec<T::ConductorServerEvent>,
    pub conductor_client_events: Vec<T::ConductorClientEvent>,
    pub player_server_events: BTreeMap<PlayerId, Vec<T::PlayerServerEvent>>,
    pub player_client_events: BTreeMap<PlayerId, Vec<T::PlayerClientEvent>>,
}

impl<T: Game> EventLog<T> {
    fn new() -> Self {
        Self {
            conductor_server_events: Vec::new(),
            conductor_client_events: Vec::new(),
            player_server_events: BTreeMap::new(),
            player_client_events: BTreeMap::new(),
        }
    }
}

impl EventHistory {
    pub fn new() -> Self {
        Self {
            games: Default::default()
        }
    }

    pub fn log_conductor_server_event<T: Game>(
        &mut self,
        game_handle: GameHandle<T>,
        event: T::ConductorServerEvent,
    ) -> usize {
        let vec = &mut self
            .games
            .entry(game_handle.game_instance_id)
            .or_insert_with(|| Box::new(EventLog::<T>::new()))
            .downcast_mut::<EventLog<T>>()
            .unwrap()
            .conductor_server_events;
        vec.push(event);
        vec.len() - 1
    }

    pub fn log_conductor_client_event<T: Game>(
        &mut self,
        game_handle: GameHandle<T>,
        event: T::ConductorClientEvent,
    ) -> usize {
        let vec = &mut self
            .games
            .entry(game_handle.game_instance_id)
            .or_insert_with(|| Box::new(EventLog::<T>::new()))
            .downcast_mut::<EventLog<T>>()
            .unwrap()
            .conductor_client_events;
        vec.push(event);
        vec.len() - 1
    }

    pub fn log_player_server_event<T: Game>(
        &mut self,
        game_handle: GameHandle<T>,
        player_id: PlayerId,
        event: T::PlayerServerEvent,
    ) -> usize {
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
        vec.len() - 1
    }

    pub fn log_player_client_event<T: Game>(
        &mut self,
        game_handle: GameHandle<T>,
        player_id: PlayerId,
        event: T::PlayerClientEvent,
    ) -> usize {
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
        vec.len() - 1
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
}
