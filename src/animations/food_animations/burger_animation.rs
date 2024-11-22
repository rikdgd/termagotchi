use crate::utils::sprite_management::load_sprite;
use chrono::Utc;
use crate::load_embedded_sprite;
use crate::shapes::PixelVectorShape;
use crate::utils::ColorWrapper;
use super::Animation;

const FRAME_COUNT: u8 = 4;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BurgerAnimation {
    current_frame: u8,
    last_time_update: i64,
}

impl BurgerAnimation {
    pub fn new() -> Self {
        Self {
            current_frame: 0,
            last_time_update: Utc::now().timestamp_millis(),
        }
    }
}

impl Animation for BurgerAnimation {
    fn next_frame(&mut self) -> Option<PixelVectorShape> {
        let pixel_vec = match self.current_frame {
            0 => load_embedded_sprite!("../../../assets/food/burger.png", ColorWrapper::White),
            1 => load_embedded_sprite!("../../../assets/food/burger1.png", ColorWrapper::White),
            2 => load_embedded_sprite!("../../../assets/food/burger2.png", ColorWrapper::White),
            _ => load_embedded_sprite!("../../../assets/food/burger3.png", ColorWrapper::White),
        };
        
        let now = Utc::now().timestamp_millis();
        if now - self.last_time_update >= 750 {
            self.current_frame += 1;
            self.last_time_update = now;
        }
        
        if self.current_frame >= FRAME_COUNT {
            return None;
        }
        
        Some(PixelVectorShape::new(pixel_vec))
    }
}