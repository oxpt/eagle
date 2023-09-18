use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Tsify, Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UltimatumPlayer {}
