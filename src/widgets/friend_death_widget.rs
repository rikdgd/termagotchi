use ratatui::layout::Alignment;
use ratatui::prelude::{Line, Stylize, Widget};
use ratatui::symbols::Marker;
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::widgets::canvas::Canvas;
use crate::load_embedded_sprite;
use crate::shapes::PixelVectorShape;
use crate::utils::ColorWrapper;
use crate::utils::sprite_management::load_sprite;

pub fn death_canvas() -> impl Widget + 'static {
    let death_message = load_embedded_sprite!("../../assets/death-message.png", ColorWrapper::White);
    let death_message = PixelVectorShape::new(death_message)
        .translate(10, 0);
    
    Canvas::default()
        .block(Block::bordered())
        .marker(Marker::Braille)
        .x_bounds([0.0, 170.0])
        .y_bounds([0.0, 100.0])
        .paint(move |ctx| {
            ctx.draw(&death_message);
        })
}

pub fn name_input<'a>(input: &'a str) -> impl Widget + 'a {
    let text = vec![
        Line::from(input.bold()),
    ];
    Paragraph::new(text)
        .block(Block::bordered().title("Enter a name for your new pet:"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}