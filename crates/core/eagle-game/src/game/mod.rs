pub mod context;
pub mod handle;

use eagle_types::{events::SystemCommand, ids::PlayerId};
use serde::{de::DeserializeOwned, Serialize};

use crate::{model::Model};
use context::GameContext;

pub trait Game: Sized + 'static {
    type Config: Clone + Serialize + DeserializeOwned + 'static;
    type ConductorNotify: Clone + Serialize + DeserializeOwned + 'static;
    type ConductorCommand: Clone + Serialize + DeserializeOwned + 'static;
    type PlayerNotify: Clone + Serialize + DeserializeOwned + 'static;
    type PlayerCommand: Clone + Serialize + DeserializeOwned + 'static;
    type Conductor: Model<Notify = Self::ConductorNotify>;
    type Player: Model<Notify = Self::PlayerNotify>;

    fn new(config: Self::Config) -> Self;

    fn name() -> &'static str;

    fn handle_conductor_command(
        &mut self,
        context: &mut impl GameContext<Self>,
        command: Self::ConductorCommand,
    );

    fn handle_player_command(
        &mut self,
        context: &mut impl GameContext<Self>,
        player_id: PlayerId,
        command: Self::PlayerCommand,
    );

    fn handle_system_command(&mut self, context: &mut impl GameContext<Self>, command: SystemCommand);
}
