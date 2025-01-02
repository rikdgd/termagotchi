use chrono::Utc;
use crate::animations::Animation;
use crate::shapes::PixelVectorShape;

const FRAME_COUNT: u8 = 4;

pub struct HealthAnimation {
    current_frame: u8,
    last_time_update: i64,
}

impl Animation for HealthAnimation {
    fn next_frame(&mut self) -> Option<PixelVectorShape> {
        todo!()
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
