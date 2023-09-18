use eagle_ultimatum::{conductor_model::UltimatumConductor, player_model::UltimatumPlayer};
use serde::{Deserialize, Serialize};
use tsify::{declare, Tsify};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub mod types;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Message<T> {
    Notify { view: T },
    Pong,
    Ack { index: u64 },
}

#[declare]
type UltimatumPlayerMessage = Message<UltimatumPlayer>;

#[declare]
type UltimatumConductorMessage = Message<UltimatumConductor>;

#[wasm_bindgen]
pub fn player_parse_message(message: &str) -> Result<Message<UltimatumPlayer>, JsValue> {
    serde_json::from_str::<Message<UltimatumPlayer>>(message)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn conductor_parse_message(message: &str) -> Result<Message<UltimatumConductor>, JsValue> {
    serde_json::from_str::<Message<UltimatumConductor>>(message)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[derive(Tsify, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ClientToServerMessage<T> {
    Command { index: u64, command: T },
    Ping,
}

#[derive(Tsify, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerToClientMessage<T> {
    Ack { index: u64 },
    Notify { view: T },
    Pong,
}
