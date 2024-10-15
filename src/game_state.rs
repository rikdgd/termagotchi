use crate::friend::Friend;
use crate::shapes::creatures::CreatureShapes;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};

const SAVE_FILE_PATH: &str = "./save-file.txt";

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    friend: Friend<CreatureShapes>,
    last_update_time: i64,
}

impl GameState {
    pub fn new(friend: Friend<CreatureShapes>) -> Self {
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

    pub fn file_exists() -> bool {
        std::path::Path::new(SAVE_FILE_PATH).is_file()
    }

    pub fn update(&mut self) {
        self.last_update_time = Utc::now().timestamp_millis();
        self.friend.update_state();
    }

    pub fn friend(&self) -> &Friend<CreatureShapes> {
        &self.friend
    }
}
