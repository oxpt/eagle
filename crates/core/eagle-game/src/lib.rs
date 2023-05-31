pub mod system_state;

use bevy_reflect::Reflect;
use eagle_types::{ids::PlayerId, events::GameServerInput};
use serde::{de::DeserializeOwned, Serialize};
use system_state::SystemState;

pub trait Game {
    type Config: Serialize + DeserializeOwned + Reflect;
    type State: Serialize + DeserializeOwned;
    type Conductor: Client;
    type Player: Client;

    fn new(config: Self::Config) -> Self;

    fn handle_input<T: SystemState>(
        &mut self,
        system_state: &T,
        input: GameServerInput<<Self::Conductor as Client>::ServerEvent, <Self::Player as Client>::ServerEvent>,
    ) -> ServerOutput<<Self::Conductor as Client>::ClientEvent, <Self::Player as Client>::ClientEvent>;
}

pub struct ServerOutput<ConductorEvent, PlayerEvent> {
    pub conductor_events: Vec<ConductorEvent>,
    pub player_events: Vec<(PlayerId, PlayerEvent)>,
}

pub trait Client {
    type ServerEvent;
    type ClientEvent;
    type ClientState;

    fn new() -> Self;
    fn handle_server_event(&mut self, event: Self::ServerEvent);
    fn yield_client_events(&mut self) -> Vec<Self::ClientEvent>;
}
