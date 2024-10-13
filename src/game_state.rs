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
    current_time: String,
}

impl GameState {
    pub fn store_to_file(&mut self) -> std::io::Result<()> {
        self.update();
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
        self.current_time = format!("{}", Utc::now());
        self.friend.update_state();
    }
}
