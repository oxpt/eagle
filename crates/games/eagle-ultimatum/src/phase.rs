use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum Phase {
    #[default]
    WaitingForAttachment,
    Standby,
    Proposing,
    ProposalHidden,
    Responding,
    Result,
}
