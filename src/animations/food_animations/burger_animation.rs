use crate::shapes::PixelVectorShape;
use super::Animation;

pub struct BurgerAnimation {
    last_time_update: i64,
}

impl Animation for BurgerAnimation {
    fn next_frame(&mut self) -> PixelVectorShape {
        todo!()
    }

    fn is_running(&self) -> bool {
        todo!()
    }
}