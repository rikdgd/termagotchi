use ratatui::{widgets::{Block, Clear}, Frame};
use ratatui::layout::Rect;
use ratatui::widgets::canvas::Canvas;
use super::animation::Animation;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PopupAnimation<T: Animation> {
    animation: T,
}

impl<T: Animation> PopupAnimation<T> {
    pub fn new(animation: T) -> Self {
        Self { animation }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = self.get_popup_rect(frame);
        let next_frame = self.animation.next_frame();

        let canvas = Canvas::default()
            .block(Block::bordered())
            // .x_bounds(friend_widget_x_bounds)
            // .y_bounds(friend_widget_y_bounds)
            .paint(|ctx| { ctx.draw(&next_frame) });

        frame.render_widget(Clear, area); // Clear out the background behind the popup.
        frame.render_widget(canvas, area);
    }
    
    /// Returns a rectangle that has 1/3 the width and height of
    /// the given frame, which is also centered in the frame.
    ///
    /// ## parameters:
    /// * `frame` - The `ratatui::Frame` to use for calculating the popup area.
    ///
    /// ## Returns:
    /// A `ratatui::layout::Rect` instance that corresponds to the popup area.
    fn get_popup_rect(&self, frame: &Frame) -> Rect {
        let area = frame.area();
        let popup_width = area.width / 3;
        let popup_height = area.height / 3;
        let popup_x = (area.width - popup_width) / 2;
        let popup_y = (area.height - popup_height) / 2;

        Rect::new(popup_x, popup_y, popup_width, popup_height)
    }
}


