use ratatui::{widgets::{Block, Clear}, Frame};
use crate::animations::popup_animation::PopupAnimation;
use crate::Food;
use super::get_popup_rect;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EatAnimation {
    food: Food,
}

impl EatAnimation {
    pub fn new(food: Food) -> Self {
        Self { food }
    }
}

impl PopupAnimation for EatAnimation {
    fn render(&mut self, frame: &mut Frame) {
        let block = Block::bordered();
        let area = get_popup_rect(frame);
        frame.render_widget(Clear, area); // Clear out the background behind the popup.
        frame.render_widget(block, area);
    }
}
