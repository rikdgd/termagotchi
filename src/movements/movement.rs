use super::{EggHopMovement, SmallStepsMovement};

pub trait Movement {
    /// Updates the state of the movement and returns the new location.
    fn next_position(&mut self) -> Location;
}


#[derive(Debug, Clone, PartialEq)]
pub enum MovementWrapper {
    EggHop(EggHopMovement),
    SmallSteps(SmallStepsMovement),
}
impl Movement for MovementWrapper {
    fn next_position(&mut self) -> Location {
        match self {
            MovementWrapper::EggHop(movement) => movement.next_position(),
            MovementWrapper::SmallSteps(movement) => movement.next_position(),
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Location {
    pub x: u32,
    pub y: u32,
}
impl Location {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x, 
            y,
        }
    }
}
