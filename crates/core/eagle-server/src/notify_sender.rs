use eagle_types::{client::ClientState, ids::ClientId, messages::ServerToClientMessage};
use serde::Serialize;

/// This term, channel, is used in the context of game server and not in the context of games.
pub trait NotifySender: 'static {
    type Error: std::fmt::Debug;
    fn client_id(&self) -> ClientId;
    fn send<T: Serialize>(&self, message: ServerToClientMessage<T>) -> Result<(), Self::Error>;
    fn close(&self) -> Result<(), Self::Error>;
    fn client_state(&self) -> ClientState;
}
