use std::{collections::HashMap, ops::DerefMut, sync::Arc};

use argon2::Argon2;
use eagle_game::{prelude::Game, room::Room};
use eagle_server::{channel::Channel, server::GameServer};
use eagle_types::{
    client::User,
    ids::{ClientId, GameInstanceId, PlayerId},
    messages::{ClientToServerMessage, ServerToClientMessage},
};

use futures::{lock::Mutex, StreamExt};
use password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString};
use uuid::Uuid;
use worker::{WebSocket, *};
use xeejp::types::{
    AddPlayerRequest, ConductRequest, PlayRequest, PlayerResponse, PlayersResponse,
    StartGameInstanceRequest,
};

use crate::repository::GameLog;

struct WebSocketConnection {
    websocket: WebSocket,
}

pub struct Data<T: Game> {
    state: State,
    server: GameServer<T, WebSocketConnection>,
    log: GameLog<T>,
    players: HashMap<String, Player>,
}

pub struct Player {
    player_uuid: PlayerId,
    password_hash: String,
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
    env: Env,
    game_state: GameState<T>,
}

const GAME_LOG_STORAGE_KY: &str = "GAME_LOG";
const CONDUCTOR_HASH_KV_KEY: &str = "CONDUCTOR_HASH";

pub fn hash_password(password: &str) -> String {
    let algon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hash = algon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Hashing failed");
    hash.serialize().as_str().to_string()
}

pub fn verify_password(password: &str, hash: &str) {
    let password_hash = password_hash::PasswordHash::new(hash).expect("invalid password hash");
    let algs: &[&dyn PasswordVerifier] = &[&Argon2::default()];
    password_hash
        .verify_password(algs, password)
        .expect("invalid password");
}

impl<T: Game> WorkerGame<T> {
    pub fn new(state: State, env: Env) -> Self {
        // TODO: Load game state and players if log exists
        let mut seed = [0; 32];
        getrandom::getrandom(&mut seed).unwrap();
        let game_instance_id = GameInstanceId::gen(); // It's not same as
        let room = Room::new(game_instance_id, T::Config::default(), seed);
        let data = Data {
            state,
            server: GameServer::new(room),
            log: GameLog::new(T::Config::default(), seed),
            players: HashMap::new(),
        };
        Self {
            env,
            game_state: Arc::new(Mutex::new(data)),
        }
    }

    pub async fn fetch(&mut self, req: Request) -> worker::Result<Response> {
        Router::with_data(self.game_state.clone())
            .post_async(
                "/games/:game_instance_id/start",
                |mut req, ctx| async move {
                    let body: StartGameInstanceRequest = req.json().await?;
                    let data = ctx.data.lock().await;
                    data.state
                        .storage()
                        .put(
                            CONDUCTOR_HASH_KV_KEY,
                            hash_password(&body.conductor_password),
                        )
                        .await?;
                    Response::ok("Game started")
                },
            )
            .post_async(
                "/games/:game_instance_id/players",
                |mut req, ctx| async move {
                    let body: AddPlayerRequest = req.json().await?;
                    let mut data = ctx.data.lock().await;
                    let algon2 = Argon2::default();
                    let salt = SaltString::generate(&mut OsRng);
                    let hash = algon2
                        .hash_password(body.player_password.as_bytes(), &salt)
                        .expect("Hashing failed");
                    data.players.insert(
                        body.player_id,
                        Player {
                            player_uuid: PlayerId(Uuid::parse_str(&body.player_uuid).unwrap()),
                            password_hash: hash.serialize().as_str().to_string(),
                        },
                    );
                    // TODO: Save player to storage
                    Response::ok("Player added")
                },
            )
            .get_async("/games/:game_instance_id/players", |_req, ctx| async move {
                let data = ctx.data.lock().await;
                let players = data
                    .players
                    .iter()
                    .map(|(k, v)| PlayerResponse {
                        player_id: k.clone(),
                        player_uuid: v.player_uuid.0.to_string(),
                    })
                    .collect();
                Response::from_json(&PlayersResponse { players })
            })
            .on_async("/games/:game_instance_id/play", |mut req, ctx| async move {
                let body: PlayRequest = req.json().await?;
                let client_id = ClientId::gen();
                let data = ctx.data.lock().await;
                let player = data.players.get(&body.player_id).unwrap();
                let player_id = player.player_uuid;
                verify_password(&body.player_password, &player.password_hash);
                websocket(ctx.data.clone(), User::Player(player_id), client_id).await
            })
            .on_async(
                "/games/:game_instance_id/conduct",
                |mut req, ctx| async move {
                    let body: ConductRequest = req.json().await?;
                    let client_id = ClientId::gen();
                    let data = ctx.data.lock().await;
                    let conductor_hash = data
                        .state
                        .storage()
                        .get::<String>(CONDUCTOR_HASH_KV_KEY)
                        .await?;
                    verify_password(&body.conductor_password, &conductor_hash);
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
                            &mut data.log,
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
                            &mut data.log,
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
