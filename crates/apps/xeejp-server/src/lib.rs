pub mod game;
pub mod repository;
pub mod room_key;
mod tracing;
mod types;
pub mod ultimatum;
pub mod user;
mod utils;

use tracing::init_tracing_once;
use worker::*;
use xeejp::types::CreateRoomRequest;

use crate::{
    room_key::{RegisterRoomRequest, RoomRegistration},
    types::Start,
    utils::forward,
    utils::get,
    utils::post,
};

const GAME_OBJECT_NS: &str = "ULTIMATUM2023";
const USER_OBJECT_NS: &str = "USER";
const ROOM_KEY_OBJECT_NS: &str = "ROOM_KEY";

#[event(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    init_tracing_once();
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let cors = Cors::new()
        .with_allowed_headers(["Content-Type"])
        .with_methods([
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Options,
        ])
        .with_origins([env
            .var("ALLOWED_ORIGINS")
            .expect("ALLOWED_ORIGINS environment variable")
            .to_string()])
        .with_credentials(true);

    if req.method() == Method::Options {
        return Response::ok("")?.with_cors(&cors);
    }

    let router = Router::new();

    fn room_object(
        ctx: &RouteContext<()>,
        user_id: &str,
        room_key: &str,
    ) -> worker::Result<worker::Stub> {
        let room_key = format!("{}@{}", room_key, user_id);
        ctx.durable_object(GAME_OBJECT_NS)?
            .id_from_name(&room_key)?
            .get_stub()
    }
    fn user_object(ctx: &RouteContext<()>, user_id: &str) -> worker::Result<worker::Stub> {
        ctx.durable_object(USER_OBJECT_NS)?
            .id_from_name(user_id)?
            .get_stub()
    }
    fn room_key_object(ctx: &RouteContext<()>) -> worker::Result<worker::Stub> {
        ctx.durable_object(ROOM_KEY_OBJECT_NS)?
            .id_from_name("ALL")?
            .get_stub()
    }
    async fn get_room_object(
        ctx: &RouteContext<()>,
        room_key: &str,
    ) -> worker::Result<worker::Stub> {
        let registration: RoomRegistration = room_key_object(ctx)?
            .fetch_with_request(get(&format!("/rooms/{}", room_key))?)
            .await?
            .json()
            .await?;
        assert!(
            user_object(ctx, &registration.user_id)?
                .fetch_with_request(get(&format!("/rooms/{}", room_key))?)
                .await?
                .status_code()
                == 200
        );
        room_object(&ctx, &registration.user_id, room_key)
    }

    let res = router
        .post_async("/users/:user_id/rooms", |mut req, ctx| async move {
            // FIXME: Authenticate user

            let user_id = ctx.param("user_id").unwrap();
            let body: CreateRoomRequest = req.json().await?;

            let res = user_object(&ctx, user_id)?
                .fetch_with_request(post("/rooms", req.headers().clone(), &body)?)
                .await?;

            if res.status_code() == 201 {
                room_key_object(&ctx)?
                    .fetch_with_request(post(
                        "/rooms",
                        req.headers().clone(),
                        &RegisterRoomRequest {
                            room_key: body.room_key.clone(),
                            user_id: user_id.to_string(),
                        },
                    )?)
                    .await?;
                room_object(&ctx, user_id, &body.room_key)?
                    .fetch_with_request(post(
                        "/start",
                        req.headers().clone(),
                        &Start {
                            conductor_password: body.conductor_password,
                        },
                    )?)
                    .await
            } else {
                Ok(res)
            }
        })
        .get_async("/users/:user_id/rooms", |_req, ctx| async move {
            // FIXME: Authenticate user
            let user_id = ctx.param("user_id").unwrap();
            let res = user_object(&ctx, user_id)?
                .fetch_with_request(get("/rooms")?)
                .await?;
            Ok(res)
        })
        .post_async(
            "/users/:user_id/rooms/:room_key/players",
            |req, ctx| async move {
                let user_id = ctx.param("user_id").unwrap();
                // FIXME: Authenticate user

                let room_key = ctx.param("room_key").unwrap();
                room_object(&ctx, user_id, room_key)?
                    .fetch_with_request(forward(req, "/players")?)
                    .await
            },
        )
        .get_async(
            "/users/:user_id/rooms/:room_key/players",
            |_req, ctx| async move {
                let user_id = ctx.param("user_id").unwrap();
                // FIXME: Authenticate user

                let room_key = ctx.param("room_key").unwrap();
                room_object(&ctx, user_id, room_key)?
                    .fetch_with_request(get("/players")?)
                    .await
            },
        )
        .on_async("/rooms/:room_key/conduct", |req, ctx| async move {
            let room_key = ctx.param("room_key").unwrap();
            get_room_object(&ctx, room_key)
                .await?
                .fetch_with_request(forward(req, "/conduct")?)
                .await
        })
        .on_async("/rooms/:room_key/play", |req, ctx| async move {
            let room_key = ctx.param("room_key").unwrap();
            get_room_object(&ctx, room_key)
                .await?
                .fetch_with_request(forward(req, "/play")?)
                .await
        })
        .run(req, env)
        .await;
    res?.with_cors(&cors)
}
