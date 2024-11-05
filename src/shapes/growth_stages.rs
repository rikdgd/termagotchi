use ratatui::widgets::canvas::{Painter, Shape};
use serde::{Deserialize, Serialize};
use crate::shapes::PixelImage;
use crate::utils::{ColorWrapper, Pixel, sprite_management::load_sprite};


macro_rules! load_embedded_sprite {
    ($sprite_path:expr, $color:expr) => {
        {
            let sprite = include_bytes!($sprite_path);
            load_sprite(sprite, $color.get_ratatui_color()).expect("Failed to load sprite")
        }
    };
}


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
