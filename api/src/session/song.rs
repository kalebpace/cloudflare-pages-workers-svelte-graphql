use serde::{Deserialize, Serialize};

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

    pub fn video_id(&self) -> &str {
        &self.video_id
    }

    pub fn votes(&self) -> u32 {
        self.votes
    }

    pub fn upvote(&mut self) {
        self.votes += 1;
    }
}
