use ratatui::prelude::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, List};


pub const ITEMS: [&str; 4] = [
    "Eat", 
    "Play", 
    "Sleep",
    "Medicine",
];

pub fn actions_widget() -> List<'static> {
    List::new(ITEMS)
        .block(Block::bordered().title(Line::from(" Actions ").centered()))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .repeat_highlight_symbol(true)
}