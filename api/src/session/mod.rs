pub mod internal;
pub mod song;

use internal::Internal;
use serde_json::json;
use worker::*;

#[durable_object]
pub struct Session {
    internal: Internal,
    state: State,
    env: Env,
}

#[durable_object]
impl DurableObject for Session {
    fn new(state: State, env: Env) -> Self {
        Self {
            internal: Internal::new(&state.id().to_string()),
            state: state,
            env: env,
        }
    }

    async fn fetch(&mut self, _req: Request) -> Result<Response> {
        let url = _req.url()?;
        let path_segments = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();
        match path_segments[0] {
            "get-session" => Response::from_json(&json!(self.internal)),
            _ => Response::error("Resource not found", 404),
        }
    }
}
