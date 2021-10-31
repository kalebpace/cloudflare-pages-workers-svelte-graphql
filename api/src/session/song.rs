use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject)]
#[graphql(description = "A song which maps to a given youtube video id")]
pub struct Song {
    video_id: String,
    votes: i32,
    length: i32,
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

    pub fn votes(&self) -> i32 {
        self.votes
    }

    pub fn upvote(&mut self) {
        self.votes += 1;
    }
}
