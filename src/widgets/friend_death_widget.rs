use ratatui::prelude::Widget;
use ratatui::symbols::Marker;
use ratatui::widgets::Block;
use ratatui::widgets::canvas::Canvas;
use crate::load_embedded_sprite;
use crate::shapes::PixelVectorShape;
use crate::utils::ColorWrapper;
use crate::utils::sprite_management::load_sprite;

pub fn death_canvas() -> impl Widget + 'static {
    let death_message = load_embedded_sprite!("../../assets/death-message.png", ColorWrapper::White);
    let death_message = PixelVectorShape::new(death_message);
    
    Canvas::default()
        .block(Block::bordered())
        .marker(Marker::Braille)
        .x_bounds([0.0, 150.0])
        .y_bounds([0.0, 100.0])
        .paint(move |ctx| {
            ctx.draw(&death_message);
        })
}