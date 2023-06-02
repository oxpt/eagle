use serde::{Deserialize, Serialize};

use crate::ids::PlayerId;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum User {
    Conductor,
    Player(PlayerId),
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClientParams {
    pub latest_received_server_event: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ClientState {
    pub alive: bool,
    pub last_successful_communication: Option<chrono::DateTime<chrono::Utc>>,
}
