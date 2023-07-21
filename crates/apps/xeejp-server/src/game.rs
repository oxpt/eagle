use std::{ops::DerefMut, sync::Arc};

use eagle_game::{prelude::Game, room::Room};
use eagle_server::{channel::Channel, server::GameServer};
use eagle_types::{
    client::User,
    ids::{ClientId, GameInstanceId, PlayerId},
    messages::{ClientToServerMessage, ServerToClientMessage},
};

use futures::{lock::Mutex, StreamExt};
use uuid::Uuid;
use worker::{WebSocket, *};

use crate::repository::GameLog;

struct WebSocketConnection {
    websocket: WebSocket,
}

pub struct Data<T: Game> {
    server: GameServer<T, WebSocketConnection>,
    state: GameLog<T>,
}

type GameState<T> = Arc<Mutex<Data<T>>>;

impl Channel for WebSocketConnection {
    type Error = worker::Error;

    fn send_message<T: serde::Serialize>(&self, view: ServerToClientMessage<T>) -> Result<()> {
        self.websocket.send(&view)
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
    pub fn new(state: State, env: Env) -> Self {
        // TODO: Load game state if log exists
        let mut seed = [0; 32];
        getrandom::getrandom(&mut seed).unwrap();
        let room = Room::new(GameInstanceId::gen(), T::Config::default(), seed);
        let data = Data {
            server: GameServer::new(room),
            state: GameLog::new(T::Config::default(), seed),
        };
        Self {
            state,
            env,
            game_state: Arc::new(Mutex::new(data)),
        }
    }

    pub async fn fetch(&mut self, req: Request) -> worker::Result<Response> {
        fn get_param<T>(ctx: &RouteContext<T>, name: &str) -> worker::Result<Uuid> {
            let value = ctx.param(name).unwrap();
            Uuid::parse_str(value).map_err(|_| Error::Json(("Invalid UUID".into(), 400)))
        }
        Router::with_data(self.game_state.clone())
            // .post_async("/games/:game_instance_id/start", |_req, _ctx| async move {
            //     Response::ok("Game started")
            // })
            .on_async(
                "/games/:game_instance_id/clients/:client_id/play/:player_id",
                |_req, ctx| async move {
                    let client_id = ClientId(get_param(&ctx, "client_id")?);
                    let player_id = PlayerId(get_param(&ctx, "player_id")?);
                    websocket(ctx.data.clone(), User::Player(player_id), client_id).await
                },
            )
            .on_async(
                "/games/:game_instance_id/clients/:client_id/conduct",
                |_req, ctx| async move {
                    let client_id = ClientId(get_param(&ctx, "client_id")?);
                    websocket(ctx.data.clone(), User::Conductor, client_id).await
                },
            )
            .run(req, self.env.clone().into())
            .await
    }
}

async fn websocket<T: Game>(
    state: GameState<T>,
    user: User,
    client_id: ClientId,
) -> Result<Response> {
    let WebSocketPair { client, server } = WebSocketPair::new()?;

    let game_server = state.clone();

    match user {
        User::Conductor => {
            game_server.lock().await.server.add_conductor_client(
                client_id,
                WebSocketConnection {
                    websocket: server.clone(),
                },
            );
        }
        User::Player(player_id) => {
            game_server.lock().await.server.add_player_client(
                player_id,
                client_id,
                WebSocketConnection {
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
                WebsocketEvent::Message(msg) => match user {
                    User::Conductor => {
                        let mut data = state.lock().await;
                        let conductor_command = msg
                            .json::<ClientToServerMessage<T::ConductorCommand>>()
                            .unwrap();
                        let data = data.deref_mut();
                        data.server.handle_conductor_command(
                            &mut data.state,
                            client_id,
                            conductor_command,
                        );
                    }
                    User::Player(player_id) => {
                        let mut data = state.lock().await;
                        let player_command = msg
                            .json::<ClientToServerMessage<T::PlayerCommand>>()
                            .unwrap();
                        let data = data.deref_mut();
                        data.server.handle_player_command(
                            &mut data.state,
                            client_id,
                            player_id,
                            player_command,
                        );
                    }
                },
                WebsocketEvent::Close(_) => {
                    state.lock().await.server.remove_client(user, client_id)
                }
            }
        }
    });

    Response::from_websocket(client)
}
