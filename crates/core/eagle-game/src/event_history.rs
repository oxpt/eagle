use std::collections::BTreeMap;

use eagle_types::ids::{GameInstanceId, PlayerId};
use serde::{Deserialize, Serialize};

use crate::{game::{Game, GameHandle}, serialized_event::SerializedEvent};

#[derive(Default, Serialize, Deserialize)]
/// This stores server and client events in RON format.
pub(crate) struct EventHistory {
    // This Any is a GameEventHistory<T> where T is the game type.
    conductor: BTreeMap<GameInstanceId, EventLog>,
    player: BTreeMap<PlayerId, BTreeMap<GameInstanceId, EventLog>>,
}

#[derive(Default, Serialize, Deserialize)]
struct EventLog {
    pub client_events: Vec<String>,
    pub server_events: Vec<String>,
}

impl EventHistory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn log_conductor_server_event<T: Game>(
        &mut self,
        game_handle: &GameHandle<T>,
        event: T::ConductorServerEvent,
    ) {
        self.conductor
            .entry(game_handle.game_instance_id)
            .or_default()
            .server_events
            .push(ron::ser::to_string(&event).unwrap());
    }

    pub fn log_conductor_client_event<T: Game>(
        &mut self,
        game_handle: &GameHandle<T>,
        event: T::ConductorClientEvent,
    ) {
        self.conductor
            .entry(game_handle.game_instance_id)
            .or_default()
            .client_events
            .push(ron::ser::to_string(&event).unwrap());
    }

    pub fn log_player_server_event<T: Game>(
        &mut self,
        game_handle: &GameHandle<T>,
        player_id: PlayerId,
        event: T::PlayerServerEvent,
    ) {
        self.player
            .entry(player_id)
            .or_default()
            .entry(game_handle.game_instance_id)
            .or_default()
            .server_events
            .push(ron::ser::to_string(&event).unwrap());
    }

    pub fn log_player_client_event<T: Game>(
        &mut self,
        game_handle: &GameHandle<T>,
        player_id: PlayerId,
        event: T::PlayerClientEvent,
    ) {
        self.player
            .entry(player_id)
            .or_default()
            .entry(game_handle.game_instance_id)
            .or_default()
            .client_events
            .push(ron::ser::to_string(&event).unwrap());
    }

    pub fn conductor_server_events<T: Game>(
        &self,
        game_handle: &GameHandle<T>,
    ) -> impl Iterator<Item = SerializedEvent<T::ConductorServerEvent>> {
        self.conductor
            .get(&game_handle.game_instance_id)
            .unwrap()
            .server_events
            .iter()
            .map(|event| SerializedEvent {
                event,
                deserialize: |event| ron::de::from_str(event).unwrap(),
            })
    }

    pub fn conductor_client_events<T: Game>(
        &self,
        game_handle: &GameHandle<T>,
    ) -> impl Iterator<Item = SerializedEvent<T::ConductorClientEvent>> {
        self.conductor
            .get(&game_handle.game_instance_id)
            .unwrap()
            .client_events
            .iter()
            .map(|event| SerializedEvent {
                event,
                deserialize: |event| ron::de::from_str(event).unwrap(),
            })
    }

    pub fn player_server_events<T: Game>(
        &self,
        game_handle: &GameHandle<T>,
        player_id: PlayerId,
    ) -> impl Iterator<Item = SerializedEvent<T::PlayerServerEvent>> {
        self.player
            .get(&player_id)
            .unwrap()
            .get(&game_handle.game_instance_id)
            .unwrap()
            .server_events
            .iter()
            .map(|event| SerializedEvent {
                event,
                deserialize: |event| ron::de::from_str(event).unwrap(),
            })
    }

    pub fn player_client_events<T: Game>(
        &self,
        game_handle: &GameHandle<T>,
        player_id: PlayerId,
    ) -> impl Iterator<Item = SerializedEvent<T::PlayerClientEvent>> {
        self.player
            .get(&player_id)
            .unwrap()
            .get(&game_handle.game_instance_id)
            .unwrap()
            .client_events
            .iter()
            .map(|event| SerializedEvent {
                event,
                deserialize: |event| ron::de::from_str(event).unwrap(),
            })
    }
}
