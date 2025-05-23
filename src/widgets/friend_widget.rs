use chrono::Utc;
use ratatui::prelude::Color;
use ratatui::symbols::Marker;
use ratatui::widgets::{Widget, Block};
use ratatui::widgets::canvas::{Canvas, Context};
use crate::friend::Friend;
use crate::friend::ShapeWrapper;
use crate::shapes::{PixelImage, PixelVectorShape};
use crate::utils::location::Location;
use ratatui::layout::Rect;
use ratatui::text::Line;
use crate::animations::{Animation, SleepingAnimation};

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
            .block(Block::bordered().title(Line::from(self.title_string()).centered()))
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
                };
                
                if self.friend.is_asleep() {
                    let mut sleep_animation = SleepingAnimation::new(self.friend_location);
                    
                    if let Some(frame) = sleep_animation.next_frame() {
                        ctx.draw(&frame);
                    }
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
        format!("  {name}  |  Age: {hours_alive} hours  ")
    }
}

fn draw_shape_at_location<S: PixelImage>(ctx: &mut Context, shape: &S, location: &Location) {
    let vec_shape = PixelVectorShape::from_pixel_image(shape)
        .translate(location.x as i32, location.y as i32);
    
    ctx.draw(&vec_shape);
}

