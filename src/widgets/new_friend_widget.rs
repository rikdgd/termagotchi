use ratatui::{
    widgets::Widget,
    layout::Alignment,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
};

pub fn new_friend_dialog<'a>() -> impl Widget + 'a {
    let text = vec![
        Line::from("You currently don't have a Friend, we should change that."),
        Line::from("To start, what should your friend be called? Enter a name and then press 'enter'"),
    ];
    Paragraph::new(text)
        .block(Block::bordered().title("new friend"))
        // .style(Style::new().white().on_black())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

pub fn new_friend_name_input<'a>(input: &'a str) -> impl Widget + 'a {
    let text = vec![
        Line::from(input.bold()),
    ];
    Paragraph::new(text)
        .block(Block::bordered().title("enter name:"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}