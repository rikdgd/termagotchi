use ratatui::widgets::canvas::{Painter, Shape};
use serde::{Deserialize, Serialize};
use crate::shapes::PixelImage;
use crate::utils::{ColorWrapper, Pixel, sprite_management::load_sprite};
use crate::load_embedded_sprite;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GrowthStageShapes {
    Egg(ColorWrapper),
    Baby(ColorWrapper),
    Kid(ColorWrapper),
}

impl PixelImage for GrowthStageShapes {
    fn pixels(&self) -> Vec<Pixel> {
        match self {
            GrowthStageShapes::Egg(color) => load_embedded_sprite!("../../assets/egg.png", color),
            GrowthStageShapes::Baby(color) => load_embedded_sprite!("../../assets/baby.png", color),
            GrowthStageShapes::Kid(color) => load_embedded_sprite!("../../assets/kid.png", color),
        }
    }
}

impl Shape for GrowthStageShapes {
    fn draw(&self, painter: &mut Painter) {
        for pixel in self.pixels() {
            painter.paint(
                pixel.x as usize, 
                pixel.y as usize, 
                pixel.color
            );
        }
    }
}
