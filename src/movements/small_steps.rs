use chrono::Utc;
use crate::movements::Movement;
use crate::utils::location::Location;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SmallStepsMovement {
    initial_location: Location,
    x_move: i32,
    y_move: i32,
    x_toggle: bool,
    y_toggle: bool,
    last_update: i64,
}

impl SmallStepsMovement {
    pub fn new(initial_location: Location) -> Self {
        let updated_location = Location {
            x: initial_location.x - 7, // 7 for sprite dimensions (15x15)
            y: initial_location.y - 7,
        };
        
        Self {
            initial_location: updated_location,
            x_move: 0,
            y_move: 0,
            x_toggle: true,
            y_toggle: true,
            last_update: Utc::now().timestamp_millis(),
        }
    }
    
    fn update_state(&mut self) {
        let now = Utc::now().timestamp_millis();
        if now - self.last_update > 500 { // half a second
            self.last_update = now;
            
            if self.x_move == 10 || self.x_move == -10 {
                self.x_toggle = !self.x_toggle;
            }
            if self.y_move == 2 || self.y_move == -2 {
                self.y_toggle = !self.y_toggle;
            }

            if self.x_toggle {
                self.x_move += 1;
            } else {
                self.x_move -= 1;
            }
            
            if self.y_toggle {
                self.y_move += 1;
            } else {
                self.y_move -= 1;
            }
        }
    }
    
    fn get_location(&self) -> Location {
        let mut new_x = self.initial_location.x as i32 + self.x_move;
        let mut new_y = self.initial_location.y as i32 + self.y_move;
        
        if new_x < 0 {
            new_x = 0;
        }
        
        if new_y < 0 {
            new_y = 0;
        }
        
        Location::new(new_x as u32, new_y as u32)
    }
}

impl Movement for SmallStepsMovement {
    fn next_position(&mut self) -> Location {
        self.update_state();
        self.get_location()
    }
}
