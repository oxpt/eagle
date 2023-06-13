use crate::types::{Proposal, Response};

#[derive(Debug, Default)]
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
