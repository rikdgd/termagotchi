use crate::shapes::pixel_image::{Pixel, PixelImage};
use ratatui::prelude::Color;
use ratatui::widgets::canvas::{Painter, Shape};

pub struct DuckShape {
    color: Color,
}

impl DuckShape {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Shape for DuckShape {
    fn draw(&self, painter: &mut Painter) {
        for pixel in self.pixels() {
            painter.paint(
                pixel.x as usize, 
                pixel.y as usize, 
                self.color
            );
        }
    }
}

impl PixelImage for DuckShape {
    fn pixels(&self) -> Vec<Pixel> {
        todo!()
    }
}
