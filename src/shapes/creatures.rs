use crate::shapes::pixel_image::{Pixel, PixelImage};
use ratatui::prelude::Color;
use serde::{Deserialize, Serialize};
use image::load_from_memory;
use std::error::Error;
use ratatui::widgets::canvas::{Painter, Shape};

#[derive(Debug, Serialize, Deserialize)]
pub enum CreatureShapes {
    Duck,
}

impl Shape for CreatureShapes {
    fn draw(&self, painter: &mut Painter) {
        for pixel in self.pixels() {
            painter.paint(
                pixel.x as usize,
                pixel.y as usize,
                pixel.color,
            );
        }
    }
}

impl PixelImage for CreatureShapes {
    fn pixels(&self) -> Vec<Pixel> {
        match self {
            CreatureShapes::Duck => {
                let duck_sprite = include_bytes!("../../assets/duck.png");
                load_sprite(duck_sprite, Color::Cyan).unwrap()
            },
        }
    }
}

fn load_sprite(image_bytes: &[u8], color: Color) -> std::io::Result<Vec<Pixel>> {
    let mut pixels = Vec::new();
    let black_pixel_cords = get_black_pixel_coordinates(image_bytes).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound, 
            "Sprite image not found"
        )
    })?;

    for cords in black_pixel_cords {
        pixels.push(Pixel {
            x: cords.0,
            y: cords.1,
            color,
        });
    }

    Ok(pixels)
}


/// Returns the coordinates of each black pixel in a vector.
fn get_black_pixel_coordinates(image_bytes: &[u8]) -> Result<Vec<(u32, u32)>, Box<dyn Error>> {
    let image = load_from_memory(image_bytes).unwrap().to_luma8();
    let dimensions = image.dimensions();
    let mut buffer: Vec<(u32, u32)> = Vec::new();

    for (pixel_index, pixel) in image.pixels().enumerate() {
        if pixel.0[0] == 0 {
            let cords = get_pixel_coordinates(pixel_index, dimensions);
            buffer.push(cords);
        }
    }
    
    Ok(buffer)
}

fn get_pixel_coordinates(pixel_index: usize, img_dimensions: (u32, u32)) -> (u32, u32) {
    let pixel_index = pixel_index as u32;
    let y_cord = pixel_index / img_dimensions.0;
    let x_cord = pixel_index % img_dimensions.0;

    (x_cord, y_cord)
}

#[cfg(test)]
mod test {
    use crate::shapes::creatures::get_pixel_coordinates;

    #[test]
    fn get_pixel_coordinates_test() {
        let dimensions_10x10: (u32, u32) = (10, 10);
        let dimensions_5x5: (u32, u32) = (5, 5);
        let dimensions_width_1: (u32, u32) = (1, 30);

        let result_a = get_pixel_coordinates(24, dimensions_10x10);
        let result_b = get_pixel_coordinates(6, dimensions_5x5);
        let result_c = get_pixel_coordinates(1, dimensions_5x5);
        let result_width_1 = get_pixel_coordinates(7, dimensions_width_1);

        assert_eq!(result_a, (4, 2));
        assert_eq!(result_b, (1, 1));
        assert_eq!(result_c, (1, 0));
        assert_eq!(result_width_1, (0, 7));
    }
}