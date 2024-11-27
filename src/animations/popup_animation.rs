use std::fmt::Debug;
use ratatui::{widgets::{Block, Clear}, Frame};
use ratatui::layout::Rect;
use ratatui::widgets::canvas::Canvas;
use crate::shapes::PixelVectorShape;
use super::animation::Animation;


/// ## PopupAnimation
/// PopupAnimations can be used to display a short little animation 
/// in a small popup that covers the rest of the UI. They are for example used 
/// for the eating/playing/sleeping animations.
pub struct PopupAnimation<> {
    is_running: bool,
    animation: Box<dyn Animation>,
}

impl PopupAnimation {
    pub fn new(animation: Box<dyn Animation>) -> Self {
        Self {
            is_running: true,
            animation, 
        }
    }
    
    /// Renders the PopupAnimation on the `ratatui::Frame`. 
    /// This method automatically updates the animations state.
    /// ## parameters: 
    /// * `frame` - The `ratatui::Frame` to render the PopupAnimation on
    pub fn render(&mut self, frame: &mut Frame) {
        let area = self.get_popup_rect(frame);
        let next_animation_frame = 
            if let Some(mut animation_frame) = self.animation.next_frame() {
                let area = self.get_popup_rect(frame);
                
                animation_frame.move_shape(
                    area.width as u32 / 2, 
                    area.height as u32 / 2,
                );

                animation_frame
            } else {
                self.is_running = false;
                PixelVectorShape::new(Vec::new())
            };

        let canvas = Canvas::default()
            .block(Block::bordered())
            .paint(|ctx| { 
                ctx.draw(&next_animation_frame) 
            });

        frame.render_widget(Clear, area); // Clear out the background behind the popup.
        frame.render_widget(canvas, area);
    }
    
    /// Returns if the animation is still running, can be used to 
    /// decide when to close the PopupAnimation.
    pub fn is_running(&self) -> bool {
        self.is_running
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
