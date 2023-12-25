use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::{
    phase::Phase,
    types::{Proposal, Response},
};

#[derive(Tsify, Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct UltimatumConductor {
    pub phase: Phase,
    pub proposal: Option<Proposal>,
    pub response: Option<Response>,
    pub errors: Vec<String>,
}

impl UltimatumConductor {
    pub fn new() -> Self {
        Self::default()
    }
}
