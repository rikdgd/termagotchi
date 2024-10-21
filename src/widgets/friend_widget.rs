use ratatui::prelude::Color;
use ratatui::widgets::Block;
use ratatui::widgets::canvas::{Canvas, Context, Map, MapResolution};
use crate::shapes::creatures::CreatureShapes;
use crate::utils::ColorWrapper;

pub fn friend_widget() -> Canvas<'static, fn(&mut Context)> {
    let friend_widget_x_bounds = [-180.0, 180.0];
    let friend_widget_y_bounds = [-90.0, 90.0];

    Canvas::default()
        .block(Block::bordered().title("Friend"))
        .x_bounds(friend_widget_x_bounds)
        .y_bounds(friend_widget_y_bounds)
        .paint(|ctx| {
            ctx.draw(&Map {
                resolution: MapResolution::High,
                color: Color::White,
            });
            ctx.layer();
            ctx.draw(&CreatureShapes::Duck(ColorWrapper::Cyan))
        })
}
