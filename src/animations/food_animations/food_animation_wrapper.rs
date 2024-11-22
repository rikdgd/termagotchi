use crate::animations::Animation;
use crate::animations::food_animations::BurgerAnimation;
use crate::shapes::PixelVectorShape;

pub enum FoodAnimationWrapper {
    Burger(BurgerAnimation),
}

impl Animation for FoodAnimationWrapper {
    fn next_frame(&mut self) -> Option<PixelVectorShape> {
        match self {
            FoodAnimationWrapper::Burger(animation) => animation.next_frame(),
        }
    }
}