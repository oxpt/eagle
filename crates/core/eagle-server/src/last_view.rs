use std::collections::BTreeMap;

use eagle_game::prelude::Game;
use eagle_types::ids::PlayerId;

pub(crate) struct LastViews<T: Game> {
    conductor: Option<T::ConductorView>,
    players: BTreeMap<PlayerId, T::PlayerView>,
}

impl<T: Game> Default for LastViews<T> {
    fn default() -> Self {
        Self {
            conductor: None,
            players: BTreeMap::new(),
        }
    }
}

impl<T: Game> LastViews<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_conductor_view(&mut self) -> Option<&T::ConductorView> {
        self.conductor.as_ref()
    }

    pub fn get_player_view(&mut self, player_id: PlayerId) -> Option<&T::PlayerView> {
        self.players.get(&player_id)
    }

    pub fn update_conductor_view(&mut self, view: &T::ConductorView) -> UpdateResult {
        if self.conductor.as_ref() != Some(view) {
            self.conductor = Some(view.clone());
            UpdateResult::Updated
        } else {
            UpdateResult::NotUpdated
        }
    }

    pub fn update_player_view(
        &mut self,
        player_id: PlayerId,
        view: &T::PlayerView,
    ) -> UpdateResult {
        if self.players.get(&player_id) != Some(view) {
            self.players.insert(player_id, view.clone());
            UpdateResult::Updated
        } else {
            UpdateResult::NotUpdated
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum UpdateResult {
    Updated,
    NotUpdated,
}
