use chrono::Utc;
use crate::animations::Animation;
use crate::load_embedded_sprite;
use crate::utils::ColorWrapper;
use crate::utils::sprite_management::load_sprite;
use crate::shapes::PixelVectorShape;

const FRAME_COUNT: u8 = 4;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HealthAnimation {
    current_frame: u8,
    last_time_update: i64,
}

impl Animation for HealthAnimation {
    fn next_frame(&mut self) -> Option<PixelVectorShape> {
        if self.current_frame >= FRAME_COUNT {
            return None;
        }
        
        let pixel_vec = match self.current_frame {
            0 => load_embedded_sprite!("../../assets/health/syringe.png", ColorWrapper::White),
            1 => load_embedded_sprite!("../../assets/health/syringe1.png", ColorWrapper::White),
            2 => load_embedded_sprite!("../../assets/health/syringe2.png", ColorWrapper::White),
            _ => load_embedded_sprite!("../../assets/health/syringe3.png", ColorWrapper::White),
        };
        
        let now = Utc::now().timestamp_millis();
        if now - self.last_time_update >= 750 {
            self.current_frame += 1;
            self.last_time_update = now;
        }
        
        Some(PixelVectorShape::new(pixel_vec))
    }
}

impl HealthAnimation {
    pub fn new() -> Self {
        Self {
            current_frame: 0,
            last_time_update: Utc::now().timestamp_millis(),
        }
    }
}
