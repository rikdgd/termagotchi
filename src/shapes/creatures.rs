use crate::shapes::PixelImage;
use crate::utils::{ColorWrapper, Pixel, sprite_management::load_sprite};
use serde::{Deserialize, Serialize};
use rand::Rng;
use crate::load_embedded_sprite;

const NUM_SHAPES: u32 = 7;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreatureShapes {
    Duck(ColorWrapper),
    Turtle(ColorWrapper),
    Spider(ColorWrapper),
    Snail(ColorWrapper),
    Fish(ColorWrapper),
    Mouse(ColorWrapper),
    Frog(ColorWrapper),
}


impl PixelImage for CreatureShapes {
    fn pixels(&self) -> Vec<Pixel> {
        match self {
            CreatureShapes::Duck(color) => load_embedded_sprite!("../../assets/duck.png", color),
            CreatureShapes::Turtle(color) => load_embedded_sprite!("../../assets/turtle.png", color),
            CreatureShapes::Spider(color) => load_embedded_sprite!("../../assets/spider.png", color),
            CreatureShapes::Snail(color) => load_embedded_sprite!("../../assets/snail.png", color),
            CreatureShapes::Fish(color) => load_embedded_sprite!("../../assets/fish.png", color),
            CreatureShapes::Mouse(color) => load_embedded_sprite!("../../assets/mouse.png", color),
            CreatureShapes::Frog(color) => load_embedded_sprite!("../../assets/frog.png", color),
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
            3 => CreatureShapes::Snail(color),
            4 => CreatureShapes::Fish(color),
            5 => CreatureShapes::Mouse(color),
            _ => CreatureShapes::Frog(color),
        }
    }
    
    pub fn get_color(&self) -> ColorWrapper {
        match self {
            CreatureShapes::Duck(color) => *color,
            CreatureShapes::Turtle(color) => *color,
            CreatureShapes::Spider(color) => *color,
            CreatureShapes::Snail(color) => *color,
            CreatureShapes::Fish(color) => *color,
            CreatureShapes::Mouse(color) => *color,
            CreatureShapes::Frog(color) => *color,
        }
    }
}
