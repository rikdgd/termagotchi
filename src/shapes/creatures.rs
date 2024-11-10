use crate::shapes::PixelImage;
use crate::utils::{ColorWrapper, Pixel, sprite_management::load_sprite};
use ratatui::widgets::canvas::{Painter, Shape};
use serde::{Deserialize, Serialize};
use rand::Rng;
use crate::load_embedded_sprite;

const NUM_SHAPES: u32 = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreatureShapes {
    Duck(ColorWrapper),
    Turtle(ColorWrapper),
    Spider(ColorWrapper),
}

impl Shape for CreatureShapes {
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

impl PixelImage for CreatureShapes {
    fn pixels(&self) -> Vec<Pixel> {
        match self {
            CreatureShapes::Duck(color) => load_embedded_sprite!("../../assets/duck.png", color),
            CreatureShapes::Turtle(color) => load_embedded_sprite!("../../assets/turtle.png", color),
            CreatureShapes::Spider(color) => load_embedded_sprite!("../../assets/spider.png", color),
        }
    }
}

impl CreatureShapes {
    pub fn new_random() -> Self {
        let color = ColorWrapper::new_random();
        let mut rng = rand::thread_rng();
        
        match rng.gen_range(0..NUM_SHAPES) {
            0 => CreatureShapes::Duck(color),
            1 => CreatureShapes::Turtle(color),
            2 => CreatureShapes::Spider(color),
            _ => CreatureShapes::Duck(color),
        }
    }
    
    pub fn get_color(&self) -> ColorWrapper {
        match self {
            CreatureShapes::Duck(color) => color.clone(),
            CreatureShapes::Turtle(color) => color.clone(),
            CreatureShapes::Spider(color) => color.clone()
        }
    }
}
