use crate::shapes::pixel_image::{Pixel, PixelImage};
use ratatui::prelude::Color;
use serde::{Deserialize, Serialize};
use image::ImageReader;
use std::error::Error;


const DUCK_SPRITE_PATH: &str = "../assets/duck.png";


#[derive(Debug, Serialize, Deserialize)]
pub enum CreatureShapes {
    Duck,
}

impl PixelImage for CreatureShapes {
    fn pixels(&self) -> Vec<Pixel> {
        match self {
            CreatureShapes::Duck => load_sprite(DUCK_SPRITE_PATH, Color::Cyan).unwrap(),
        }
    }
}

fn load_sprite(path: &str, color: Color) -> std::io::Result<Vec<Pixel>> {
    let mut pixels = Vec::new();
    let img_data = load_image_data(DUCK_SPRITE_PATH).map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Sprite image not found")
    })?;

    for cords in img_data {
        pixels.push(Pixel {
            x: cords.0,
            y: cords.1,
            color,
        });
    }
    
    Ok(pixels)
}


/// Returns the coordinates of each black pixel in a vector.
fn load_image_data(path: &str) -> Result<Vec<(u32, u32)>, Box<dyn Error>> {
    let image = ImageReader::open(path)?.decode()?.to_luma8();
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