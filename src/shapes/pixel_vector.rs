use ratatui::widgets::canvas::{Painter, Shape};
use crate::shapes::PixelImage;
use crate::utils::Pixel;

#[derive(Debug, Clone, PartialEq)]
pub struct PixelVectorShape(Vec<Pixel>);

impl PixelVectorShape {
    pub fn new(vector: Vec<Pixel>) -> Self {
        Self(vector)
    }
    
    #[allow(unused)]
    pub fn from_pixel_image<T: PixelImage>(pixel_image: &T) -> Self {
        Self(pixel_image.pixels())
    }
    
    /// Moves the shape over the x and y-axis.
    /// ## parameters:
    /// * `move_x` - The amount of pixels to move the image to the right.
    /// * `move_y` - The amount of pixels to move the image downwards.
    pub fn move_shape(&mut self, move_x: u32, move_y: u32) {
        for pixel in &mut self.0 {
            pixel.x += move_x;
            pixel.y += move_y;
        }
    }
}

impl Shape for PixelVectorShape {
    fn draw(&self, painter: &mut Painter) {
        for pixel in &self.0 {
            painter.paint(
                pixel.x as usize, 
                pixel.y as usize, 
                pixel.color,
            );
        }
    }
}

impl PixelImage for PixelVectorShape {
    fn pixels(&self) -> Vec<Pixel> {
        self.0.clone()
    }
}
