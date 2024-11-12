use super::movement::{Movement, Location};
use chrono::Utc;

#[derive(Debug, Clone, PartialEq)]
pub struct DvdBounceMovement {
    location: Location,
    x_direction_toggle: bool,
    y_direction_toggle: bool,
}
impl DvdBounceMovement {
    pub fn new(location: Location) -> Self {
        Self {
            location,
            x_direction_toggle: true,
            y_direction_toggle: true,
        }
    }

    fn update_state(&mut self) { // TODO: Make based on time.
        if self.location.x <= 0 || self.location.x >= 40 {
            self.x_direction_toggle = !self.x_direction_toggle;
        }
        if self.location.y <= 0 || self.location.y >= 40 {
            self.y_direction_toggle = !self.y_direction_toggle;
        }
        
        let new_x = if self.x_direction_toggle {
            self.location.x + 1
        } else {
            self.location.x - 1 
        };
        let new_y = if self.y_direction_toggle {
            self.location.y + 1
        } else {
            self.location.y - 1
        };
        
        self.location = Location::new(new_x, new_y);
    }
}

impl Movement for DvdBounceMovement {
    fn next_position(&mut self) -> Location {
        self.update_state();
        self.location
    }
}