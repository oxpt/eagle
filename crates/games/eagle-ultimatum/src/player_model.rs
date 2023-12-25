use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::types::{Proposal, Response};

#[derive(Tsify, Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UltimatumPlayer {
    #[default]
    Standby,
    WaitingForProposal {
        realtime_proposal: Option<Proposal>,
    },
    Proposing,
    WaitingForResponse,
    Responding {
        proposal: Proposal,
    },
    Result {
        proposal: Proposal,
        response: Response,
    },
}
