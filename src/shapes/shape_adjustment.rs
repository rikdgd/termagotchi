use crate::utils::Pixel;
use super::PixelImage;

pub fn move_pixel_image<T: PixelImage>(image: &T, xy_move: (u32, u32)) -> Vec<Pixel> {
    let mut pixels = image.pixels();

    for pixel in &mut pixels {
        pixel.x += xy_move.0;
        pixel.y += xy_move.1;
    }

    pixels
}



#[cfg(test)]
mod test {
    use super::*;
    use ratatui::prelude::Color;
    use crate::shapes::PixelImage;
    use crate::utils::Pixel;

    struct BasicShape;
    impl PixelImage for BasicShape {
        fn pixels(&self) -> Vec<Pixel> {
            vec![
                Pixel { color: Color::White, x: 0, y: 0 },
                Pixel { color: Color::White, x: 1, y: 0 },
                Pixel { color: Color::White, x: 0, y: 1 },
                Pixel { color: Color::White, x: 1, y: 1 },
            ]
        }
    }

    #[test]
    fn move_pixel_image_test() {
        let pixel_image = BasicShape;
        let pixels = pixel_image.pixels();
        let movement = (10, 5);
        
        
        let expected_res = vec![
            Pixel { color: Color::White, x: 10, y: 5 },
            Pixel { color: Color::White, x: 11, y: 5 },
            Pixel { color: Color::White, x: 10, y: 6 },
            Pixel { color: Color::White, x: 11, y: 6 },
        ];
        
        let res = move_pixel_image(&pixel_image, movement);
        
        
        assert_eq!(res, expected_res);
    }
}
