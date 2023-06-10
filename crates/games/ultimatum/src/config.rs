use serde::{Deserialize, Serialize};

use crate::types::{ControlVisibility, ProposalOpenTiming, ProposalRange};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct UltimatumConfig {
    pub proposal_open_timing: ProposalOpenTiming,
    pub control_visibility: ControlVisibility,
    pub proposal_range: ProposalRange,
}
