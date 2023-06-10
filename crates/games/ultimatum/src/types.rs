use std::ops::RangeInclusive;

use eagle_game::prelude::PlayerId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ProposalRange {
    min: u16,
    max: u16,
}

impl ProposalRange {
    pub fn new(min: u16, max: u16) -> Result<Self, String> {
        if min > max {
            return Err("min must be less than or equal to max".to_string());
        }
        Ok(Self { min, max })
    }

    pub fn range(&self) -> RangeInclusive<u16> {
        self.min..=self.max
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProposalOpenTiming {
    Immediate,
    ByConductor, // Includes a consistent delay
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlVisibility {
    Realtime,
    Hidden,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Players {
    pub proposer: PlayerId,
    pub responder: PlayerId,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Proposal {
    pub proposal: u16,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Response {
    Yes,
    No,
}
