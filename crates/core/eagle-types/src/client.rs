use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::ids::PlayerId;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum User {
    Conductor,
    Player(PlayerId),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct ClientState {
    last_successful_communication: Option<DateTime<Utc>>,
    last_error: Option<DateTime<Utc>>,
    continuous_error_count: u8,
}

impl ClientState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn last_successful_communication(&self) -> Option<DateTime<Utc>> {
        self.last_successful_communication
    }

    pub fn last_error(&self) -> Option<DateTime<Utc>> {
        self.last_error
    }

    pub fn continuous_error_count(&self) -> u8 {
        self.continuous_error_count
    }

    pub fn update_last_successful_communication(&mut self, datetime: DateTime<Utc>) {
        self.last_successful_communication = Some(datetime);
        self.continuous_error_count = 0;
    }

    pub fn update_last_error(&mut self, datetime: DateTime<Utc>) {
        self.last_error = Some(datetime);
        self.continuous_error_count += 1;
    }
}
