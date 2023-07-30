use std::collections::{hash_map::Entry, HashMap};

use serde::{Deserialize, Serialize};
use worker::*;

use crate::types::CreateRoom;

#[durable_object]
pub struct User {
    #[allow(dead_code)]
    state: State,
    env: Env,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Rooms(HashMap<String, Room>);

#[derive(Serialize, Deserialize)]
pub struct Room {}

const ROOMS_KV_KEY: &str = "ROOMS";

#[durable_object]
impl DurableObject for User {
    fn new(state: State, env: Env) -> Self {
        Self { state, env }
    }

    async fn fetch(&mut self, req: Request) -> worker::Result<Response> {
        Router::with_data(&self)
            .post_async("/rooms", |mut req, ctx| async move {
                let body = req.json::<CreateRoom>().await?;
                let mut rooms: Rooms = ctx.data.state.storage().get(ROOMS_KV_KEY).await?;
                if let Entry::Vacant(e) = rooms.0.entry(body.room_key) {
                    e.insert(Room {});
                    ctx.data.state.storage().put(ROOMS_KV_KEY, rooms).await?;
                    Response::ok("Room created").map(|resp| resp.with_status(201))
                } else {
                    Response::error("Room already exists", 409)
                }
            })
            .get_async("/rooms/:room_key", |_req, ctx| async move {
                let room_key = ctx.param("room_key").unwrap();
                let rooms: Rooms = ctx.data.state.storage().get(ROOMS_KV_KEY).await?;
                if let Some(Room {}) = rooms.0.get(room_key) {
                    Response::ok("Room exists")
                } else {
                    Response::error("Room not found", 404)
                }
            })
            .run(req, self.env.clone().into())
            .await
    }
}
