use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::*;

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Song {
    url: String,
    votes: u32,
    added_by: u32,
    length: u32,
}

#[durable_object]
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    name: String,
    queue: Vec<Song>,
    current_play_time: u32,
    // state: State,
    // env: Env,
}

impl Session {
    pub fn name(self) -> String {
        self.name
    }

    fn create_session(&mut self, path_segments: &Vec<&str>) -> Result<Response> {
        self.name = String::from(path_segments[1]);
        Response::ok(format!("Session created: {}", self.name.as_str()))
    }
}

#[durable_object]
impl DurableObject for Session {
    fn new(state: State, _env: Env) -> Self {
        Self {
            name: String::from(""),
            queue: vec![],
            current_play_time: 0,
        }
    }

    async fn fetch(&mut self, _req: Request) -> Result<Response> {
        let url = _req.url()?;
        let path_segments = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();
        match path_segments[0] {
            "/" => Response::ok(&format!("Your session name is: {}", self.name)),
            "create-session" => self.create_session(&path_segments),
            "get-session" => Response::from_json(&json!(self)),
            _ => Response::ok(format!("nop")),
        }
    }
}
