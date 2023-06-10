use eagle_game::{
    model::render_context::RenderContext,
    prelude::{Model, ModelContext},
};

use crate::{
    conductor_view::ConductorView,
    events::{UltimatumConductorCommand, UltimatumConductorNotify},
};

pub struct UltimatumConductor {}

pub struct UltimatumConductorInput {}

impl Model for UltimatumConductor {
    type View = ConductorView;
    type Input = UltimatumConductorInput;
    type Notify = UltimatumConductorNotify;
    type Command = UltimatumConductorCommand;

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
