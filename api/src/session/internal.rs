use super::song::Song;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Internal {
    name: String,
    queue: Vec<Song>,
    current_play_time: u32,
}

impl Internal {
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

    pub fn get_item(&self, index: usize) -> &Song {
        &self.queue[index]
    }

    pub fn add_song_to_queue(&mut self, song: Song) {
        self.queue.push(song)
    }

    pub fn remove_song_by_id(&mut self, video_id: &str) -> Option<Song> {
        let index = self.queue.iter().position(|x| x.video_id() == video_id);
        match index {
            Some(index) => Some(self.queue.remove(index)),
            _ => None,
        }
    }

    pub fn upvote_song_by_id(&mut self, video_id: &str) -> Result<(), String> {
        let index = self.queue.iter().position(|x| x.video_id() == video_id);
        match index {
            Some(index) => Ok(self.queue[index].upvote()),
            _ => Err("Item not found".to_string()),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use song::Song;

    #[test]
    fn test_add_song_to_queue() {
        let mut internal = Internal::new("test-session");
        let new_song = Song::new("njGpfHhUOE8");

        internal.add_song_to_queue(new_song.clone());

        assert_eq!(internal.get_item(0).video_id(), new_song.video_id());
    }

    #[test]
    fn test_upvote_song_by_id() {
        let mut session = Internal::new("test-session");
        let song_id = "njGpfHhUOE8";
        let new_song = Song::new(song_id);

        session.add_song_to_queue(new_song.clone());
        let _ = session.upvote_song_by_id(song_id);

        assert_eq!(session.get_item(0).votes(), 1)
    }
}
