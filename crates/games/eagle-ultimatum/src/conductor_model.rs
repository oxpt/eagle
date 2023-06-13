use eagle_game::{
    model::render_context::RenderContext,
    prelude::{Model, ModelContext},
};

use crate::{
    conductor_view::ConductorView,
    events::{UltimatumConductorCommand, UltimatumConductorNotify},
    types::{Proposal, Response},
};

#[derive(Debug, Default)]
pub struct UltimatumConductor {
    proposal: Option<Proposal>,
    proposed: bool,
    response: Option<Response>,
    errors: Vec<String>,
}

pub struct UltimatumConductorInput {}

impl Model for UltimatumConductor {
    type View = ConductorView;
    type Input = UltimatumConductorInput;
    type Notify = UltimatumConductorNotify;
    type Command = UltimatumConductorCommand;

    fn new() -> Self {
        Default::default()
    }

    fn handle_notify(&mut self, _: &mut impl ModelContext<Self>, notify: Self::Notify) {
        match notify {
            UltimatumConductorNotify::UpdateProposal(proposal) => self.proposal = Some(proposal),
            UltimatumConductorNotify::Proposed => self.proposed = true,
            UltimatumConductorNotify::Response(response) => self.response = Some(response),
            UltimatumConductorNotify::Error(error) => self.errors.push(format!("{:?}", error)),
        }
    }

    fn render(&self, _: &impl RenderContext) -> Self::View {
        ConductorView {}
    }

    fn handle_input(&self, input: Self::Input) -> Self::Command {
        todo!()
    }
}
