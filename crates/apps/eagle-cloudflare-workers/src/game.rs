use std::{collections::BTreeMap, sync::Arc};

use eagle_types::ids::{ClientId, PlayerId};

use futures::{lock::Mutex, StreamExt};
use serde::Deserialize;
use uuid::Uuid;
use worker::{WebSocket, *};

pub struct GameState {
    // A conductor can have multiple devices to manage
    conductor_channels: Vec<Channel>,
    // This must not HashMap, but it does not have to be BTreeMap
    player_channels: BTreeMap<PlayerId, Vec<Channel>>,
}

struct Channel {
    client_id: ClientId,
    websocket: WebSocket,
}

#[derive(Debug, Clone, Copy)]
enum ClientType {
    Conductor,
    Player(PlayerId),
}

#[derive(Debug, Clone, Deserialize)]
struct ClientParams {
    latest_received_server_event: Option<u32>,
}

impl GameState {
    fn add_channel(&mut self, client_type: ClientType, channel: Channel) {
        match client_type {
            ClientType::Conductor => {
                self.conductor_channels.push(channel);
            }
            ClientType::Player(player_id) => {
                let channels = self.player_channels.entry(player_id).or_default();
                channels.push(channel);
            }
        }
    }

    fn handle_message(&mut self, channel_type: ClientType, msg: MessageEvent) {
        match channel_type {
            ClientType::Conductor => {}
            ClientType::Player(player_id) => {}
        }
    }

    fn remove_channel(&mut self, channel_type: ClientType, client_id: ClientId) {
        match channel_type {
            ClientType::Conductor => {
                self.conductor_channels
                    .retain(|channel| channel.client_id != client_id);
            }
            ClientType::Player(player_id) => {
                if let Some(channels) = self.player_channels.get_mut(&player_id) {
                    channels.retain(|channel| channel.client_id != client_id);
                }
            }
        }
    }

    fn get_or_create_player_id(&mut self, client_id: ClientId) -> Option<PlayerId> {
        todo!()
    }
}

#[durable_object]
pub struct Game {
    state: State,
    env: Env,
    game_state: Arc<Mutex<GameState>>,
}

#[durable_object]
impl DurableObject for Game {
    fn new(state: State, env: Env) -> Self {
        Self {
            state,
            env,
            game_state: Arc::new(Mutex::new(GameState {
                conductor_channels: Default::default(),
                player_channels: Default::default(),
            })),
        }
    }

    async fn fetch(&mut self, req: Request) -> worker::Result<Response> {
        fn get_client_id<T>(ctx: &RouteContext<T>) -> worker::Result<ClientId> {
            let client_id = ctx.param("client_id").unwrap();
            match Uuid::parse_str(client_id) {
                Ok(uuid) => Ok(ClientId(uuid)),
                Err(_) => Err(Error::Json(("Invalid Client ID".into(), 400))),
            }
        }
        async fn get_client_params(req: &mut Request) -> worker::Result<ClientParams> {
            req.json()
                .await
                .map_err(|_| Error::Json(("Invalid JSON of client params".into(), 400)))
        }
        Router::with_data(self.game_state.clone())
            .get_async("/play/:client_id", |mut req, ctx| async move {
                let client_id = get_client_id(&ctx)?;
                let params = get_client_params(&mut req).await?;
                if let Some(player_id) = ctx.data.lock().await.get_or_create_player_id(client_id) {
                    websocket(
                        ctx.data.clone(),
                        ClientType::Player(player_id),
                        client_id,
                        params,
                    )
                    .await
                } else {
                    Response::error("This Client ID cannot to be connected any player.", 403)
                }
            })
            .get_async("/conduct/:client_id", |mut req, ctx| async move {
                let client_id = get_client_id(&ctx)?;
                let params = get_client_params(&mut req).await?;
                websocket(
                    ctx.data.clone(),
                    ClientType::Conductor,
                    client_id,
                    params,
                )
                .await
            })
            .run(req, self.env.clone().into())
            .await
    }
}

async fn websocket(
    state: Arc<Mutex<GameState>>,
    channel_type: ClientType,
    client_id: ClientId,
    client_params: ClientParams,
) -> Result<Response> {
    let WebSocketPair { client, server } = WebSocketPair::new()?;

    state.lock().await.add_channel(
        channel_type,
        Channel {
            client_id,
            websocket: server.clone(),
        },
    );

    server.accept()?;

    wasm_bindgen_futures::spawn_local(async move {
        let mut stream = server.events().unwrap();

        while let Some(event) = stream.next().await {
            let event = event.unwrap();

            match event {
                WebsocketEvent::Message(msg) => {
                    state.lock().await.handle_message(channel_type, msg)
                }
                WebsocketEvent::Close(_) => {
                    state.lock().await.remove_channel(channel_type, client_id)
                }
            }
        }
    });

    Response::from_websocket(client)
}