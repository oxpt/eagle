use chrono::{DateTime, Utc};
use eagle_types::{client::ClientState, ids::GameInstanceId};

use crate::effectful::Effectful;

pub struct EffHandler<'a> {
    pub client_states: &'a mut Effectful<Vec<ClientState>>,
    pub now: &'a mut Effectful<DateTime<Utc>>,
    pub new_game: &'a mut Effectful<GameInstanceId>,
}

impl EffHandler<'_> {
    pub fn new<'a>(
        client_states: &'a mut Effectful<Vec<ClientState>>,
        now: &'a mut Effectful<DateTime<Utc>>,
        new_game: &'a mut Effectful<GameInstanceId>,
    ) -> EffHandler<'a> {
        EffHandler {
            client_states,
            now,
            new_game,
        }
    }
}
