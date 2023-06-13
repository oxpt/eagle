use serde::{Deserialize, Serialize};

use crate::ids::PlayerId;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum User {
    Conductor,
    Player(PlayerId),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ClientState {
    pub last_successful_communication: Option<chrono::DateTime<chrono::Utc>>,
}

impl ClientState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_last_successful_communication(
        &mut self,
        datetime: chrono::DateTime<chrono::Utc>,
    ) {
        self.last_successful_communication = Some(datetime);
    }
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            last_successful_communication: None,
        }
    }
}
