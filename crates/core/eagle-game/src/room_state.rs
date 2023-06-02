use std::collections::{BTreeMap, BTreeSet};

use eagle_types::{client::User, ids::ClientId};
use serde::{Deserialize, Serialize};

use crate::event_history::EventHistory;

#[derive(Default, Serialize, Deserialize)]
pub struct RoomState {
    // This should be a bidirectional map but BTreeMap is sufficient for now
    // because reverse lookups is used only when a client starts to connect.
    pub(crate) client_attachments: BTreeMap<User, BTreeSet<ClientId>>,
    pub(crate) sent_server_events: EventHistory,
    pub(crate) event_history: EventHistory,
}

impl RoomState {
    fn new() -> Self {
        Self::default()
    }
}
