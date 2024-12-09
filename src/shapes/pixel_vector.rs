use ratatui::widgets::canvas::{Painter, Shape};
use crate::shapes::PixelImage;
use crate::utils::Pixel;

pub struct PixelVectorShape(Vec<Pixel>);

impl PixelVectorShape {
    pub fn new(vector: Vec<Pixel>) -> Self {
        Self(vector)
    }
    
    pub fn from_pixel_image<T: PixelImage>(pixel_image: &T) -> Self {
        Self(pixel_image.pixels())
    }
    
    /// Translates the shape over the x and y-axis. If the suggested movement makes any of the
    /// pixel's coordinates lower than 0 (outside of screen), that coordinate will be set to 0.
    /// <br>
    /// ## parameters:
    /// * `x_move` - The amount to move the shape on the x-axis.
    /// * `y_move` - The amount to move the shape on the y-axis.
    pub fn translate(mut self, x_move: i32, y_move: i32) -> Self {
        for pixel in &mut self.0 {
            // Move all pixels, if their x or y location drops below 0, make it 0.
            pixel.x = (pixel.x as i32 + x_move).max(0) as u32;
            pixel.y = (pixel.y as i32 + y_move).max(0) as u32;
        }
        
        self
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
