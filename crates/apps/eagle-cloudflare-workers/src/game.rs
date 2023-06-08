use std::sync::Arc;

use eagle_game::{prelude::Game, room::Room};
use eagle_server::{GameServer, NotifySender};
use eagle_types::{
    client::{ClientParams, User},
    events::NotifyIndex,
    ids::{ClientId, GameInstanceId},
};

use futures::{lock::Mutex, StreamExt};
use uuid::Uuid;
use worker::{WebSocket, *};

struct WebSocketConnection {
    client_id: ClientId,
    websocket: WebSocket,
}

type GameState<T: Game> = Arc<Mutex<GameServer<T, WebSocketConnection>>>;

impl NotifySender for WebSocketConnection {
    type Error = worker::Error;

    fn client_id(&self) -> ClientId {
        self.client_id
    }

    fn send<T: serde::Serialize>(
        &self,
        index: NotifyIndex,
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
                let player_id = todo!("get player_id from kv");
                websocket(ctx.data.clone(), User::Player(player_id), client_id, params).await
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
            game_server.lock().await.add_conductor_client(client_params, WebSocketConnection { client_id, websocket: server.clone() });
        }
        User::Player(player_id) => {
            game_server.lock().await.add_player_client(player_id, client_params, WebSocketConnection { client_id, websocket: server.clone() });
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
                        User::Conductor => {
                        },
                        User::Player(player_id) => {
                        }
                    }
                }
                WebsocketEvent::Close(_) => {
                    state.lock().await.remove_client(user, client_id)
                }
            }
        }
    });

    Response::from_websocket(client)
}
