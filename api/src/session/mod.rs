use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    video_id: String,
    votes: u32,
    length: u32,
}

impl Song {
    pub fn new(video_id: &str) -> Self {
        Self {
            video_id: String::from(video_id),
            votes: 0,
            length: 0,
        }
    }

    pub fn upvote(&mut self) {
        self.votes += 1;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    name: String,
    queue: Vec<Song>,
    current_play_time: u32,
}

impl Session {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            queue: vec![],
            current_play_time: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_song_to_queue(&mut self, song: Song) {
        self.queue.push(song)
    }

    pub fn remove_song_by_id(&mut self, video_id: &str) -> Option<Song> {
        let index = self.queue.iter().position(|x| x.video_id == video_id);
        match index {
            Some(index) => Some(self.queue.remove(index)),
            _ => None,
        }
    }

    pub fn upvote_song_by_id(&mut self, video_id: &str) -> Result<()> {
        let index = self.queue.iter().position(|x| x.video_id == video_id);
        match index {
            Some(index) => Ok(self.queue[index].upvote()),
            _ => Err(Error::RustError("Item not found".to_string())),
        }
    }
}

#[durable_object]
pub struct DurableObject {
    session: Session,
    state: State,
    env: Env,
}

#[durable_object]
impl DurableObject for DurableObject {
    fn new(state: State, env: Env) -> Self {
        Self {
            session: Session::new(&state.id().to_string()),
            state: state,
            env: env,
        }
    }

    async fn fetch(&mut self, _req: Request) -> Result<Response> {
        let url = _req.url()?;
        let path_segments = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();
        match path_segments[0] {
            "/" => Response::ok(&format!("Your session name is: {}", self.session.name)),
            "get-session" => Response::from_json(&json!(self.session)),
            _ => Response::error("Resource not found", 404),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_add_song_to_queue() {
        let mut session = Session::new("test-session");
        let new_song = Song::new("njGpfHhUOE8");

        session.add_song_to_queue(new_song.clone());

        assert_eq!(session.queue[0].video_id, new_song.video_id);
    }

    #[test]
    fn test_upvote_song_by_id() {
        let mut session = Session::new("test-session");
        let song_id = "njGpfHhUOE8";
        let new_song = Song::new(song_id);

        session.add_song_to_queue(new_song.clone());
        let result = session.upvote_song_by_id(song_id);

        assert_eq!(session.queue[0].votes, 1)
    }
}
