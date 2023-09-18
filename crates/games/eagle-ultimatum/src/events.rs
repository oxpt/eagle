use eagle_game::prelude::PlayerId;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::types::{Proposal, Response};

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
pub enum UltimatumConductorCommand {
    StartGame,
    AttachProposer(PlayerId),
    AttachResponder(PlayerId),
    OpenProposal,
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
pub enum UltimatumPlayerCommand {
    UpdateProposal(Proposal),
    SubmitProposal(Proposal),
    Respond(Response),
}
