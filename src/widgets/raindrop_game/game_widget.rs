use crate::friend::Friend;
use crate::shapes::PixelImage;
use crate::friend::ShapeWrapper;

pub struct GameWidgetManager {
    player_x_location: u32,
    score: u32,
    friend_shape: Box<dyn PixelImage>,
}

impl GameWidgetManager {
    pub fn new(friend: &Friend) -> Self {
        let shape: Box<dyn PixelImage> = match friend.get_shape_wrapper() {
            ShapeWrapper::Growing(shape) => Box::new(shape),
            ShapeWrapper::Adult(shape) => Box::new(shape),
        };
        
        Self {
            player_x_location: 0, // TODO: This should be the center of the screen, maybe just get it from the terminal.
            score: 0,
            friend_shape: shape,
        }
    }
}