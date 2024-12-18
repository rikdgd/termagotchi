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
    
    /// Get the dimensions of the shape as `(x, y)`. <br>
    /// Returns `(0, 0)` if the shape doesn't have any pixels.
    pub fn get_dimensions(&self) -> (u32, u32) {
        if self.0.is_empty() {
            return (0, 0);
        }
        
        let (mut min_x_pos, mut max_x_pos) = (None, None);
        let (mut min_y_pos, mut max_y_pos) = (None, None);
        
        for pixel in &self.0 {
            Self::check_min(&mut min_x_pos, pixel.x);
            Self::check_max(&mut max_x_pos, pixel.x);

            Self::check_min(&mut min_y_pos, pixel.y);
            Self::check_max(&mut max_y_pos, pixel.y);
        }
        
        match (min_x_pos, max_x_pos, min_y_pos, max_y_pos) {
            (Some(min_x), Some(max_x), Some(min_y), Some(max_y)) => {
                // + 1 to account for the fact that whenever we have any pixel, the width/height is automatically 1.
                // Typical off by 1 error :)
                (
                    max_x - min_x + 1,
                    max_y - min_y + 1,
                )
            },
            _ => (0, 0),
        }
    }

    /// Checks if the previously stored lowest x/y position is lower than the next position.
    /// If `previous_min` ends up being higher than `new_pos`, `previous_min`'s value is set to 
    /// that of `new_pos`.
    /// <br>
    /// ## parameters:
    /// * `previous_min` - The stored lowest x/y position.
    /// * `new_pos` - The new position to check the old one against.
    fn check_min(previous_min: &mut Option<u32>, new_pos: u32) {
        if let Some(pos) = previous_min {
            if new_pos < *pos {
                *previous_min = Some(new_pos);
            }
        } else {
            *previous_min = Some(new_pos)
        }
    }

    /// Checks if the previously stored highest x/y position is higher than the next position.
    /// If `previous_max` ends up being lower than `new_pos`, `previous_max`'s value is set to 
    /// that of `new_pos`.
    /// <br>
    /// ## parameters:
    /// * `previous_max` - The stored highest x/y position.
    /// * `new_pos` - The new position to check the old one against.
    fn check_max(previous_max: &mut Option<u32>, new_pos: u32) {
        if let Some(pos) = previous_max {
            if new_pos > *pos {
                *previous_max = Some(new_pos);
            }
        } else {
            *previous_max = Some(new_pos)
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

impl PixelImage for PixelVectorShape {
    fn pixels(&self) -> Vec<Pixel> {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use ratatui::prelude::Color;
    use crate::shapes::PixelVectorShape;
    use crate::utils::Pixel;

    #[test]
    fn get_dimensions() {
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

        let vertical_line_dimensions = (1, 3);
        let distributed_pixels_dimensions = (100, 65);
        let single_pixel_dimensions = (1, 1);
        let no_pixel_dimensions = (0, 0);


        let vertical_res = vertical_line.get_dimensions();
        let distributed_res = distributed_pixels.get_dimensions();
        let single_pixel_res = single_pixel.get_dimensions();
        let no_pixel_res = no_pixel_shape.get_dimensions();


        assert_eq!(vertical_res, vertical_line_dimensions);
        assert_eq!(distributed_res, distributed_pixels_dimensions);
        assert_eq!(single_pixel_res, single_pixel_dimensions);
        assert_eq!(no_pixel_res, no_pixel_dimensions);
    }
}