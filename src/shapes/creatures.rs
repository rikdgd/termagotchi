use crate::shapes::pixel_image::{Pixel, PixelImage};
use ratatui::prelude::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CreatureShapes {
    Duck,
}

impl PixelImage for CreatureShapes {
    fn pixels(&self) -> Vec<Pixel> {
        match self {
            CreatureShapes::Duck => {
                vec![
                    Pixel { x: 2, y: 2, color: Color::Cyan},
                    Pixel { x: 2, y: 3, color: Color::Cyan},
                    Pixel { x: 2, y: 4, color: Color::Cyan}
                ]
            }
        }
    }
}
