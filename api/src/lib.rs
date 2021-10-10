use juniper_cf_workers::{graphiql_source, graphql_handler, playground_source};
use worker::*;

mod schema;
mod utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    utils::set_panic_hook();

    let router = Router::new();
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
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
