use serde::{Deserialize, Serialize};

/// A managed u32 that always remains in range: _0 <= X <= 100_
#[derive(Debug, Serialize, Deserialize)]
pub struct Stat(u32);
impl Stat {
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn new(value: u32) -> Result<Self, std::io::Error> {
        if value <= 100 {
            return Ok(Stat { 0: value });
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "value not between 0 and 100",
        ))
    }

    /// Adds the provided value, never exceeding 100.
    pub fn add(&mut self, value: u32) {
        let new_value = self.0 + value;

        if new_value <= 100 {
            self.0 = new_value;
        } else {
            self.0 = 100;
        }
    }

    /// subtracts the provided value, never wrapping around.
    pub fn subtract(&mut self, value: u32) {
        if self.0 >= value {
            self.0 -= value;
        } else {
            self.0 = 0;
        }
    }

    /// Sets the Stat to the provided value <br>
    /// returns [std::io::ErrorKind::InvalidInput] when value does not fit range: <br>
    /// _0 <= value <= 100_
    pub fn set(&mut self, value: u32) -> Result<(), std::io::Error> {
        if value <= 100 {
            self.0 = value;
            return Ok(());
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "value not between 0 and 100",
        ))
    }
}
