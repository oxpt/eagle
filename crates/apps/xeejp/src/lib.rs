pub mod types;

use eagle_types::messages::ServerToClientMessage;
use eagle_ultimatum::{conductor_model::UltimatumConductor, player_model::UltimatumPlayer};

use futures::{stream::SplitSink, SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use js_sys::JSON;
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
pub struct CommandSender {
    sink: SplitSink<WebSocket, Message>,
}

impl CommandSender {
    pub fn new(sink: SplitSink<WebSocket, Message>) -> Self {
        Self { sink }
    }

    pub async fn send(&mut self, message: JsValue) -> Result<(), String> {
        let json = JSON::stringify(&message).unwrap();
        self.sink
            .send(Message::Text(json.into()))
            .await
            .map_err(|e| e.to_string())
    }
}

pub async fn connect_to_server<T: Serialize + DeserializeOwned>(
    addr: &str,
    on_update: js_sys::Function,
) -> Result<CommandSender, String> {
    let ws = WebSocket::open(addr).map_err(|e| e.to_string())?;
    let (w, mut r) = ws.split();
    spawn_local(async move {
        while let Some(result) = r.next().await {
            match result {
                Ok(Message::Text(json)) => {
                    let message = serde_json::from_str::<ServerToClientMessage<T>>(&json)
                        .map_err(|e| e.to_string())
                        .unwrap();
                    match message {
                        ServerToClientMessage::Notify { view } => {
                            let value = serde_wasm_bindgen::to_value(&view).unwrap();
                            on_update.call1(&JsValue::NULL, &value).unwrap();
                        }
                        ServerToClientMessage::Pong => {}
                        ServerToClientMessage::Ack { index } => {
                            todo!("ack {}", index.0)
                        }
                    }
                }
                Ok(Message::Bytes(_)) => {
                    web_sys::console::error_1(&"unexpected binary message from server".into());
                }
                Err(e) => {
                    web_sys::console::error_1(&e.to_string().into());
                }
            }
        }
        web_sys::console::log_1(&"websocket closed".into());
    });
    Ok(CommandSender::new(w))
}

#[wasm_bindgen]
pub async fn connect_to_server_as_conductor(
    addr: &str,
    on_update: js_sys::Function,
) -> Result<CommandSender, String> {
    connect_to_server::<UltimatumConductor>(addr, on_update).await
}

#[wasm_bindgen]
pub async fn connect_to_server_as_player(
    addr: &str,
    on_update: js_sys::Function,
) -> Result<CommandSender, String> {
    connect_to_server::<UltimatumPlayer>(addr, on_update).await
}
