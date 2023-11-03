use eagle_types::{client::ClientState, messages::ServerToClientMessage};
use serde::Serialize;

pub trait Channel: 'static {
    type Error: std::error::Error;
    fn send_message<T: Serialize>(
        &self,
        message: ServerToClientMessage<T>,
    ) -> Result<(), Self::Error>;
    fn client_state(&self) -> ClientState;
}
