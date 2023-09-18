use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::types::{Proposal, Response};

#[derive(Tsify, Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UltimatumConductor {
    proposal: Option<Proposal>,
    proposed: bool,
    response: Option<Response>,
    errors: Vec<String>,
}

impl UltimatumConductor {
    pub fn new() -> Self {
        Self::default()
    }
}
