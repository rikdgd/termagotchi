use ratatui::widgets::canvas::{Painter, Shape};
use crate::movements::Location;
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
    
    /// Get the height of the shape. Returns 0 if the shape doesn't have any pixels.
    fn get_height(&self) -> u32 {
        if self.0.is_empty() {
            return 0;
        }
        
        let mut min_y_pos = None;
        let mut max_y_pos = None;
        
        for pixel in &self.0 {
            if let Some(min_pos) = min_y_pos {
                if pixel.y < min_pos {
                    min_y_pos = Some(pixel.y);
                }
            } else {
                min_y_pos = Some(pixel.y)
            }
            
            if let Some(max_pos) = max_y_pos {
                if pixel.y > max_pos {
                    max_y_pos = Some(pixel.y)
                }
            } else {
                max_y_pos = Some(pixel.y)
            }
        }
        
        match (min_y_pos, max_y_pos) {
            (Some(min_y), Some(max_y)) => {
                // + 1 to account for the fact that whenever we have any pixel, the height is automatically 1.
                // Typical of by 1 error.
                max_y - min_y + 1
            },
            _ => 0,
        }
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
    fn get_height() {
        let vertical_line = PixelVectorShape::new(vec![
            Pixel { x: 1, y: 2, color: Color::Black },
            Pixel { x: 1, y: 3, color: Color::Black },
            Pixel { x: 1, y: 4, color: Color::Black },
        ]);
        let distributed_pixels = PixelVectorShape::new(vec![
            Pixel { x: 1, y: 2, color: Color::Black },
            Pixel { x: 100, y: 66, color: Color::Black },
            Pixel { x: 12, y: 17, color: Color::Black },
        ]);
        let single_pixel = PixelVectorShape::new(vec![
            Pixel { x: 55, y: 34, color: Color::Black }
        ]);
        let no_pixel_shape = PixelVectorShape::new(vec![]);


        let vertical_res = vertical_line.get_height();
        let distributed_res = distributed_pixels.get_height();
        let single_pixel_res = single_pixel.get_height();
        let no_pixel_res = no_pixel_shape.get_height();


        assert_eq!(vertical_res, 3);
        assert_eq!(distributed_res, 65);
        assert_eq!(single_pixel_res, 1);
        assert_eq!(no_pixel_res, 0);
    }
}