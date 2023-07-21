use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::types::{Proposal, Response};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[wasm_bindgen]
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
