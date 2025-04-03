use crate::friend::Friend;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};

const SAVE_FILE_PATH: &str = "./save-file.txt";

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    friend: Friend,
    last_update_time: i64,
}

impl GameState {
    pub fn new(friend: Friend) -> Self {
        Self {
            friend,
            last_update_time: Utc::now().timestamp_millis(),
        }
    }

    pub fn store_to_file(&mut self) -> std::io::Result<()> {
        self.update(); // Update so we store the latest changes.
        let serialized = serde_json::to_string(&self)?;

        let mut save_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(SAVE_FILE_PATH)?;

        save_file.set_len(0)?;
        save_file.write_all(serialized.as_bytes())?;
        save_file.flush()?;

        Ok(())
    }

    pub fn read_from_file() -> std::io::Result<Self> {
        let mut save_file = OpenOptions::new().read(true).open(SAVE_FILE_PATH)?;

        let mut save_buffer = String::new();
        save_file.read_to_string(&mut save_buffer)?;
        save_file.flush()?;

        let state: Self = serde_json::from_str(&save_buffer)?;
        Ok(state)
    }

    pub fn update(&mut self) {
        let now =Utc::now().timestamp_millis();
        self.last_update_time = now;
        self.friend.update_state(now);
    }
    
    pub fn friend(&self) -> &Friend {
        &self.friend
    }

    pub fn friend_mut(&mut self) -> &mut Friend {
        &mut self.friend
    }
}
