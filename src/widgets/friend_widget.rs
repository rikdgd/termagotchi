use chrono::Utc;
use ratatui::prelude::Color;
use ratatui::symbols::Marker;
use ratatui::widgets::{Widget, Block};
use ratatui::widgets::canvas::{Canvas, Context, Rectangle};
use crate::friend::Friend;
use crate::friend::ShapeWrapper;
use crate::shapes::{PixelImage, move_pixel_image, PixelVectorShape};
use crate::movements::Location;
use ratatui::layout::Rect;

pub struct FriendWidget<'a> {
    friend: &'a Friend,
    friend_location: Location,
    movement_area: Rect,
}
impl<'a> FriendWidget<'a> {
    pub fn new(friend: &'a Friend, friend_location: Location, movement_area: Rect) -> Self {
        Self { 
            friend, 
            friend_location,
            movement_area,
        }
    }

    pub fn get_widget(&self) -> impl Widget + '_ {
        let friend_widget_x_bounds = [0.0, f64::from(self.movement_area.width)];
        let friend_widget_y_bounds = [0.0, f64::from(self.movement_area.height)];
        
        let canvas = Canvas::default()
            .block(Block::bordered().title(self.title_string()))
            .marker(Marker::Braille)
            .x_bounds(friend_widget_x_bounds)
            .y_bounds(friend_widget_y_bounds)
            .paint(|ctx| {
                // TODO: Create simple background
                // ctx.draw(&background or something);
                // ctx.layer();

                match self.friend.get_shape_wrapper() {
                    ShapeWrapper::Growing(shape) => draw_shape_at_location(ctx, &shape, &self.friend_location),
                    ShapeWrapper::Adult(shape) => draw_shape_at_location(ctx, &shape, &self.friend_location),
                }
            });
        
        match self.friend.is_asleep() {
            true => canvas.background_color(Color::Black),
            false => canvas.background_color(Color::Reset),
        }
    }

    fn title_string(&self) -> String {
        let now = Utc::now().timestamp_millis();
        let millis_alive = now - self.friend.time_created();
        let hours_alive = millis_alive / 1000 / 60 / 60;
        let name = self.friend.name();
        format!("{name} - age: {hours_alive} hours")
    }
}

fn draw_shape_at_location<S: PixelImage>(ctx: &mut Context, shape: &S, location: &Location) {
    let new_pixels = move_pixel_image(shape, (location.x, location.y));
    let vec_image = PixelVectorShape::new(new_pixels);
    
    ctx.draw(&vec_image);
}

