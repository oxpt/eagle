use eagle_types::messages::ClientToServerMessage;
use serde::Serialize;

pub trait Channel {
    type Error;

    fn send_message<T: Serialize>(
        &self,
        message: ClientToServerMessage<T>,
    ) -> Result<(), Self::Error>;
}
