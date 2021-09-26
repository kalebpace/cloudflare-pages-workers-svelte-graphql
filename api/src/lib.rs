use juniper::{http as juniper_http, http::GraphQLRequest};
use serde_json::json;
use worker::*;

mod schema;
mod utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .get("/___graphiql", |_, _| {
            Response::from_html(juniper_http::graphiql::graphiql_source("/graphql", None))
        })
        .get("/___graphql", |_, _| {
            Response::from_html(juniper_http::playground::playground_source(
                "/graphql", None,
            ))
        })
        .post_async("/graphql", |mut req, _ctx| async move {
            let schema_doc = schema::create_schema();
            let graphql_request = req.json::<GraphQLRequest>().await.unwrap();
            let result = graphql_request.execute_sync(&schema_doc, &());
            Response::from_json(&json!(&result))
        })
        .run(req, env)
        .await
}
