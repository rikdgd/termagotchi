use crate::friend::Friend;
use crate::shapes::PixelVectorShape;
use crate::friend::GrowthShapeWrapper;
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
    
    pub fn get_widget(&mut self) -> impl Widget + '_ {
        let canvas = Canvas::default()
            .block(Block::bordered().title(Line::from("Score: x").centered()))
            .marker(Marker::Braille)
            .x_bounds([0.0, f64::from(self.game_area.width)])
            .y_bounds([0.0, f64::from(self.game_area.height)])
            .paint(|ctx| {
                // TODO: Draw the actual game state
            });
        
        canvas 
    }

    pub fn get_frame(&mut self) -> Option<PixelVectorShape> {
        self.game_state.update_state();
        // TODO: Create the necessary shapes

        todo!()
    }
}

