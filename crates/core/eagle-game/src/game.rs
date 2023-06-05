use eagle_types::{ids::PlayerId, events::SystemEvent};
use serde::{de::DeserializeOwned, Serialize};

use crate::context::Context;

pub trait Game: Sized + Serialize + DeserializeOwned + 'static {
    type Config: Clone + Serialize + DeserializeOwned + 'static;
    type ConductorServerEvent: Clone + Serialize + DeserializeOwned + 'static;
    type ConductorClientEvent: Clone + Serialize + DeserializeOwned + 'static;
    type PlayerServerEvent: Clone + Serialize + DeserializeOwned + 'static;
    type PlayerClientEvent: Clone + Serialize + DeserializeOwned + 'static;
    type ConductorClient: Client<Event = Self::ConductorServerEvent> + 'static;
    type PlayerClient: Client<Event = Self::PlayerServerEvent> + 'static;

    fn new(config: Self::Config) -> Self;

    fn name() -> &'static str;

    fn handle_conductor_event(&mut self, context: &mut Context<Self>, event: Self::ConductorClientEvent);

    fn handle_player_event(
        &mut self,
        context: &mut Context<Self>,
        player_id: PlayerId,
        event: Self::PlayerClientEvent,
    );

    fn handle_system_event(
        &mut self,
        context: &mut Context<Self>,
        event: SystemEvent,
    );

    fn log_error(&mut self, error: anyhow::Error);
}

pub trait Client {
    type Event: Clone + Serialize + DeserializeOwned + 'static;
    fn new() -> Self;
    fn handle_server_event(&mut self, event: Self::Event);
}
