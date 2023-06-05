use std::sync::Arc;

use eagle_game::Room;
use eagle_server::{Channel, Clients, GameServer};
use eagle_types::{
    client::{ClientParams, User},
    events::ServerEventIndex,
    ids::{ClientId, GameInstanceId},
};

use futures::{lock::Mutex, StreamExt};
use uuid::Uuid;
use worker::{WebSocket, *};

struct WebSocketConnection {
    client_id: ClientId,
    websocket: WebSocket,
}

type GameState = Arc<Mutex<GameServer<WebSocketConnection>>>;

impl Channel for WebSocketConnection {
    type Error = worker::Error;

    fn client_id(&self) -> ClientId {
        self.client_id
    }

    fn send<T: serde::Serialize>(
        &self,
        game_instance_id: GameInstanceId,
        index: ServerEventIndex,
        event: T,
    ) -> std::result::Result<(), Self::Error> {
        todo!()
    }

    fn close(&self) -> std::result::Result<(), Self::Error> {
        todo!()
    }

    fn client_state(&self) -> eagle_types::client::ClientState {
        todo!()
    }
}

pub struct WorkerGame<T: Game> {
    state: State,
    env: Env,
    game_state: Arc<Mutex<GameState>>,
}

impl <T: Game> WorkerGame<T> {
    pub fn new(state: State, env: Env) -> Self {
        let room = Room::new(todo!(), todo!(), todo!());
        Self {
            state,
            env,
            game_state: Arc::new(Mutex::new(GameServer::new(room))),
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
                    websocket(ctx.data.clone(), User::Player(player_id), client_id, params).await
                } else {
                    Response::error("This Client ID cannot to be connected any player.", 403)
                }
            })
            .get_async("/conduct/:client_id", |mut req, ctx| async move {
                let client_id = get_client_id(&ctx)?;
                let params = get_client_params(&mut req).await?;
                websocket(ctx.data.clone(), ClientType::Conductor, client_id, params).await
            })
            .run(req, self.env.clone().into())
            .await
    }
}

async fn websocket(
    state: GameState,
    user: User,
    client_id: ClientId,
    client_params: ClientParams,
) -> Result<Response> {
    let WebSocketPair { client, server } = WebSocketPair::new()?;

    let game_server = state.lock().await;

    match user {
        User::Conductor => {
            game_server.lock
    game_server.add_conductor_client(
        Channel {
            client_id,
            websocket: server.clone(),
        },
    );
        },
            User::Player(player_id) =>{
            },
    }

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
