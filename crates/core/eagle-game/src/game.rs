use eagle_types::{ids::PlayerId, events::SystemCommand};
use serde::{de::DeserializeOwned, Serialize};

use crate::context::Context;

pub trait Game: Sized + 'static {
    type Config: Clone + Serialize + DeserializeOwned + 'static;
    type ConductorNotify: Clone + Serialize + DeserializeOwned + 'static;
    type ConductorCommand: Clone + Serialize + DeserializeOwned + 'static;
    type PlayerNotify: Clone + Serialize + DeserializeOwned + 'static;
    type PlayerCommand: Clone + Serialize + DeserializeOwned + 'static;
    type Conductor: Frontend<Notify = Self::ConductorNotify, Command = Self::ConductorCommand>;
    type Player: Frontend<Notify = Self::PlayerNotify, Command = Self::PlayerCommand>;

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
}

pub trait Frontend: 'static {
    type Notify: Clone + Serialize + DeserializeOwned + 'static;
    type Command: Clone + Serialize + DeserializeOwned + 'static;

    fn new() -> Self;

    fn handle_notify(&mut self, notify: Self::Notify);
}

pub use self::Frontend as Player;
pub use self::Frontend as Conductor;
