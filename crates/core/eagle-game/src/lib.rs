pub mod game_context;
pub mod server;

use bevy_reflect::Reflect;
use eagle_types::ids::PlayerId;
use game_context::GameContext;
use serde::{de::DeserializeOwned, Serialize};

pub trait Game: Sized {
    type Config: Serialize + DeserializeOwned + Reflect;
    type State: Serialize + DeserializeOwned;
    type Conductor: Client;
    type Player: Client;

    fn name() -> &'static str;

    fn new(config: Self::Config) -> Self;

    fn handle_conductor_event(
        &mut self,
        context: &mut impl GameContext<Self>,
        event: <Self::Conductor as Client>::ServerEvent,
    );

    fn handle_player_event(
        &mut self,
        context: &mut impl GameContext<Self>,
        player_id: PlayerId,
        event: <Self::Player as Client>::ServerEvent,
    );
}

pub trait Client {
    type ServerEvent;
    type ClientEvent;
    type ClientState;

    fn new() -> Self;
    fn handle_server_event(&mut self, event: Self::ServerEvent);
    fn yield_client_events(&mut self) -> Vec<Self::ClientEvent>;
}
