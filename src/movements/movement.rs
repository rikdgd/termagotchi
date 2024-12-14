use crate::utils::location::Location;
use super::{
    DvdBounceMovement,
    EggHopMovement,
    SmallStepsMovement
};


pub trait Movement {
    /// Updates the state of the movement and returns the new location.
    fn next_position(&mut self) -> Location;
}


#[derive(Debug, Clone, PartialEq)]
pub enum MovementWrapper {
    EggHop(EggHopMovement),
    SmallSteps(SmallStepsMovement),
    DvdBounce(DvdBounceMovement),
}
impl<'a> Movement for MovementWrapper {
    fn next_position(&mut self) -> Location {
        match self {
            MovementWrapper::EggHop(movement) => movement.next_position(),
            MovementWrapper::SmallSteps(movement) => movement.next_position(),
            MovementWrapper::DvdBounce(movement) => movement.next_position(),
        }
    }
}
