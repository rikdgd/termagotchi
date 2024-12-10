use ratatui::widgets::canvas::{Painter, Shape};
use crate::shapes::PixelImage;
use crate::utils::Pixel;

#[derive(Debug, Clone, PartialEq)]
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
    
    /// Mirrors the image over the x-axis, since the `ratatui::widgets::Canvas` goes from left to right,
    /// BOTTOM to TOP. This is not the default, and also not how sprite data is loaded. 
    /// <br><br>
    /// This method consumes `self` and returns a new `PixelVectorShape`.
    pub fn prepare_for_pixel_canvas(mut self) -> Self {
        for pixel in &mut self.0 {
            // Yes Rik, you need to do this yourself... 'image' crate does not transform images.
        }
        
        todo!()
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

#[cfg(test)]
mod tests {
    use ratatui::prelude::Color;
    use crate::shapes::PixelVectorShape;
    use crate::utils::Pixel;

    #[test]
    fn prepare_for_pixel_canvas_odd_height() {
        let pixel_shape = PixelVectorShape::new(vec![
            Pixel { x: 2, y: 3, color: Color::Black },
            Pixel { x: 3, y: 3, color: Color::Black },
            Pixel { x: 2, y: 2, color: Color::White },
            Pixel { x: 3, y: 2, color: Color::White },
            Pixel { x: 3, y: 1, color: Color::Cyan },
        ]);
        let expected_result = PixelVectorShape::new(vec![
            Pixel { x: 2, y: 1, color: Color::Black },
            Pixel { x: 3, y: 1, color: Color::Black },
            Pixel { x: 2, y: 2, color: Color::White },
            Pixel { x: 3, y: 2, color: Color::White },
            Pixel { x: 3, y: 3, color: Color::Cyan },
        ]);
        
        let result_shape = pixel_shape.prepare_for_pixel_canvas();
        
        assert_eq!(result_shape, expected_result);
    }
    
    #[test]
    fn prepare_for_pixel_canvas_even_height() {
        let pixel_shape = PixelVectorShape::new(vec![
            Pixel { x: 5, y: 5, color: Color::Red },
            Pixel { x: 5, y: 6, color: Color::Green },
            Pixel { x: 5, y: 7, color: Color::Blue },
            Pixel { x: 5, y: 8, color: Color::Cyan },
            Pixel { x: 5, y: 9, color: Color::Yellow },
            Pixel { x: 5, y: 10, color: Color::Magenta },
        ]);
        
        let expected_shape = PixelVectorShape::new(vec![
            Pixel { x: 5, y: 5, color: Color::Magenta },
            Pixel { x: 5, y: 6, color: Color::Yellow },
            Pixel { x: 5, y: 7, color: Color::Cyan },
            Pixel { x: 5, y: 8, color: Color::Blue },
            Pixel { x: 5, y: 9, color: Color::Green },
            Pixel { x: 5, y: 10, color: Color::Red },
        ]);
        
        let result_shape = pixel_shape.prepare_for_pixel_canvas();
        
        assert_eq!(result_shape, expected_shape);
    }
}