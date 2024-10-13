use crate::structs::Pixel;

pub trait PixelImage {
    /// Returns a vector with each pixel that should be colored for this pixel image.
    fn pixels(&self) -> Vec<Pixel>;
}
