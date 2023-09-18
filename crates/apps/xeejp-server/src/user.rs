use std::collections::{hash_map::Entry, HashMap};

use worker::*;
use xeejp::types::{CreateRoomRequest, Room, Rooms, RoomsResponse};

#[durable_object]
pub struct User {
    #[allow(dead_code)]
    state: State,
    env: Env,
}

const ROOMS_KV_KEY: &str = "ROOMS";

#[durable_object]
impl DurableObject for User {
    fn new(state: State, env: Env) -> Self {
        Self { state, env }
    }

    async fn fetch(&mut self, req: Request) -> worker::Result<Response> {
        Router::with_data(&self)
            .post_async("/rooms", |mut req, ctx| async move {
                console_log!("POST /rooms");
                let body = req.json::<CreateRoomRequest>().await?;
                let mut rooms: Rooms = ctx
                    .data
                    .state
                    .storage()
                    .get(ROOMS_KV_KEY)
                    .await
                    .unwrap_or_else(|_| Rooms(HashMap::new()));
                if let Entry::Vacant(e) = rooms.0.entry(body.room_key) {
                    e.insert(Room {
                        name: body.room_name,
                    });
                    ctx.data.state.storage().put(ROOMS_KV_KEY, rooms).await?;
                    Response::ok("Room created").map(|resp| resp.with_status(201))
                } else {
                    Response::error("Room already exists", 409)
                }
            })
            .get_async("/rooms", |_req, ctx| async move {
                console_log!("GET /rooms");
                let rooms: Rooms = ctx
                    .data
                    .state
                    .storage()
                    .get(ROOMS_KV_KEY)
                    .await
                    .unwrap_or_else(|_| Rooms(HashMap::new()));
                let response = RoomsResponse { rooms };
                Response::ok(serde_json::to_string(&response).unwrap())
            })
            .get_async("/rooms/:room_key", |_req, ctx| async move {
                let room_key = ctx.param("room_key").unwrap();
                let rooms: Rooms = ctx.data.state.storage().get(ROOMS_KV_KEY).await?;
                if rooms.0.contains_key(room_key) {
                    Response::ok("Room exists")
                } else {
                    Response::error("Room not found", 404)
                }
            })
            .run(req, self.env.clone().into())
            .await
    }
}
