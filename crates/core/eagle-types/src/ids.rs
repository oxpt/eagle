use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct GameId(pub Uuid);

impl GameId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
/// A unique identifier for a game client that can be host/guest.
pub struct ClientId(pub Uuid);

impl ClientId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PlayerId(pub Uuid);

impl PlayerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
