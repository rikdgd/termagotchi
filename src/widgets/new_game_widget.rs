use ratatui::{
    widgets::Widget,
    layout::Alignment,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Wrap},
};

pub fn new_game_dialog<'a>() -> impl Widget + 'a {
    let text = vec![
        Line::from("Welcome to Termagotchi! Since this is your first time, I'll explain the basics."),
        Line::from(""),
        Line::from("Your goal is to keep your pet alive by taking good care of it."),
        Line::from("To keep your pet healthy, its stats need to stay as high as possible. They are visible on the left."),
        Line::from("You can raise a stat by performing the appropriate action."),
        Line::from("To navigate the 'actions' menu, use the 'Up' and 'Down' keys on your keyboard. Press 'Enter' to perform the action."),
        Line::from(""),
        Line::from("If you want to close the game you can press 'q', or you can always just close the terminal."),
        Line::from(""),
        Line::from("To start, what should your first pet be called? Enter a name and then press 'Enter' to continue."),
    ];
    Paragraph::new(text)
        .block(Block::bordered())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

pub fn new_game_name_input(input: &str) -> impl Widget + '_ {
    let text = vec![
        Line::from(input.bold()),
    ];
    Paragraph::new(text)
        .block(Block::bordered().title("Enter pet name:"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}