use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ClientToServerMessage<T> {
    Command { index: CommandIndex, command: T },
    Ping,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandIndex(usize);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerToClientMessage<T> {
    Ack { index: CommandIndex },
    Notify { view: T },
    Pong,
}
