pub mod context;
pub mod handle;

use std::fmt::Debug;

use eagle_types::{events::SystemCommand, ids::PlayerId};
use serde::{de::DeserializeOwned, Serialize};

use crate::model::Model;
use context::GameContext;

pub trait Game: Debug + Sized + 'static {
    type Config: Default + Debug + Clone + Serialize + DeserializeOwned + 'static;
    type ConductorNotify: Debug + Clone + Serialize + DeserializeOwned + 'static;
    type ConductorCommand: Debug + Clone + Serialize + DeserializeOwned + 'static;
    type PlayerNotify: Debug + Clone + Serialize + DeserializeOwned + 'static;
    type PlayerCommand: Debug + Clone + Serialize + DeserializeOwned + 'static;
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

    fn handle_system_command(
        &mut self,
        context: &mut impl GameContext<Self>,
        command: SystemCommand,
    );
}
