use chrono::Utc;
use crate::animations::Animation;
use crate::load_embedded_sprite;
use crate::utils::sprite_management::load_sprite;
use crate::shapes::PixelVectorShape;
use crate::utils::ColorWrapper;
use crate::utils::location::Location;

pub struct SleepingAnimation {
    friend_location: Location,
}

impl Animation for SleepingAnimation {
    fn next_frame(&mut self) -> Option<PixelVectorShape> {
        let shape = load_embedded_sprite!("../../assets/sleeping/zz.png", ColorWrapper::White);
        let shape = PixelVectorShape::new(shape).translate(
            self.friend_location.x as i32,
            self.friend_location.y as i32,
        );
        
        let now = Utc::now().timestamp_millis();
        if (now / 1000) % 2 == 0 {
            Some(shape.translate(20, 20))
        } else {
            Some(shape.translate(20, 22))
        }
    }
}

impl SleepingAnimation {
    pub fn new(friend_location: Location) -> Self {
        Self {
            friend_location,
        }
    }
}