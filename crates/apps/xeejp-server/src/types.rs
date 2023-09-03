use eagle_types::ids::PlayerId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Player {
    player_id: PlayerId,
    label: String,
    player_password_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Start {
    pub conductor_password: String,
}
