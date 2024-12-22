use crate::shapes::PixelImage;
use crate::utils::{Pixel, sprite_management::load_sprite, ColorWrapper};
use crate::load_embedded_sprite;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StatShape {
    Food,
    Joy,
    Sleep,
    Waste,
}
impl PixelImage for StatShape {
    fn pixels(&self) -> Vec<Pixel> {
        let color = ColorWrapper::White;
        match self {
            StatShape::Food => load_embedded_sprite!("../../assets/stat_symbols/burger.png", color),
            StatShape::Joy => load_embedded_sprite!("../../assets/stat_symbols/heart.png", color),
            StatShape::Sleep => load_embedded_sprite!("../../assets/stat_symbols/zz.png", color),
            StatShape::Waste => load_embedded_sprite!("../../assets/stat_symbols/toilet_paper.png", color),
        }
    }
}
