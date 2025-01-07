mod popup_animation;
mod animation;
pub mod food_animation;
pub mod health_animation;
pub mod joy_animation;
mod sleeping_animation;

pub use animation::Animation;
pub use popup_animation::PopupAnimation;

pub use health_animation::HealthAnimation;
pub use joy_animation::JoyAnimation;
pub use sleeping_animation::SleepingAnimation;