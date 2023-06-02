use eagle_types::ids::{PlayerId, GameInstanceId};
use serde::{de::DeserializeOwned, Serialize};

use crate::room_state::RoomState;

pub trait Game: Sized + Serialize + DeserializeOwned {
    type ConductorServerEvent: Serialize + DeserializeOwned;
    type ConductorClientEvent: Serialize + DeserializeOwned;
    type PlayerServerEvent: Serialize + DeserializeOwned;
    type PlayerClientEvent: Serialize + DeserializeOwned;
    type ConductorClient: Client<Event = Self::ConductorServerEvent>;
    type PlayerClient: Client<Event = Self::PlayerServerEvent>;

    fn name() -> &'static str;

    fn handle_conductor_event(
        &mut self,
        context: &mut RoomState,
        event: Self::ConductorClientEvent,
    );

    fn handle_player_event(
        &mut self,
        context: &mut RoomState,
        player_id: PlayerId,
        event: Self::PlayerClientEvent,
    );
}

pub trait Client {
    type Event;
    fn handle_server_event(&mut self, event: Self::Event);
}

#[derive(Debug, Clone, Copy)]
pub struct GameHandle<T: Game> {
    game: T,
    pub game_instance_id: GameInstanceId,
}
