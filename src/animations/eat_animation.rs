use ratatui::{
    layout::Rect,
    widgets::{Block, Clear},
    Frame,
};
use crate::animations::popup_animation::PopupAnimation;
use crate::Food;

struct EatAnimation {
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
        let area = Rect::new(20, 20, 30, 40); // TODO: place in center of terminal.
        frame.render_widget(Clear, area); // Clear out the background behind the popup.
        frame.render_widget(block, area);
    }
}
