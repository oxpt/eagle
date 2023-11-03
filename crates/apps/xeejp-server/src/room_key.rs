use std::collections::{hash_map::Entry, HashMap};

use serde::{Deserialize, Serialize};
use worker::*;

use crate::tracing::init_tracing_once;

#[durable_object]
pub struct RoomKey {
    #[allow(dead_code)]
    state: State,
    env: Env,
}

const ROOMS_STORAGE_KEY: &str = "ROOMS";

#[derive(Serialize, Deserialize)]
pub struct RegisterRoomRequest {
    pub room_key: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RoomRegistration {
    pub user_id: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Rooms {
    registrations: HashMap<String, RoomRegistration>,
}

#[durable_object]
impl DurableObject for RoomKey {
    fn new(state: State, env: Env) -> Self {
        console_error_panic_hook::set_once();
        init_tracing_once();
        Self { state, env }
    }

    async fn fetch(&mut self, req: Request) -> worker::Result<Response> {
        Router::with_data(&self)
            .post_async("/rooms", |mut req, ctx| async move {
                let body = req.json::<RegisterRoomRequest>().await?;
                let mut rooms: Rooms = ctx
                    .data
                    .state
                    .storage()
                    .get(ROOMS_STORAGE_KEY)
                    .await
                    .unwrap_or_default();
                if let Entry::Vacant(e) = rooms.registrations.entry(body.room_key) {
                    e.insert(RoomRegistration {
                        user_id: body.user_id,
                    });
                    ctx.data
                        .state
                        .storage()
                        .put(ROOMS_STORAGE_KEY, rooms)
                        .await?;
                    Response::ok("Room registered").map(|resp| resp.with_status(201))
                } else {
                    Response::error("Room already registered", 409)
                }
            })
            .get_async("/rooms/:room_key", |_req, ctx| async move {
                let room_key = ctx.param("room_key").unwrap();
                let rooms: Rooms = ctx
                    .data
                    .state
                    .storage()
                    .get(ROOMS_STORAGE_KEY)
                    .await
                    .unwrap_or_default();
                if let Some(registration) = rooms.registrations.get(room_key) {
                    Response::from_json(&registration)
                } else {
                    Response::error("Room not registered", 404)
                }
            })
            .run(req, self.env.clone().into())
            .await
    }
}
