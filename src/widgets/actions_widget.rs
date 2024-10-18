use ratatui::prelude::{Modifier, Style};
use ratatui::widgets::{Block, List};


pub const ITEMS: [&str; 4] = [
        "Eat", 
        "Play", 
        "Sleep",
        "Poop",
    ];

pub fn actions_widget() -> List<'static> {
    List::new(ITEMS)
        .block(Block::bordered().title("List"))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">")
        .repeat_highlight_symbol(true)
}