use serde::{Deserialize, Serialize};

use crate::events::{CommandIndex, NotifyIndex};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerToClientMessage<T> {
    pub index: NotifyIndex,
    pub notify: T,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientToServerMessage<T> {
    pub index: CommandIndex,
    pub request: T,
}
