use crate::utils::Pixel;

pub trait PixelAnimation {
    /// Returns the next frame and updates the animation's state.
    fn next_frame(&self) -> Vec<Pixel>;
}
