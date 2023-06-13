pub mod context;
pub mod handle;
pub mod render_context;

use std::fmt::Debug;

use eagle_types::{events::SystemCommand, ids::PlayerId};
use serde::{de::DeserializeOwned, Serialize};

use context::GameContext;

use self::render_context::RenderContext;

pub trait Game: Debug + Sized + 'static {
    type Config: Default + Debug + Clone + Serialize + DeserializeOwned + 'static;
    type ConductorCommand: Debug + Clone + Serialize + DeserializeOwned + 'static;
    type PlayerCommand: Debug + Clone + Serialize + DeserializeOwned + 'static;
    type ConductorView: Debug + Clone + PartialEq + Serialize + DeserializeOwned + 'static;
    type PlayerView: Debug + Clone + PartialEq + Serialize + DeserializeOwned + 'static;

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

    fn render_conductor(&self, context: &impl RenderContext) -> Self::ConductorView;

    fn render_player(&self, context: &impl RenderContext, player_id: PlayerId) -> Self::PlayerView;
}
