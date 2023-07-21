#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Room {
    room_key: String,
    conductor_password_hash: Vec<u8>,
}
