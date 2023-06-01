use eagle_types::ids::ClientId;
use serde::Serialize;

pub trait Channel {
    type Error;
    fn client_id(&self) -> ClientId;
    fn send<T: Serialize>(&self, event: T) -> Result<(), Self::Error>;
    fn close(&self) -> Result<(), Self::Error>;
}
