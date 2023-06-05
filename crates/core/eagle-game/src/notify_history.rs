use std::collections::BTreeMap;

use eagle_types::{events::NotifyIndex, ids::PlayerId};

use crate::Game;

pub(crate) struct NotifyHistory<T: Game> {
    /// Used to re-send events to new clients or reconected clients.
    // Any is Notifies<ConductorNotify> or Notifies<PlayerNotify>
    conductor: Vec<T::ConductorNotify>,
    players: BTreeMap<PlayerId, Vec<T::PlayerNotify>>,
}

impl<T: Game> NotifyHistory<T> {
    pub fn new() -> Self {
        Self {
            conductor: Default::default(),
            players: Default::default(),
        }
    }

    pub fn log_conductor_notify(&mut self, notify: T::ConductorNotify) -> NotifyIndex {
        self.conductor.push(notify);
        NotifyIndex::from_len(self.conductor.len())
    }

    pub fn log_player_notify(
        &mut self,
        player_id: PlayerId,
        notify: T::PlayerNotify,
    ) -> NotifyIndex {
        self.players
            .entry(player_id)
            .or_insert_with(Vec::new)
            .push(notify);
        NotifyIndex::from_len(self.players[&player_id].len())
    }

    pub fn get_conductor_notifies(&self) -> impl Iterator<Item = &T::ConductorNotify> {
        self.conductor.iter()
    }

    pub fn get_player_notifies<'a>(
        &'a self,
        player_id: PlayerId,
    ) -> impl Iterator<Item = &'a T::PlayerNotify> {
        self.players.get(&player_id).into_iter().flatten()
    }
}
