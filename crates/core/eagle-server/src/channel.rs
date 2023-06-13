use eagle_types::client::ClientState;
use serde::Serialize;

pub trait Channel: 'static {
    type Error;
    fn notify_view<T: Serialize>(&mut self, view: T) -> Result<(), Self::Error>;
    fn client_state(&self) -> ClientState;
}
