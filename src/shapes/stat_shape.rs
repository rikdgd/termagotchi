use crate::shapes::PixelImage;
use crate::utils::{Pixel, sprite_management::load_sprite, ColorWrapper};
use crate::load_embedded_sprite;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StatShape {
    Food,
    Joy,
    Sleep,
    Health,
}
impl PixelImage for StatShape {
    fn pixels(&self) -> Vec<Pixel> {
        let color = ColorWrapper::White;
        match self {
            StatShape::Food => load_embedded_sprite!("../../assets/stat_symbols/eat.png", color),
            StatShape::Joy => load_embedded_sprite!("../../assets/stat_symbols/heart.png", color),
            StatShape::Sleep => load_embedded_sprite!("../../assets/stat_symbols/zz.png", color),
            StatShape::Health => load_embedded_sprite!("../../assets/stat_symbols/health.png", color),
        }
    }
}
