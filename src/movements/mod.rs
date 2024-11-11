mod egg_hop;
mod movement;
mod small_steps;

pub use movement::{Movement, MovementWrapper, Location};
pub use egg_hop::EggHopMovement;
pub use small_steps::SmallStepsMovement;