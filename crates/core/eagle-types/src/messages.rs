use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ClientToServerMessage<T> {
    Command {
        index: ClientCommandIndex,
        command: T,
    },
    Ping,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientCommandIndex(pub usize);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerToClientMessage<T> {
    Ack { index: ClientCommandIndex },
    Notify { view: T },
    Pong,
}
