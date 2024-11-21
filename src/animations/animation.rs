use crate::shapes::PixelVectorShape;

pub trait Animation {
    /// Gives the next frame of the animation, updating its own state.
    /// ## returns:
    /// `crate::shapes::PixelVectorShape` - A pixel vector wrapper that implements `ratatui::widgets::canvas::Shape`.
    fn next_frame(&mut self) -> PixelVectorShape;
    fn is_running(&self) -> bool;
}
