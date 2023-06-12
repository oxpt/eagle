use eagle_game::{
    model::render_context::RenderContext,
    prelude::{Model, ModelContext},
};

use crate::{
    conductor_view::ConductorView,
    events::{UltimatumConductorCommand, UltimatumConductorNotify},
    types::Proposal,
};

pub struct UltimatumConductor {
    proposal: Option<Proposal>,
}

pub struct UltimatumConductorInput {}

impl Model for UltimatumConductor {
    type View = ConductorView;
    type Input = UltimatumConductorInput;
    type Notify = UltimatumConductorNotify;
    type Command = UltimatumConductorCommand;

    fn new() -> Self {
        todo!()
    }

    fn handle_notify(&mut self, _: &mut impl ModelContext<Self>, notify: Self::Notify) {
        match notify {
            UltimatumConductorNotify::UpdateProposal(_) => todo!(),
            UltimatumConductorNotify::Proposed => todo!(),
            UltimatumConductorNotify::Response(_) => todo!(),
            UltimatumConductorNotify::Error(_) => todo!(),
        }
    }

    fn render(&self, context: &impl RenderContext) -> Self::View {
        todo!()
    }

    fn handle_input(&self, input: Self::Input) -> Self::Command {
        todo!()
    }
}
