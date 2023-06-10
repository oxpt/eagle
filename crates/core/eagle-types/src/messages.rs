use serde::{Serialize, Deserialize};

use crate::events::NotifyIndex;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerToClientMessage<T> {
    pub index: NotifyIndex,
    pub notify: T,
}
