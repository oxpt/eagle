use eagle_types::{
    client::ClientState,
    ids::{ClientId, GameInstanceId}, events::ServerEventIndex,
};
use serde::Serialize;

/// This term, channel, is used in the context of game server and not in the context of games.
pub trait Channel: 'static {
    type Error: std::fmt::Debug;
    fn client_id(&self) -> ClientId;
    fn send<T: Serialize>(
        &self,
        game_instance_id: GameInstanceId,
        index: ServerEventIndex,
        event: T,
    ) -> Result<(), Self::Error>;
    fn close(&self) -> Result<(), Self::Error>;
    fn client_state(&self) -> ClientState;
}
