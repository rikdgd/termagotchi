use super::movement::Movement;
use rand;
use chrono::Utc;
use rand::Rng;
use ratatui::layout::Rect;
use crate::shapes::PixelVectorShape;
use crate::utils::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct DvdBounceMovement {
    location: Location,
    x_direction_toggle: bool,
    y_direction_toggle: bool,
    area: Rect,
    last_update: i64,
    friend_shape: PixelVectorShape,
}
impl DvdBounceMovement {
    pub fn new(start_location: Location, area: Rect, friend_shape: PixelVectorShape) -> Self {
        let mut rng = rand::thread_rng();
        let x_rng = rng.gen_range(-30..=30);
        let y_rng = rng.gen_range(-30..=30);
        
        let new_x = (start_location.x as i32 + x_rng) as u32;
        let new_y = (start_location.y as i32 + y_rng) as u32;
        
        let location = Location {
            x: new_x - 12, // -12 to account for sprite dimensions (25x25)
            y: new_y - 12,
        };
        
        Self {
            location,
            x_direction_toggle: true,
            y_direction_toggle: true,
            area,
            last_update: Utc::now().timestamp_millis(),
            friend_shape,
        }
    }

    fn update_state(&mut self) {
        let (shape_width, shape_height) = self.friend_shape.get_dimensions();

        let now = Utc::now().timestamp_millis();

        if now - self.last_update > 500 {
            self.last_update = now;
            
            if self.location.x <= self.area.left() as u32
                || self.location.x > self.area.right() as u32 - shape_width
            {
                self.x_direction_toggle = !self.x_direction_toggle;
            }
            
            if self.location.y <= self.area.top() as u32
                || self.location.y > self.area.bottom() as u32 - shape_height
            {
                self.y_direction_toggle = !self.y_direction_toggle;
            }

            let new_x = if self.x_direction_toggle {
                self.location.x + 1
            } else {
                self.location.x - 1
            };
            let new_y = if self.y_direction_toggle {
                self.location.y + 1
            } else {
                self.location.y - 1
            };

            self.location = Location::new(new_x, new_y);
        }
    }
}

impl Movement for DvdBounceMovement {
    fn next_position(&mut self) -> Location {
        self.update_state();
        self.location
    }
}