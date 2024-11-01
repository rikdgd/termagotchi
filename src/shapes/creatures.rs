use crate::shapes::PixelImage;
use crate::utils::{ColorWrapper, Pixel, sprite_management::load_sprite};
use ratatui::widgets::canvas::{Painter, Shape};
use serde::{Deserialize, Serialize};
use rand::Rng;

const NUM_SHAPES: u32 = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreatureShapes {   // TODO: This enum should only contain the adult sprites
    Egg(ColorWrapper),
    Duck(ColorWrapper),
}

impl Shape for CreatureShapes {
    fn draw(&self, painter: &mut Painter) {
        for pixel in self.pixels() {
            painter.paint(pixel.x as usize, pixel.y as usize, pixel.color);
        }
    }
}

impl PixelImage for CreatureShapes {
    fn pixels(&self) -> Vec<Pixel> {
        match self {
            CreatureShapes::Egg(color) => {
                let egg_sprite = include_bytes!("../../assets/egg.png");
                load_sprite(egg_sprite, color.get_ratatui_color()).unwrap()
            },
            CreatureShapes::Duck(color) => {
                let duck_sprite = include_bytes!("../../assets/duck.png");
                load_sprite(duck_sprite, color.get_ratatui_color()).unwrap()
            },
        }
    }
}

impl CreatureShapes {
    pub fn new_random() -> Self {
        let color = ColorWrapper::new_random();
        let mut rng = rand::thread_rng();
        
        match rng.gen_range(0..NUM_SHAPES) {
            0 => CreatureShapes::Egg(color),
            1 => CreatureShapes::Duck(color),
            _ => CreatureShapes::Duck(color),
        }
    }
    
    pub fn get_color(&self) -> ColorWrapper {
        match self {
            CreatureShapes::Egg(color) => color.clone(),
            CreatureShapes::Duck(color) => color.clone(),
        }
    }
}


