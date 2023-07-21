pub mod game;
pub mod repository;
pub mod ultimatum;

use futures::join;
use uuid::Uuid;
use worker::*;
use xeejp::types::CreateGameInstanceRequest;

const KV_NS_GAMES: &str = "GAMES";

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    fn get_param<T>(ctx: &RouteContext<T>, name: &str) -> worker::Result<Uuid> {
        let value = ctx.param(name).unwrap();
        value
            .parse()
            .map_err(|_| Error::Json(("Invalid UUID".into(), 400)))
    }

    router
        .post_async(
            "/games/:game_instance_id/clients/:client_id/start",
            |mut req, ctx| async move {
                let game_instance_id = get_param(&ctx, "game_instance_id")?;
                let client_id = get_param(&ctx, "client_id")?;
                let body: CreateGameInstanceRequest = req.json().await?;
                let (room_key_result,  = join!(
                    ctx.kv(KV_NS_ROOM_KEY)
                        .expect("room key namespace")
                        .put(&body.room_key, game_instance_id)?
                        .execute(),
                    ctx.kv(KV_NS_CONDUCTOR_CLIENT_IDS)
                        .expect("conductor client ids namespace")
                        .put(&client_id.to_string(), game_instance_id)?
                        .execute()
                );
                room_key_result.expect("put room key");
                Response::ok("")
            },
        )
        .on_async(
            "/games/:game_instance_id/clients/:client_id/conduct",
            |req, ctx| async move {
                let client_id = get_param(&ctx, "client_id")?;
                let game_instance_id = get_param(&ctx, "game_instance_id")?;
                // TODO: authenticate the conductor
                let attached_game_instance = ctx
                    .kv(KV_NS_CONDUCTOR_CLIENT_IDS)?
                    .get(&client_id.to_string())
                    .text()
                    .await?
                    .ok_or_else(|| {
                        Error::Json(("Game instance is not attached to the client".into(), 400))
                    })?;
                if game_instance_id.to_string() != attached_game_instance {
                    return Err(Error::Json((
                        "Game instance is not attached to the client".into(),
                        400,
                    )));
                }
                let namespace = ctx.durable_object("Ultimatum")?;
                let stub = namespace
                    .id_from_name(&attached_game_instance)?
                    .get_stub()?;
                stub.fetch_with_request(req).await
            },
        )
        .on_async(
            "/games/:game_instance_id/clients/:client_id/play/:player_id",
            |req, ctx| async move {
                let game_instance_id = get_param(&ctx, "game_instance_id")?;
                // TODO: authenticate the player
                let namespace = ctx.durable_object("Ultimatum")?;
                let stub = namespace
                    .id_from_name(&game_instance_id.to_string())?
                    .get_stub()?;
                stub.fetch_with_request(req).await
            },
        )
        .run(req, env)
        .await
}
