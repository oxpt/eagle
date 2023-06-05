use chrono::{DateTime, Utc};
use eagle_types::{client::ClientState, ids::GameInstanceId};

use crate::effectful::Effectful;

#[derive(Default)]
pub struct EffHandler {
    pub client_states: Effectful<Vec<ClientState>>,
    pub now: Effectful<DateTime<Utc>>,
    pub new_game: Effectful<GameInstanceId>,
}
