pub mod game;
pub mod repository;
mod types;
pub mod ultimatum;

use uuid::Uuid;
use worker::*;
use xeejp::types::{RoomResponse, StartGameInstanceRequest};

const GAME_OBJECT_NS: &str = "GAME";
const ROOM_KEY_NS: &str = "ROOM_KEY";

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    fn get_param<T>(ctx: &RouteContext<T>, name: &str) -> worker::Result<Uuid> {
        let value = ctx.param(name).unwrap();
        value
            .parse()
            .map_err(|_| Error::Json(("Invalid UUID".into(), 400)))
    }

    fn stub(ctx: &RouteContext<()>) -> worker::Result<worker::Stub> {
        let game_instance_id = get_param(ctx, "game_instance_id")?;
        ctx.durable_object(GAME_OBJECT_NS)?
            .id_from_name(&game_instance_id.to_string())?
            .get_stub()
    }

    router
        .get_async("/rooms/:room_key", |_req, ctx| async move {
            let room_key = ctx.param("room_key").unwrap();
            let kv = ctx.kv(ROOM_KEY_NS)?;
            if let Some(game_instance_id) = kv.get(room_key).text().await? {
                Ok(Response::from_json(&RoomResponse { game_instance_id })?)
            } else {
                Err(Error::Json(("Room not found".into(), 404)))
            }
        })
        .post_async(
            "/games/:game_instance_id/start",
            |mut req, ctx| async move {
                let game_instance_id = get_param(&ctx, "game_instance_id")?;
                let body: StartGameInstanceRequest = req.json().await?;

                // Check and store room key
                // FIXME: This is not atomic
                let kv = ctx.kv(ROOM_KEY_NS)?;
                if kv.get(&body.room_key).text().await?.is_some() {
                    return Err(Error::Json(("Room key already exists".into(), 409)));
                }
                kv.put(&body.room_key, game_instance_id.to_string())?
                    .execute()
                    .await
                    .expect("kv.put room key");

                // Start game instance
                stub(&ctx)?.fetch_with_request(req).await
            },
        )
        .post_async("/games/:game_instance_id/players", |req, ctx| async move {
            stub(&ctx)?.fetch_with_request(req).await
        })
        .get_async("/games/:game_instance_id/players", |req, ctx| async move {
            stub(&ctx)?.fetch_with_request(req).await
        })
        .on_async("/games/:game_instance_id/conduct", |req, ctx| async move {
            stub(&ctx)?.fetch_with_request(req).await
        })
        .on_async("/games/:game_instance_id/play", |req, ctx| async move {
            stub(&ctx)?.fetch_with_request(req).await
        })
        .run(req, env)
        .await
}
