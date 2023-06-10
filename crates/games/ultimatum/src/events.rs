use eagle_game::prelude::PlayerId;
use serde::{Deserialize, Serialize};

use crate::{
    error::UltimatumError,
    types::{Proposal, Response},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UltimatumConductorCommand {
    StartGame,
    AttachProposer(PlayerId),
    AttachResponder(PlayerId),
    OpenProposal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UltimatumConductorNotify {
    UpdateProposal(Proposal),
    Proposed,
    Response(Response),
    Error(UltimatumError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UltimatumPlayerCommand {
    UpdateProposal(Proposal),
    SubmitProposal(Proposal),
    Respond(Response),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UltimatumPlayerNotify {
    YouAreProposer,
    YouAreResponder,
    Proposal(Proposal),
}
