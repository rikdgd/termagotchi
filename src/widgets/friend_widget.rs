use ratatui::prelude::Color;
use ratatui::widgets::{Widget, Block};
use ratatui::widgets::canvas::{Canvas, Context};
use crate::friend::Friend;
use crate::friend::ShapeWrapper;
use crate::shapes::{PixelImage, move_pixel_image, PixelVectorShape};
use crate::movements::Location;

pub struct FriendWidget<'a> {
    friend: &'a Friend,
    friend_location: Location,
}
impl<'a> FriendWidget<'a> {
    pub fn new(friend: &'a Friend, friend_location: (u32, u32)) -> Self {
        Self { 
            friend, 
            friend_location: Location::new(friend_location.0, friend_location.1), 
        }
    }

    pub fn get_widget(&self) -> impl Widget + '_ {
        let friend_widget_x_bounds = [-180.0, 180.0];
        let friend_widget_y_bounds = [-90.0, 90.0];
        
        let canvas = Canvas::default()
            .block(Block::bordered().title(self.friend.name()))
            .x_bounds(friend_widget_x_bounds)
            .y_bounds(friend_widget_y_bounds)
            .paint(|ctx| {
                // TODO: Create simple background
                // ctx.draw(&Map {
                //     resolution: MapResolution::High,
                //     color: Color::White,
                // });
                // ctx.layer();

                match self.friend.get_shape_wrapper() {
                    ShapeWrapper::Growing(shape) => ctx.draw(&shape),
                    ShapeWrapper::Adult(shape) => draw_shape_at_location(ctx, &shape, &self.friend_location),
                }
            });
        
        match self.friend.is_asleep() {
            true => canvas.background_color(Color::Black),
            false => canvas.background_color(Color::Reset),
        }
    }
}

fn draw_shape_at_location<S: PixelImage>(ctx: &mut Context, shape: &S, location: &Location) {
    let new_pixels = move_pixel_image(shape, (location.x, location.y));
    let vec_image = PixelVectorShape::new(new_pixels);
    
    ctx.draw(&vec_image);
}
