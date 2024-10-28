use ratatui::prelude::Color;
use ratatui::widgets::{Widget, Block};
use ratatui::widgets::canvas::{Canvas, Map, MapResolution};
use crate::friend::Friend;

pub struct FriendWidget<'a> {
    friend: &'a Friend,
    friend_location: (u32, u32),
}
impl<'a> FriendWidget<'a> {
    pub fn new(friend: &'a Friend, friend_location: (u32, u32)) -> Self {
        Self { friend, friend_location }
    }

    pub fn get_widget(&self) -> impl Widget + '_ {
        let friend_widget_x_bounds = [-180.0, 180.0];
        let friend_widget_y_bounds = [-90.0, 90.0];
        
        let canvas = Canvas::default()
            .block(Block::bordered().title(self.friend.name()))
            .x_bounds(friend_widget_x_bounds)
            .y_bounds(friend_widget_y_bounds)
            .paint(|ctx| {
                ctx.draw(&Map {
                    resolution: MapResolution::High,
                    color: Color::White,
                });
                ctx.layer();
                ctx.draw(self.friend.shape())
            });
        
        match self.friend.is_asleep() {
            true => canvas.background_color(Color::Black),
            false => canvas.background_color(Color::Reset),
        }
    }
}
