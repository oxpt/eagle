use crate::ids::PlayerId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// A struct that contains information about a player for sharing among games.
pub struct PlayerInformation {
    pub player_id: PlayerId,
    /// Predefined personal identifier, e.g. student number.
    pub personal_id: Option<String>,
    /// Player name. Don't use this for identification.
    pub name: Option<String>,
}

impl PlayerInformation {
    pub fn new(player_id: PlayerId) -> Self {
        Self {
            player_id,
            personal_id: None,
            name: None,
        }
    }
}
