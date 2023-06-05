use crate::{events::ClientEventIndex, client::User};

#[derive(Debug, thiserror::Error)]
pub enum EagleError {
    #[error("Client sends client event with too ahead index: {client_side_index:?} (server side index: {server_side_index:?}) as {user:?}")]
    ClientSendsClientEventWithTooAheadIndex {
        client_side_index: ClientEventIndex,
        server_side_index: ClientEventIndex,
        user: User,
    },
}
