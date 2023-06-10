use eagle_game::prelude::PlayerId;
use serde::{Deserialize, Serialize};

use crate::types::{Players, Proposal, Response};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Phase {
    WaitingForAttachment {
        proposer: Option<PlayerId>,
        responder: Option<PlayerId>,
    },
    /// Waiting for UltimatumConductor::StartGame
    Standby { players: Players },
    Requesting {
        players: Players,
        proposal: Option<Proposal>,
    },
    ProposalHidden {
        players: Players,
        proposal: Proposal, // 0 to 100
    },
    Responding {
        players: Players,
        proposal: Proposal, // 0 to 100
    },
    Result {
        players: Players,
        proposal: Proposal,
        response: Response,
    },
}
