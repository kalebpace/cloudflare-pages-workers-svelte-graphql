mod schema;
mod session;
mod utils;

use juniper_cf_workers::{graphiql_source, graphql_handler, playground_source};
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    utils::set_panic_hook();

    let router = Router::new();
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .get_async("/session", |_, ctx| async move {
            let namespace = ctx.durable_object("SESSION")?;
            let stub = namespace.id_from_name("A")?.get_stub()?;
            stub.fetch_with_str("/").await
        })
        .get("/___graphiql", |_, _| graphiql_source("/graphql", None))
        .get("/___graphql", |_, _| playground_source("/graphql", None))
        .get_async("/graphql", |mut req, _| async move {
            graphql_handler(&schema::create_schema(), &(), &mut req).await
        })
        .post_async("/graphql", |mut req, _| async move {
            graphql_handler(&schema::create_schema(), &(), &mut req).await
        })
        .run(req, env)
        .await
}
