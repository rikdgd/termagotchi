use ratatui::widgets::canvas::{Painter, Shape};
use crate::shapes::PixelImage;
use crate::utils::Pixel;

pub struct PixelVectorShape(Vec<Pixel>);

impl PixelVectorShape {
    pub fn new(vector: Vec<Pixel>) -> Self {
        Self(vector)
    }
    
    #[allow(unused)]
    pub fn from_pixel_image<T: PixelImage>(pixel_image: &T) -> Self {
        Self(pixel_image.pixels())
    }
}

impl Shape for PixelVectorShape {
    fn draw(&self, painter: &mut Painter) {
        for pixel in &self.0 {
            if let Some((x, y)) = painter.get_point(f64::from(pixel.x), f64::from(pixel.y)) {
                painter.paint(x, y, pixel.color);
            }
        }
    }
}
