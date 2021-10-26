mod schema;
mod session;
mod utils;

use juniper_cf_workers::{graphiql_source, graphql_handler, playground_source};
use worker::*;

use serde_json::json;

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    utils::set_panic_hook();

    let router = Router::new();
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .get_async("/session/:name", |_, ctx| async move {
            let namespace = ctx.durable_object("SESSION")?;
            let session_name = ctx.param("name");
            let stub = namespace.id_from_name(session_name.unwrap())?.get_stub()?;
            let response = stub.fetch_with_str("/get-session").await;
            if response.is_ok() {
                let session = response?.json::<session::Session>().await?;
                match session.name().chars().count() {
                    0 => Response::error("Session does not exist!", 404),
                    _ => Response::from_json(&json!(session)),
                }
            } else {
                Response::error("There was a problem when fetching session", 500)
            }
        })
        .post_async("/session/:name", |_, ctx| async move {
            let namespace = ctx.durable_object("SESSION")?;
            let session_name = ctx.param("name");
            let name_len = session_name.unwrap().chars().count();
            if session_name.is_some() && name_len <= 256 {
                let name = session_name.unwrap().as_str();
                let stub = namespace.id_from_name(name)?.get_stub()?;
                stub.fetch_with_str(format!("/create-session/{}", name).as_str())
                    .await
            } else {
                Response::error("Session name must be blank or above 256 characters!", 503)
            }
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
