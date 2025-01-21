use crate::utils::sprite_management::load_sprite;
use chrono::Utc;
use crate::load_embedded_sprite;
use crate::shapes::PixelVectorShape;
use crate::utils::{ColorWrapper, Pixel};
use super::Animation;

const FRAME_COUNT: u8 = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FoodAnimation {
    animation_frames: FoodAnimationFrames,
    current_frame: u8,
    last_time_update: i64,
}

impl FoodAnimation {
    pub fn new(animation_frames: FoodAnimationFrames) -> Self {
        Self {
            animation_frames,
            current_frame: 0,
            last_time_update: Utc::now().timestamp_millis(),
        }
    }
}

impl Animation for FoodAnimation {
    fn next_frame(&mut self) -> Option<PixelVectorShape> {
        let pixel_vec = match self.current_frame {
            0 => self.animation_frames.frames()[0].clone(),
            1 => self.animation_frames.frames()[1].clone(),
            2 => self.animation_frames.frames()[2].clone(),
            _ => self.animation_frames.frames()[3].clone(),
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


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FoodAnimationFrames {
    Soup,
    Cookie,
    Burger,
}
impl FoodAnimationFrames {
    pub fn frames(&self) -> [Vec<Pixel>; FRAME_COUNT as usize] {
        match self {
            FoodAnimationFrames::Soup => [
                load_embedded_sprite!("../../assets/food/soup.png", ColorWrapper::White),
                load_embedded_sprite!("../../assets/food/soup1.png", ColorWrapper::White),
                load_embedded_sprite!("../../assets/food/soup2.png", ColorWrapper::White),
                load_embedded_sprite!("../../assets/food/soup3.png", ColorWrapper::White),
            ],
            FoodAnimationFrames::Cookie => [
                load_embedded_sprite!("../../assets/food/cookie.png", ColorWrapper::White),
                load_embedded_sprite!("../../assets/food/cookie1.png", ColorWrapper::White),
                load_embedded_sprite!("../../assets/food/cookie2.png", ColorWrapper::White),
                load_embedded_sprite!("../../assets/food/cookie3.png", ColorWrapper::White),
            ],
            FoodAnimationFrames::Burger => [
                load_embedded_sprite!("../../assets/food/burger.png", ColorWrapper::White),
                load_embedded_sprite!("../../assets/food/burger1.png", ColorWrapper::White),
                load_embedded_sprite!("../../assets/food/burger2.png", ColorWrapper::White),
                load_embedded_sprite!("../../assets/food/burger3.png", ColorWrapper::White),
            ],
        }
    }
}