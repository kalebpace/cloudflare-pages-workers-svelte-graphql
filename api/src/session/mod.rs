use worker::*;

#[durable_object]
pub struct Session {
    name: String,
    state: State,
    env: Env,
}

#[durable_object]
impl DurableObject for Session {
    fn new(state: State, env: Env) -> Self {
        Self {
            name: String::from("test session"),
            state: state,
            env,
        }
    }

    async fn fetch(&mut self, _req: Request) -> Result<Response> {
        Response::ok(&format!("Your session name is: {}", self.name))
    }
}
