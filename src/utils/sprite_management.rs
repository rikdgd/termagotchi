use std::error::Error;
use image::load_from_memory;
use image::imageops::flip_vertical;
use ratatui::prelude::Color;
use crate::utils::Pixel;

#[macro_export]
macro_rules! load_embedded_sprite {
    ($sprite_path:expr, $color:expr) => {
        {
            let sprite = include_bytes!($sprite_path);
            load_sprite(sprite, $color.get_ratatui_color()).expect("Failed to load sprite")
        }
    };
}


pub fn load_sprite(image_bytes: &[u8], color: Color) -> std::io::Result<Vec<Pixel>> {
    let mut pixels = Vec::new();
    let black_pixel_cords = get_black_pixel_coordinates(image_bytes)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "Sprite image not found"))?;

    for cords in black_pixel_cords {
        pixels.push(Pixel {
            x: cords.0,
            y: cords.1,
            color,
        });
    }

    Ok(pixels)
}

/// Returns the coordinates of each black pixel in a vector. This also flips the image vertically since
/// `ratatui`'s coordinate system goes from left to right, **BOTTOM** to **TOP**.
/// <br>
/// ## parameters:
/// * `image_bytes` - A slice of the bytes for the image that should get used. 
fn get_black_pixel_coordinates(image_bytes: &[u8]) -> Result<Vec<(u32, u32)>, Box<dyn Error>> {
    let image = load_from_memory(image_bytes).unwrap().to_luma8();
    let dimensions = image.dimensions();
    let mut buffer: Vec<(u32, u32)> = Vec::new();
    
    // flip the image over the x-axis, since ratatui coordinates start at the BOTTOM left, not TOP left.
    let flipped_image = flip_vertical(&image);
    for (pixel_index, pixel) in flipped_image.pixels().enumerate() {
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
    use super::get_pixel_coordinates;

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