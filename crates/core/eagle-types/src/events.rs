use serde::{Deserialize, Serialize};

use crate::ids::PlayerId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemCommand {
    JoinPlayer(PlayerId),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NotifyIndex(pub usize);

impl NotifyIndex {
    pub fn from_len(len: usize) -> Self {
        Self(len - 1)
    }

    pub fn skip(self) -> usize {
        // if index:0 is received by the client, it means we need to skip one event.
        // if index:1 is received by the client, it means we need to skip two events.
        self.0 + 1
    }

    pub fn next(self) -> NotifyIndex {
        NotifyIndex(self.0 + 1)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CommandIndex(usize);

impl CommandIndex {
    pub fn next_from_len(len: usize) -> Self {
        Self(len)
    }

    pub fn from_len(len: usize) -> Self {
        Self(len - 1)
    }

    pub fn index(self) -> usize {
        self.0
    }

    pub fn is_next_of(self, other: Self) -> IsNextOf {
        if self.0 == other.0 + 1 {
            IsNextOf::Yes
        } else if self < other {
            IsNextOf::No
        } else {
            IsNextOf::TooFarAhead
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IsNextOf {
    Yes,
    No,
    TooFarAhead,
}

#[cfg(test)]
mod server_event_index_test {
    use super::*;
    #[test]
    fn is_next_of() {
        assert_eq!(CommandIndex(1).is_next_of(CommandIndex(1)), IsNextOf::Yes);
        assert_eq!(CommandIndex(1).is_next_of(CommandIndex(2)), IsNextOf::No);
        assert_eq!(
            CommandIndex(3).is_next_of(CommandIndex(1)),
            IsNextOf::TooFarAhead
        );
    }
}
