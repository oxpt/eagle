use eagle_types::{ids::ClientId, client::ClientState};
use serde::Serialize;

/// This term, channel, is used in the context of game server and not in the context of games.
pub trait Channel {
    type Error: std::fmt::Debug;
    fn client_id(&self) -> ClientId;
    fn send<T: Serialize>(&self, event: T) -> Result<(), Self::Error>;
    fn close(&self) -> Result<(), Self::Error>;
    fn client_state(&self) -> ClientState;
}

