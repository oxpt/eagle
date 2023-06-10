use eagle_game::{
    model::render_context::RenderContext,
    prelude::{Model, ModelContext},
};

use crate::{
    events::{UltimatumPlayerCommand, UltimatumPlayerNotify},
    player_view::PlayerView,
};

pub struct PlayerModel {}

pub struct PlayerInput {}

impl Model for PlayerModel {
    type View = PlayerView;
    type Input = PlayerInput;
    type Notify = UltimatumPlayerNotify;
    type Command = UltimatumPlayerCommand;

    fn new() -> Self {
        todo!()
    }

    fn handle_notify(&mut self, context: &mut impl ModelContext<Self>, notify: Self::Notify) {
        todo!()
    }

    fn render(&self, context: &impl RenderContext) -> Self::View {
        todo!()
    }

    fn handle_input(&self, input: Self::Input) -> Self::Command {
        todo!()
    }
}
