use eagle_game::{EffHandler, Effectful};
use eagle_types::{client::ClientState, ids::GameInstanceId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EffectOutcomes {
    client_states: Vec<Vec<ClientState>>,
    now: Vec<chrono::DateTime<chrono::Utc>>,
    new_game: Vec<GameInstanceId>,
}

impl EffectOutcomes {
    pub fn new(
        client_states: Effectful<Vec<ClientState>>,
        now: Effectful<chrono::DateTime<chrono::Utc>>,
        new_game: Effectful<GameInstanceId>,
    ) -> Self {
        Self {
            client_states: client_states.outcomes(),
            now: now.outcomes(),
            new_game: new_game.outcomes(),
        }
    }
}

impl From<EffHandler> for EffectOutcomes {
    fn from(eff_handler: EffHandler) -> Self {
        Self {
            client_states: eff_handler.client_states.outcomes(),
            now: eff_handler.now.outcomes(),
            new_game: eff_handler.new_game.outcomes(),
        }
    }
}

impl Into<EffHandler> for EffectOutcomes {
    fn into(self) -> EffHandler {
        EffHandler {
            client_states: Effectful::replay(self.client_states),
            now: Effectful::replay(self.now),
            new_game: Effectful::replay(self.new_game),
        }
    }
}
