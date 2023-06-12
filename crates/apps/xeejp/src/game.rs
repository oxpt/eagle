use std::sync::Arc;

use eagle_game::{prelude::Game, room::Room};
use eagle_server::{GameServer, NotifySender};
use eagle_types::{
    client::{ClientParams, User},
    ids::{ClientId, GameInstanceId, PlayerId},
    messages::ServerToClientMessage,
};

use futures::{lock::Mutex, StreamExt};
use uuid::Uuid;
use worker::{WebSocket, *};

struct WebSocketConnection {
    client_id: ClientId,
    websocket: WebSocket,
}

type GameState<T> = Arc<Mutex<GameServer<T, WebSocketConnection>>>;

impl NotifySender for WebSocketConnection {
    type Error = worker::Error;

    fn client_id(&self) -> ClientId {
        self.client_id
    }

    fn send<T: serde::Serialize>(
        &self,
        message: ServerToClientMessage<T>,
    ) -> std::result::Result<(), Self::Error> {
        self.websocket.send(&message)
    }

    fn close(&self) -> std::result::Result<(), Self::Error> {
        todo!()
    }

    fn client_state(&self) -> eagle_types::client::ClientState {
        todo!()
    }
}

pub struct WorkerGame<T: Game> {
    #[allow(dead_code)]
    state: State,
    env: Env,
    game_state: GameState<T>,
}

impl<T: Game> WorkerGame<T> {
    pub fn new(
        state: State,
        env: Env,
        game_instance_id: GameInstanceId,
        config: T::Config,
        rand_seed: [u8; 32],
    ) -> Self {
        let room = Room::new(game_instance_id, config, rand_seed);
        Self {
            state,
            env,
            game_state: Arc::new(Mutex::new(GameServer::new(room))),
        }
    }

    pub async fn fetch(&mut self, req: Request) -> worker::Result<Response> {
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
                let attachments = ctx.kv("PLAYER_CLIENT_ATTACHMENTS").unwrap();
                if let Some(player_id) = attachments
                    .get(&client_id.0.to_string())
                    .text()
                    .await?
                    .and_then(|s| Uuid::parse_str(&s).ok())
                {
                    websocket(
                        ctx.data.clone(),
                        User::Player(PlayerId(player_id)),
                        client_id,
                        params,
                    )
                    .await
                } else {
                    Err(Error::Json(("Player not found".into(), 400)))
                }
            })
            .get_async("/conduct/:client_id", |mut req, ctx| async move {
                let client_id = get_client_id(&ctx)?;
                let params = get_client_params(&mut req).await?;
                websocket(ctx.data.clone(), User::Conductor, client_id, params).await
            })
            .run(req, self.env.clone().into())
            .await
    }
}

async fn websocket<T: Game>(
    state: GameState<T>,
    user: User,
    client_id: ClientId,
    client_params: ClientParams,
) -> Result<Response> {
    let WebSocketPair { client, server } = WebSocketPair::new()?;

    let game_server = state.clone();

    match user {
        User::Conductor => {
            game_server.lock().await.add_conductor_client(
                client_params,
                WebSocketConnection {
                    client_id,
                    websocket: server.clone(),
                },
            );
        }
        User::Player(player_id) => {
            game_server.lock().await.add_player_client(
                player_id,
                client_params,
                WebSocketConnection {
                    client_id,
                    websocket: server.clone(),
                },
            );
        }
    }

    server.accept()?;

    wasm_bindgen_futures::spawn_local(async move {
        let mut stream = server.events().unwrap();

        while let Some(event) = stream.next().await {
            let event = event.unwrap();

            match event {
                WebsocketEvent::Message(msg) => {
                    //TODO: parse msg
                    match user {
                        User::Conductor => {}
                        User::Player(player_id) => {}
                    }
                }
                WebsocketEvent::Close(_) => state.lock().await.remove_client(user, client_id),
            }
        }
    });

    Response::from_websocket(client)
}
