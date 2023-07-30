use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateRoomRequest {
    pub room_key: String,
    pub conductor_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ConductRequest {
    pub conductor_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PlayRequest {
    pub player_id: String,
    pub player_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AddPlayerRequest {
    pub player_uuid: String,
    pub player_id: String,
    pub player_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PlayersResponse {
    pub players: Vec<PlayerResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PlayerResponse {
    pub player_uuid: String,
    pub player_id: String,
}
