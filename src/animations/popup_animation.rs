use ratatui::Frame;
use ratatui::layout::Rect;
use crate::animations::EatAnimation;

pub trait PopupAnimation {
    fn render(&mut self, frame: &mut Frame);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PopupAnimationWrapper {
    EatAnimation(EatAnimation),
}

impl PopupAnimation for PopupAnimationWrapper {
    fn render(&mut self, frame: &mut Frame) {
        match self {
            PopupAnimationWrapper::EatAnimation(animation) => animation.render(frame),
        }
    }
}


/// Returns a rectangle that has 1/3 the width and height of
/// the given frame, which is also centered in the frame.
///
/// ## parameters:
/// * `frame` - The `ratatui::Frame` to use for calculating the popup area.
///
/// ## Returns:
/// A `ratatui::layout::Rect` instance that corresponds to the popup area.
pub fn get_popup_rect(frame: &Frame) -> Rect {
    let area = frame.area();
    let popup_width = area.width / 3;
    let popup_height = area.height / 3;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;

    Rect::new(popup_x, popup_y, popup_width, popup_height)
}
