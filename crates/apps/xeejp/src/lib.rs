pub mod game;

use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .on_async("/durable", |_req, ctx| async move {
            let namespace = ctx.durable_object("CHATROOM")?;
            let stub = namespace.id_from_name("A")?.get_stub()?;
            stub.fetch_with_str("/messages").await
        })
        .get("/secret", |_req, ctx| {
            Response::ok(ctx.secret("CF_API_TOKEN")?.to_string())
        })
        .get("/var", |_req, ctx| {
            Response::ok(ctx.var("BUILD_NUMBER")?.to_string())
        })
        .post_async("/kv", |_req, ctx| async move {
            let kv = ctx.kv("SOME_NAMESPACE")?;

            kv.put("key", "value")?.execute().await?;

            Response::empty()
        })
        .run(req, env).await
}
