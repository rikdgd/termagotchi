use ratatui::{
    widgets::Widget,
    layout::Alignment,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
};

pub fn new_friend_widget<'a>() -> impl Widget + 'a {
    let text = vec![
        Line::from("You currently don't have a Friend, we should change that."),
        Line::from("To start, what should your friend be called?"),
    ];
    Paragraph::new(text)
        .block(Block::bordered().title("new friend"))
        // .style(Style::new().white().on_black())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}