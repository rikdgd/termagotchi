use super::movement::{Movement, Location};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EggHopMovement {
    start_location: Location,
    is_grounded: bool,
}

impl EggHopMovement {
    pub fn new(start_location: Location) -> Self {
        Self {
            start_location,
            is_grounded: true 
        }
    }
}

impl Movement for EggHopMovement {
    fn next_position(&mut self) -> Location {
        self.is_grounded = !self.is_grounded;
        
        if self.is_grounded {
            self.start_location.clone()
        } else {
            let mut location = self.start_location;
            location.y += 1;
            location
        }
    }

    fn frame_count(&self) -> u32 {
        2
    }
}
