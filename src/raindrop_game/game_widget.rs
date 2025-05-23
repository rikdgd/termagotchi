use crate::friend::Friend;
use crate::shapes::PixelVectorShape;
use crate::friend::GrowthShapeWrapper;
use crate::utils::{Pixel, location::Location};
use ratatui::prelude::Color;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget, Block};
use ratatui::widgets::canvas::{Canvas, Context};
use ratatui::text::Line;

use ratatui::symbols::Marker;
use super::state_management::RaindropGameState;


#[derive(Debug, Clone)]
pub struct GameWidgetManager {
    game_state: RaindropGameState,
    friend_shape: PixelVectorShape,
    game_area: Rect,
}

impl GameWidgetManager {
    pub fn new(friend: &Friend, game_area: Rect) -> Self {
        let friend_shape = match friend.get_shape_wrapper() {
            GrowthShapeWrapper::Growing(shape) => PixelVectorShape::from_pixel_image(&shape),
            GrowthShapeWrapper::Adult(shape) => PixelVectorShape::from_pixel_image(&shape),
        };
        
        Self {
            game_state: RaindropGameState::new(game_area),
            friend_shape,
            game_area,
        }
    }

    /// Get the actual game widget to be rendered on the terminal. This will have its state updated
    /// automatically.
    pub fn get_widget(&mut self) -> impl Widget + '_ {
        let frame = self.get_frame().unwrap();
        
        let canvas = Canvas::default()
            .block(Block::bordered().title(Line::from("Score: x").centered()))
            .marker(Marker::Braille)
            .x_bounds([0.0, f64::from(self.game_area.width)])
            .y_bounds([0.0, f64::from(self.game_area.height)])
            .paint(move |ctx| {
                ctx.draw(&frame);
            });
        
        canvas 
    }
    
    /// Get the next frame of the game as a `PixelVectorShape`, this automatically progresses the 
    /// game's state.
    /// ## Returns:
    /// This method returns a `PixelVectorShape` containing the entire game frame. If the players 
    /// health is 0, the game ends and `None` is returned as there are no more frames to generate.
    fn get_frame(&mut self) -> Option<PixelVectorShape> {
        if self.game_state.health == 0 {
            return None;
        }
        
        self.game_state.update_state();
                
        // Add the friend as shape to use the shape as the base for the frame, and relocate 
        // it to a proper location.
        let mut frame_shape = self.friend_shape.clone().translate(
            self.game_state.player_x as i32,
            30,
        );
        
        // Merge each drop image into the frame shape
        for drop in &self.game_state.drop_locations {
            frame_shape.merge(raindrop_shape(*drop));
        }
        
        Some(frame_shape)
    }
}

/// This function is used to get a shape representing a raindrop on the correct location of the 
/// screen.
/// ## Parameters:
/// * `location` - The location where the raindrop should be drawn on screen.
/// ## Returns:
/// A `PixelVectorShape` representing a raindrop with it location updated to the given one.
fn raindrop_shape(location: Location) -> PixelVectorShape {
    let pixels = vec![
        Pixel {
            x: location.x,
            y: location.y,
            color: Color::White,
        },
        Pixel {
            x: location.x,
            y: location.y + 1,
            color: Color::White,
        },
    ];

    PixelVectorShape::new(pixels)
}
