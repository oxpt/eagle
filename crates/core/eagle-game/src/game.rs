use eagle_types::{ids::PlayerId, events::SystemCommand};
use serde::{de::DeserializeOwned, Serialize};

use crate::context::Context;

pub trait Game: Sized + 'static {
    type Config: Clone + Serialize + DeserializeOwned + 'static;
    type ConductorNotify: Clone + Serialize + DeserializeOwned + 'static;
    type ConductorCommand: Clone + Serialize + DeserializeOwned + 'static;
    type PlayerNotify: Clone + Serialize + DeserializeOwned + 'static;
    type PlayerCommand: Clone + Serialize + DeserializeOwned + 'static;
    type ConductorClient: Client<Event = Self::ConductorNotify> + 'static;
    type PlayerClient: Client<Event = Self::PlayerNotify> + 'static;

    fn new(config: Self::Config) -> Self;

    fn name() -> &'static str;

    fn handle_conductor_command(&mut self, context: &mut Context<Self>, command: Self::ConductorCommand);

    fn handle_player_command(
        &mut self,
        context: &mut Context<Self>,
        player_id: PlayerId,
        command: Self::PlayerCommand,
    );

    fn handle_system_command(
        &mut self,
        context: &mut Context<Self>,
        command: SystemCommand,
    );

    fn log_error(&mut self, error: anyhow::Error);
}

pub trait Client {
    type Event: Clone + Serialize + DeserializeOwned + 'static;
    fn new() -> Self;
    fn handle_server_event(&mut self, event: Self::Event);
}
