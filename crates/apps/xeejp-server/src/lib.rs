pub mod game;
pub mod repository;
mod types;
pub mod ultimatum;
pub mod user;

use types::CreateRoom;
use worker::{wasm_bindgen::JsValue, *};
use xeejp::types::CreateRoomRequest;

use crate::types::Start;

const GAME_OBJECT_NS: &str = "ULTIMATUM2023";
const USER_OBJECT_NS: &str = "USER";

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
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
    async fn assert_room_exists(
        ctx: &RouteContext<()>,
        user_id: &str,
        room_key: &str,
    ) -> worker::Result<bool> {
        Ok(user_object(ctx, user_id)?
            .fetch_with_request(Request::new_with_init(
                format!("/rooms/{}", room_key).as_str(),
                &RequestInit {
                    method: Method::Get,
                    ..Default::default()
                },
            )?)
            .await?
            .status_code()
            == 200)
    }
    fn inner_body(req: Request) -> Option<JsValue> {
        req.inner().body().map(|b| b.into())
    }
    fn websocket_request(req: Request, path: &str) -> worker::Result<Request> {
        Request::new_with_init(
            path,
            &RequestInit {
                method: req.method(),
                headers: req.headers().clone(),
                body: inner_body(req),
                ..Default::default()
            },
        )
    }

    router
        .post_async("/users/:user_id/rooms", |mut req, ctx| async move {
            // FIXME: Authenticate user

            let user_id = ctx.param("user_id").unwrap();
            let body: CreateRoomRequest = req.json().await?;

            let res = user_object(&ctx, user_id)?
                .fetch_with_request(Request::new_with_init(
                    "/rooms",
                    &RequestInit {
                        body: Some(
                            serde_wasm_bindgen::to_value(&CreateRoom {
                                room_key: body.room_key.clone(),
                            })
                            .unwrap(),
                        ),
                        method: Method::Post,
                        ..Default::default()
                    },
                )?)
                .await?;

            if res.status_code() == 201 {
                room_object(&ctx, user_id, &body.room_key)?
                    .fetch_with_request(Request::new_with_init(
                        "/start",
                        &RequestInit {
                            body: Some(
                                serde_wasm_bindgen::to_value(&Start {
                                    conductor_password: body.conductor_password,
                                })
                                .unwrap(),
                            ),
                            method: Method::Post,
                            ..Default::default()
                        },
                    )?)
                    .await
            } else {
                Ok(res)
            }
        })
        .post_async(
            "/users/:user_id/rooms/:room_key/players",
            |req, ctx| async move {
                let user_id = ctx.param("user_id").unwrap();
                // FIXME: Authenticate user

                let room_key = ctx.param("room_key").unwrap();
                room_object(&ctx, user_id, room_key)?
                    .fetch_with_request(Request::new_with_init(
                        "/players",
                        &RequestInit {
                            method: Method::Post,
                            body: inner_body(req),
                            ..Default::default()
                        },
                    )?)
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
                    .fetch_with_request(Request::new_with_init(
                        "/players",
                        &RequestInit {
                            method: Method::Get,
                            ..Default::default()
                        },
                    )?)
                    .await
            },
        )
        .on_async(
            "/users/:user_id/rooms/:room_key/conduct",
            |req, ctx| async move {
                let user_id = ctx.param("user_id").unwrap();
                let room_key = ctx.param("room_key").unwrap();
                if !assert_room_exists(&ctx, user_id, room_key).await? {
                    return Response::error("Room not found", 404);
                }
                room_object(&ctx, user_id, room_key)?
                    .fetch_with_request(websocket_request(req, "/conduct")?)
                    .await
            },
        )
        .on_async(
            "/users/:user_id/rooms/:room_key/play",
            |req, ctx| async move {
                let user_id = ctx.param("user_id").unwrap();
                let room_key = ctx.param("room_key").unwrap();
                if !assert_room_exists(&ctx, user_id, room_key).await? {
                    return Response::error("Room not found", 404);
                }

                room_object(&ctx, user_id, room_key)?
                    .fetch_with_request(websocket_request(req, "/play")?)
                    .await
            },
        )
        .run(req, env)
        .await
}
