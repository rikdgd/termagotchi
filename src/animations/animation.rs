use crate::shapes::PixelVectorShape;

pub trait Animation {
    /// Gives the next frame of the animation, updating its own state.
    /// ## returns:
    /// * `Some(PixelVectorShape)` - A pixel vector wrapper that implements `ratatui::widgets::canvas::Shape`. 
    /// * `None` - When no next frame remains.
    fn next_frame(&mut self) -> Option<PixelVectorShape>;
}
