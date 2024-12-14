use super::movement::Movement;
use chrono::Utc;
use crate::utils::location::Location;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EggHopMovement {
    start_location: Location,
    is_grounded: bool,
    last_update: i64,
}

impl EggHopMovement {
    pub fn new(start_location: Location) -> Self {
        Self {
            start_location,
            is_grounded: true,
            last_update: Utc::now().timestamp_millis(),
        }
    }
    
    fn try_update_state(&mut self) {
        let now = Utc::now().timestamp_millis();
        if now - self.last_update > 500 { // half a second
            self.is_grounded = !self.is_grounded;
            self.last_update = now;
        }
    }
}

impl Movement for EggHopMovement {
    fn next_position(&mut self) -> Location {
        self.try_update_state();
        
        if self.is_grounded {
            self.start_location
        } else {
            let mut location = self.start_location;
            location.y += 1;
            location
        }
    }
}
