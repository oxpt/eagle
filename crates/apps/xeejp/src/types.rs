use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use uuid::Uuid;

#[derive(Tsify, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoomRequest {
    pub room_name: String,
    pub room_key: String,
    pub conductor_password: String,
}

#[derive(Tsify, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RoomsResponse {
    pub rooms: Rooms,
}

#[derive(Tsify, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct Rooms(pub HashMap<String, Room>);

#[derive(Tsify, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Room {
    pub name: String,
}

#[derive(Tsify, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConductRequest {
    pub client_id: String,
    pub conductor_password: String,
}

#[derive(Tsify, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlayRequest {
    pub client_id: Uuid,
    pub player_id: Uuid,
    pub player_password: String,
}

#[derive(Tsify, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddPlayerRequest {
    pub player_uuid: String,
    pub player_id: String,
    pub player_password: String,
}

#[derive(Tsify, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlayersResponse {
    pub players: Vec<PlayerResponse>,
}

#[derive(Tsify, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    pub player_uuid: String,
    pub player_id: String,
}
